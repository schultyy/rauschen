use std::{
    fs::File,
    io::BufReader,
    sync::{mpsc::{self, Sender}},
    thread,
};

use log::{error, debug};
use rodio::{Decoder, OutputStream, Sink, Source};

use crate::home;

pub enum PlaybackControl {
    VolumeUp(f32),
    VolumeDown(f32),
    Play
}

pub fn start_playback() -> Sender<PlaybackControl> {
    let (tx, rx) = mpsc::channel();

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
                debug!("Sink is empty");
                let file = File::open(filename.clone()).map_err(|err| {
                    error!("{}", err.to_string());
                    panic!("{}", err)
                })
                .and_then(|file| Ok(BufReader::new(file)))
                .unwrap();

                let source = Decoder::new(file)
                .and_then(|source| Ok(source.repeat_infinite()))
                .and_then(|source| Ok(sink.append(source)));
                if let Err(error) = source {
                    error!("{}", error.to_string());
                    panic!("{}", error)
                }
            }

            match rx.recv().unwrap() {
                PlaybackControl::VolumeUp(new_volume) | PlaybackControl::VolumeDown(new_volume) => {
                    sink.set_volume(new_volume);
                },
                PlaybackControl::Play => {
                    sink.play();
                },
            }
        }
    });

    tx
}