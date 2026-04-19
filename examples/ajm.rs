use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use midir::MidiInput;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::io::{Write, stdin, stdout};
use std::sync::Arc;
use std::sync::Mutex;
use std::{env, fs};
use tokio::sync::mpsc;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
enum MappingMode {
    Octaves,
    Notes,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    mapping_mode: Option<MappingMode>, // 如果配置文件未指定，则使用命令行参数
    octaves: HashMap<String, HashMap<String, String>>,
    velocity_threshold: u8,
    note_mappings: HashMap<u8, String>,
}

impl Config {
    fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let config_str = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    // 将音符转换为八度和音名
    fn note_to_pitch(note: u8) -> (u8, String) {
        let octave = (note / 12) - 1;
        let pitch = match note % 12 {
            0 => "C",
            1 => "C#/Db",
            2 => "D",
            3 => "D#/Eb",
            4 => "E",
            5 => "F",
            6 => "F#/Gb",
            7 => "G",
            8 => "G#/Ab",
            9 => "A",
            10 => "A#/Bb",
            11 => "B",
            _ => unreachable!(),
        };
        (octave, pitch.to_string())
    }

    fn get_key_for_note(&self, note: u8, mode: &MappingMode) -> Option<Key> {
        let key_str = match mode {
            MappingMode::Notes => self.note_mappings.get(&note).cloned(),
            MappingMode::Octaves => {
                let (octave, pitch) = Self::note_to_pitch(note);
                self.octaves
                    .get(&octave.to_string())
                    .and_then(|octave_map| octave_map.get(&pitch).cloned())
            }
        };

        key_str.and_then(|key_str| match key_str.as_str() {
            "," => Some(Key::Unicode(',')),
            "." => Some(Key::Unicode('.')),
            "/" => Some(Key::Unicode('/')),
            ";" => Some(Key::Unicode(';')),
            "'" => Some(Key::Unicode('\'')),
            "[" => Some(Key::Unicode('[')),
            "]" => Some(Key::Unicode(']')),
            "\\" => Some(Key::Unicode('\\')),
            "-" => Some(Key::Unicode('-')),
            "=" => Some(Key::Unicode('=')),
            "Space" => Some(Key::Space),
            "Left" => Some(Key::LeftArrow),
            "Right" => Some(Key::RightArrow),
            "A" => Some(Key::Unicode('A')),
            "B" => Some(Key::Unicode('B')),
            "C" => Some(Key::Unicode('C')),
            "D" => Some(Key::Unicode('D')),
            "E" => Some(Key::Unicode('E')),
            "F" => Some(Key::Unicode('F')),
            "G" => Some(Key::Unicode('G')),
            "H" => Some(Key::Unicode('H')),
            "I" => Some(Key::Unicode('I')),
            "J" => Some(Key::Unicode('J')),
            "K" => Some(Key::Unicode('K')),
            "L" => Some(Key::Unicode('L')),
            "M" => Some(Key::Unicode('M')),
            "N" => Some(Key::Unicode('N')),
            "O" => Some(Key::Unicode('O')),
            "P" => Some(Key::Unicode('P')),
            "Q" => Some(Key::Unicode('Q')),
            "R" => Some(Key::Unicode('R')),
            "S" => Some(Key::Unicode('S')),
            "T" => Some(Key::Unicode('T')),
            "U" => Some(Key::Unicode('U')),
            "V" => Some(Key::Unicode('V')),
            "W" => Some(Key::Unicode('W')),
            "X" => Some(Key::Unicode('X')),
            "Y" => Some(Key::Unicode('Y')),
            "Z" => Some(Key::Unicode('Z')),
            "RAlt" => Some(Key::Alt),
            _ => None,
        })
    }
}

#[derive(Debug)]
enum KeyEvent {
    NoteOn(u8, u8), // (note, velocity)
    NoteOff(u8),    // note
}

