use rodio::{source::SineWave, OutputStream, Sink, Source};
use std::time::Duration;

pub fn play_beep() {
    // Default system beep
    print!("\x07");

    // Attempt to initialize Rodio output stream
    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        let sink = Sink::try_new(&stream_handle).unwrap();

        let source = SineWave::new(300.0)
            .take_duration(Duration::from_millis(150)) // Play for 150ms
            .amplify(0.3); // Reduce volume

        sink.append(source);
        sink.sleep_until_end(); // Ensure sound finishes
    }
}
