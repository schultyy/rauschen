use rodio::{source::Source, Decoder, OutputStream};
use std::io::BufReader;
use std::{error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    loop {
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open("resources/eurostar_car.wav").unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        // Play the sound directly on the device
        stream_handle.play_raw(source.convert_samples())?;

        //76sec sleep / file is 1:16 long
        std::thread::sleep(std::time::Duration::from_secs(76));
    }
}