struct KeyboardMapper {
    config: Config,
    enigo: Arc<Mutex<Enigo>>,
    mode: MappingMode,
}

impl KeyboardMapper {
    fn new(config: Config, enigo: Arc<Mutex<Enigo>>, mode: MappingMode) -> Self {
        Self {
            config,
            enigo,
            mode,
        }
    }

    fn handle_event(&self, event: KeyEvent) -> Result<(), Box<dyn Error>> {
        if let Ok(mut enigo_guard) = self.enigo.lock() {
            match event {
                KeyEvent::NoteOn(note, velocity) => {
                    if velocity >= self.config.velocity_threshold {
                        if let Some(key) = self.config.get_key_for_note(note, &self.mode) {
                            enigo_guard.key(key, Press)?;
                        }
                    }
                }
                KeyEvent::NoteOff(note) => {
                    if let Some(key) = self.config.get_key_for_note(note, &self.mode) {
                        enigo_guard.key(key, Release)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));

    // 加载配置文件
    let mut config = Config::load("configs/midi_config.json")?;

    // 确定映射模式：优先使用命令行参数，其次使用配置文件，最后默认为Notes模式
    let mode = env::args()
        .nth(1)
        .and_then(|arg| match arg.to_lowercase().as_str() {
            "octaves" => Some(MappingMode::Octaves),
            "notes" => Some(MappingMode::Notes),
            _ => None,
        })
        .or(config.mapping_mode.take())
        .unwrap_or(MappingMode::Notes);

    println!("Using mapping mode: {:?}", mode);

    let mapper = Arc::new(KeyboardMapper::new(config, Arc::clone(&enigo), mode));

    // 创建channel用于传递MIDI事件
    let (tx, mut rx) = mpsc::channel::<KeyEvent>(32);

    // 创建MIDI输入连接
    let midi_in = MidiInput::new("midi-key-mapper")?;
    let in_ports = midi_in.ports();

    // 显示可用的MIDI输入端口
    println!("\nAvailable input ports:");
    for (i, p) in in_ports.iter().enumerate() {
        println!("{}: {}", i, midi_in.port_name(p)?);
    }

    // 选择MIDI输入端口
    print!("Please select input port: ");
    stdout().flush()?;
    stdin().read_line(&mut input)?;
    let in_port = in_ports
        .get(input.trim().parse::<usize>()?)
        .ok_or("Invalid port number")?;

    println!("\nOpening connection");

    let tx_clone = tx.clone();

    let _conn_in = midi_in.connect(
        in_port,
        "midi-key-mapper",
        move |_stamp, message, _| {
            if message.len() == 3 {
                let status = message[0];
                let note = message[1];
                let velocity = message[2];

                let event = if status == 0x90 && velocity > 0 {
                    Some(KeyEvent::NoteOn(note, velocity))
                } else if status == 0x80 || (status == 0x90 && velocity == 0) {
                    Some(KeyEvent::NoteOff(note))
                } else {
                    None
                };

                if let Some(event) = event {
                    let _ = tx_clone.try_send(event);
                }
            }
        },
        (),
    )?;

    // 创建事件处理任务
    let event_handler = tokio::spawn({
        let mapper = Arc::clone(&mapper);
        async move {
            while let Some(event) = rx.recv().await {
                if let Err(e) = mapper.handle_event(event) {
                    eprintln!("Error handling event: {}", e);
                }
            }
        }
    });

    println!("Connection open. Press Enter to exit.");
    input.clear();
    stdin().read_line(&mut input)?;

    // 清理和关闭
    drop(tx);
    event_handler.abort();

    Ok(())
}

// 我应该如何将这个脚本模块化并开发成一个功能完善的MIDI-键盘映射器库和TUI应用程序？1.需要拆分成哪些个模块？2.如何先列出configs文件夹里面符合格式的json来选择配置？3.如何适配非命令行的情况（例如grpc调用，如何提供接口来选择配置和启停映射？）？4.如何MIDI设备断线重连等等？请指教！
