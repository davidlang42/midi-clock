use std::io::Write;
use std::{env, fs};
use std::thread;
use std::time::Duration;
use wmidi::MidiMessage;

const TICKS_PER_BEAT: u64 = 24;

fn main() {
    let mut args = env::args().skip(1);
    let bpm: u64 = args.next().expect("The first argument should be the bpm.")
        .parse().expect("The bpm should be a positive integer");
    if bpm == 0 {
        println!("The bpm cannot be zero");
        return;
    }
    if let Some(file) = args.next() {
        // write to file
        let midi_out = fs::File::options().write(true).open(&file).expect(&format!("Cannot open MIDI OUT '{}'", file));
        send_midi_clock(midi_out, bpm);
    } else {
        // write to std out
        let std_out = std::io::stdout();
        send_midi_clock(std_out, bpm);
    }
}

fn send_midi_clock<T: Write>(mut f: T, bpm: u64) {
    let wait_ms: u64 = 60000 / TICKS_PER_BEAT / bpm;
    let message = MidiMessage::TimingClock;
    let mut buf = Vec::new();
    let expected = message.bytes_size();
    buf.resize(expected, 0);
    if message.copy_to_slice(&mut buf).is_err() {
        panic!("Error writing MIDI message");
    }
    loop {
        if f.write_all(&buf).is_err() {
            panic!("Error writing to device.")
        }
        if f.flush().is_err() {
            panic!("Error flushing to device.");
        }
        thread::sleep(Duration::from_millis(wait_ms));
    }
}
