use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

/// Plays a beep sound using `beep.wav`
pub fn play_beep() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink"); // ✅ FIXED

    let file = File::open("sound.wav").expect("Failed to open sound.wav");
    let source = Decoder::new(BufReader::new(file)).expect("Failed to decode WAV file");

    sink.append(source);
    sink.sleep_until_end(); // Ensures the sound plays fully before proceeding
    thread::sleep(Duration::from_millis(18));
}
