use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

use rodio::{Decoder, OutputStream, Source};

#[derive(PartialEq, Eq)]
pub enum SoundThreadSignal {
    Sound,
    Close,
}

pub struct SoundThread {
    send: Sender<SoundThreadSignal>,
    _handle: JoinHandle<()>,
}

impl SoundThread {
    pub fn new() -> SoundThread {
        let (send, recv) = mpsc::channel();
        SoundThread {
            send,
            _handle: thread::spawn(|| SoundThread::run_loop(recv)),
        }
    }

    fn run_loop(recv: Receiver<SoundThreadSignal>) {
        loop {
            match recv.try_recv() {
                Err(TryRecvError::Disconnected) => break,
                Ok(SoundThreadSignal::Close) => break,
                Err(TryRecvError::Empty) => continue,
                Ok(SoundThreadSignal::Sound) => SoundThread::play_beep_sound(),
            }
        }
    }

    fn play_beep_sound() {
        for _ in 0..3 {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let file = BufReader::new(File::open("src/assets/beep-07a.wav").unwrap());
            let source = Decoder::new(file).unwrap();

            stream_handle.play_raw(source.convert_samples()).unwrap();
            sleep(Duration::from_millis(300));
        }
    }

    pub fn play_sound(&mut self) {
        self.send.send(SoundThreadSignal::Sound).unwrap();
    }
}

impl Drop for SoundThread {
    fn drop(&mut self) {
        self.send.send(SoundThreadSignal::Close).unwrap();
        // self._handle.join().unwrap();
    }
}
