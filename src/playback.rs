use std::{
    fs::File,
    io::BufReader,
    sync::atomic::{AtomicU16, Ordering},
    thread,
    time::Duration,
};

use rodio::{Decoder, OutputStream, Sink, Source};

static CMD: AtomicU16 = AtomicU16::new(1);

pub fn start_playback() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        loop {
            if sink.empty() {
                let file = BufReader::new(File::open("resources/eurostar-car.ogg").unwrap());
                let source = Decoder::new(file).unwrap();
                sink.append(source);
            }

            match CMD.load(Ordering::Relaxed) {
                0 => {}
                1 => {
                    CMD.store(0, Ordering::SeqCst);
                    sink.play();
                }
                // -1 => {
                // 	CMD.store(0, Ordering::SeqCst);
                // 	match sink.is_paused() {
                // 		true => sink.play(),
                // 		false => sink.pause(),
                // 	}
                // }
                50 => {
                    CMD.store(0, Ordering::SeqCst);
                    sink.set_volume(0.5);
                }
                100 => {
                    CMD.store(0, Ordering::SeqCst);
                    sink.set_volume(1.0);
                }
                _ => {},
            }
            thread::sleep(Duration::from_millis(250));
        }
    });
}

pub fn set_cmd(cmd: u16) {
    CMD.store(cmd, Ordering::SeqCst);
}

pub fn play() {
    CMD.store(1, Ordering::SeqCst);
}
