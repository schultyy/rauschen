use app::App;
use rodio::Sink;
use rodio::{Decoder, OutputStream};
use std::cell::RefCell;
use std::io::BufReader;
use std::rc::Rc;
use std::{error::Error, fs::File};

use std::io::stdout;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::ui;
use eyre::Result;

mod app;

fn main() -> Result<()> {
    let app = Rc::new(RefCell::new(App::new())); // TODO app is useless for now
    start_ui(app)?;
    Ok(())
}


#[allow(unreachable_code)]
pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    // Configure Crossterm backend for tui
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        let app = app.borrow();

        // Render
        terminal.draw(|rect| ui::draw(rect, &app))?;

        // TODO handle inputs here
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

fn playback() -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    loop {
        println!("Starting playback");
        let file = BufReader::new(File::open("resources/eurostar-car.mp3").unwrap());
        let source = Decoder::new(file).unwrap();

        let sink = Sink::try_new(&stream_handle).unwrap();

        sink.append(source);

        sink.set_volume(2.0);

        sink.sleep_until_end();
    }
}
