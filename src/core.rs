use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MappingMode {
    Octaves,
    Notes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mapping_mode: Option<MappingMode>,
    pub octaves: HashMap<String, HashMap<String, String>>,
    pub velocity_threshold: u8,
    pub note_mappings: HashMap<u8, String>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let config_str = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    pub fn note_to_pitch(note: u8) -> (u8, String) {
        let octave = note / 12 - 1;
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

    pub fn get_key_for_note(&self, note: u8, mode: &MappingMode) -> Option<Key> {
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
            "A" => Some(Key::A),
            "B" => Some(Key::B),
            "C" => Some(Key::C),
            "D" => Some(Key::D),
            "E" => Some(Key::E),
            "F" => Some(Key::F),
            "G" => Some(Key::G),
            "H" => Some(Key::H),
            "I" => Some(Key::I),
            "J" => Some(Key::J),
            "K" => Some(Key::K),
            "L" => Some(Key::L),
            "M" => Some(Key::M),
            "N" => Some(Key::N),
            "O" => Some(Key::O),
            "P" => Some(Key::P),
            "Q" => Some(Key::Q),
            "R" => Some(Key::R),
            "S" => Some(Key::S),
            "T" => Some(Key::T),
            "U" => Some(Key::U),
            "V" => Some(Key::V),
            "W" => Some(Key::W),
            "X" => Some(Key::X),
            "Y" => Some(Key::Y),
            "Z" => Some(Key::Z),
            "RAlt" => Some(Key::Alt),
            _ => None,
        })
    }
}

#[derive(Debug)]
pub enum KeyEvent {
    NoteOn(u8, u8), // (note, velocity)
    NoteOff(u8),    // note
}

pub struct KeyboardMapper {
    pub config: Config,
    pub enigo: Arc<Mutex<Enigo>>,
    pub mode: MappingMode,
}

impl KeyboardMapper {
    pub fn new(config: Config, enigo: Arc<Mutex<Enigo>>, mode: MappingMode) -> Self {
        Self {
            config,
            enigo,
            mode,
        }
    }

    pub fn handle_event(&self, event: KeyEvent) -> Result<(), Box<dyn Error>> {
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
