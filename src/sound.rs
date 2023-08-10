use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

use rodio::{Decoder, OutputStream, Source};

pub fn play_beep_sound() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("src/assets/beep-07a.wav").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).unwrap();
    sleep(Duration::from_secs(5));
}
