use std::{fs::File, io::BufReader, thread, time::Duration};

use rodio::{Decoder, OutputStream, Sink, Source};

pub fn start_playback() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let volume = 1.0;

        let file = BufReader::new(File::open("resources/eurostar-car.mp3").unwrap());
        let source = Decoder::new(file)
            .unwrap()
            .take_duration(Duration::from_secs(76))
            .repeat_infinite();

        sink.append(source);

        sink.set_volume(volume);

        sink.sleep_until_end();
    });
}
