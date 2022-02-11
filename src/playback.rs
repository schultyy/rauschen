use std::{
    fs::File,
    io::BufReader,
    sync::atomic::{AtomicU16, Ordering},
    thread,
    time::Duration,
};

use log::error;
use rodio::{Decoder, OutputStream, Sink};

use crate::home;

static CMD: AtomicU16 = AtomicU16::new(1);

pub fn start_playback() {
    thread::spawn(move || {
        let filename = home::app_dir().join("eurostar-car.ogg");
        let (_stream, stream_handle) = OutputStream::try_default().map_err(|err| {
            error!("{}", err.to_string());
            panic!("{}", err)
        })
        .unwrap();
        let sink = Sink::try_new(&stream_handle).map_err(|err| {
            error!("{}", err.to_string());
            panic!("{}", err)
        })
        .unwrap();

        loop {
            if sink.empty() {
                let file = File::open(filename.clone()).map_err(|err| {
                    error!("{}", err.to_string());
                    panic!("{}", err)
                })
                .and_then(|file| Ok(BufReader::new(file)))
                .unwrap();

                let source = Decoder::new(file)
                .and_then(|source| Ok(sink.append(source)));
                if let Err(error) = source {
                    error!("{}", error.to_string());
                    panic!("{}", error)
                }
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
