use rodio::{source::SineWave, OutputStream, Sink, Source}; 
use std::time::Duration;
use std::thread;

pub fn play_beep() {
    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        let sink = Sink::try_new(&stream_handle).unwrap();
        let source = SineWave::new(1000.0) // 1000 Hz tone
            .take_duration(Duration::from_millis(200)) 
            .amplify(0.2); // Reduce volume

        sink.append(source);
        thread::sleep(Duration::from_millis(50)); // Ensure sound finishes
    } else {
        println!("⚠ Failed to initialize audio. Falling back to system beep.");
        print!("\x07");
    }
}

