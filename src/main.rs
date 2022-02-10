use rodio::Sink;
use rodio::{Decoder, OutputStream};
use std::io::BufReader;
use std::{error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    loop {
        println!("Starting playback");
        let file = BufReader::new(File::open("resources/eurostar_car.wav").unwrap());
        let source = Decoder::new(file).unwrap();

        let sink = Sink::try_new(&stream_handle).unwrap();

        sink.append(source);

        sink.sleep_until_end();
    }
}
