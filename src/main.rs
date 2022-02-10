use app::{App, AppReturn};
use inputs::InputEvent;
use inputs::events::Events;
use rodio::Sink;
use rodio::{Decoder, OutputStream};
use std::cell::RefCell;
use std::io::BufReader;
use std::rc::Rc;
use std::time::Duration;
use std::{error::Error, fs::File};

use std::io::stdout;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::ui;
use eyre::Result;

mod app;
mod inputs;

fn main() -> Result<()> {
    let app = Rc::new(RefCell::new(App::new())); // TODO app is useless for now
    start_ui(app)?;
    Ok(())
}

pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    // Configure Crossterm backend for tui
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // User event handler
    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    loop {
        let mut app = app.borrow_mut();

        // Render
        terminal.draw(|rect| ui::draw(rect, &app))?;

        // Handle inputs
        let result = match events.next()? {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_on_tick(),
        };
        // Check if we should exit
        if result == AppReturn::Exit {
            break;
        }
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
