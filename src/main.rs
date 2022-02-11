use app::{App, AppReturn};
use inputs::InputEvent;
use inputs::events::Events;
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;
use std::time::Duration;

use std::io::{stdout, self};

use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::ui;
use eyre::Result;

mod app;
mod inputs;
mod playback;
mod home;


fn prepare() -> Result<()> {
    let file_url = "https://github.com/schultyy/rauschen/blob/add-tui/resources/eurostar-car.ogg?raw=true";
    let app_dir = home::app_dir();
    let local_filename = app_dir.join("eurostar-car.ogg");

    home::create_home_dir_if_not_exist()?;

    if local_filename.exists() {
        return Ok(())
    }

    let mut response = reqwest::blocking::get(file_url)?;
    let mut out = File::create(app_dir.join(local_filename)).expect("failed to create file");
    io::copy(&mut response, &mut out).expect("failed to copy content");

    Ok(())
}


fn main() -> Result<()> {
    prepare()?;
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
    playback::start_playback();
    playback::set_cmd(app.borrow().state().volume().unwrap());

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
