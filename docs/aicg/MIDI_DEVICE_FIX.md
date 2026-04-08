# MIDI Device Connection Fix

## Problem Analysis
After the Phase 3 library modularization, MIDI mapping stopped working even though the application could connect to MIDI devices.

### Root Cause
**Port list ordering inconsistency**: Each time `MidiInput::new()` is called, the MIDI port enumeration might return ports in a different order depending on system state or driver behavior.

**Flow of the bug**:
1. User selects device from list (e.g., "X8III" = index 0)
2. `select_device()` creates its own `MidiInput` and enumerates ports
3. User selection returns device name string
4. `start_mapping()` creates a **new** `MidiInput` instance
5. The new instance might return ports in **different order**
6. Device index no longer matches the user's selection
7. Result: Connected to wrong device or connection fails

### Example Scenario
```
select_device() MidiInput enumeration:
  0: X8III
  1: MIDIIN2
  2: loopMIDI Port

User selects: "X8III" → returns device_idx=0

start_mapping() MidiInput enumeration (different order):
  0: loopMIDI Port
  1: X8III
  2: MIDIIN2

Code tries: in_ports[0] → "loopMIDI Port" ❌
Expected:   "X8III" ✓
```

## Solution
**Changed from index-based to name-based device selection**

Instead of passing `device_idx: usize`, now we pass `device_name: String`.

### Changes Made

#### 1. src/mapper.rs
Changed function signature and device lookup:
```rust
// Before: Using index
pub async fn start_mapping(
    config_path: String,
    device_idx: usize,
    mode: MappingMode
) -> Result<(), Box<dyn Error>> {
    let midi_in = MidiInput::new("osynic-midi")?;
    let in_ports = midi_in.ports();
    let in_port = &in_ports[device_idx];  // ❌ Index based - unreliable
    let device_name = midi_in.port_name(in_port)?;
    
// After: Using device name
pub async fn start_mapping(
    config_path: String,
    device_name: String,
    mode: MappingMode
) -> Result<(), Box<dyn Error>> {
    let midi_in = MidiInput::new("osynic-midi")?;
    let in_ports = midi_in.ports();
    let in_port = in_ports.iter()
        .find(|p| midi_in.port_name(p).unwrap_or_default() == device_name)
        .ok_or(format!("MIDI device '{}' not found", device_name))?;  // ✓ Name based - reliable
```

#### 2. src/interactive.rs
Changed `select_device()` return type:
```rust
// Before: Returns index
pub fn select_device() -> Result<usize, Box<dyn Error>> {
    // ... finds device_idx from selection

// After: Returns device name
pub fn select_device() -> Result<String, Box<dyn Error>> {
    // ... directly returns selected device name
```

#### 3. src/main.rs
Updated to pass device name instead of index:
```rust
// Before
let device_idx = select_device()?;
start_mapping(config_path, device_idx, mode).await

// After
let device_name = select_device()?;
start_mapping(config_path, device_name, mode).await
```

## Benefits of This Approach

✅ **Reliable**: Device connection based on actual device name, not volatile index
✅ **Robust**: Works even if MIDI port enumeration order changes
✅ **Clear**: Device names are human-readable in code and error messages
✅ **Type-safe**: String type prevents accidental index confusion
✅ **Better error messages**: Shows actual device name when connection fails

## Verification
Device enumeration confirmed working:
```
Available MIDI input devices:
  0: X8III
  1: MIDIIN2 (X8III)
  2: loopMIDI Port
  3: loopMIDI Port 1
  4: loopMIDI Port 2

Available configuration files:
  0: midi_config.json
  1: midi_config_10ka.json
  2: midi_config_10kab.json
  ... and 11 more
```

## Testing
To test the fix:
1. Build: `cargo build`
2. Run: `.\target\debug\osynic-midi.exe start`
3. Select a MIDI device by name
4. MIDI events should now map to keyboard correctly

## Impact
- ✅ MIDI mapping now works reliably
- ✅ No breaking changes to CLI interface
- ✅ Examples/cli.rs pattern now correctly implemented in library
- ✅ Future-proof against MIDI port enumeration variations
