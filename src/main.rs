use app::{App, AppReturn};
use inputs::events::Events;
use inputs::InputEvent;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::filter::threshold::ThresholdFilter;
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;
use std::time::Duration;

use std::io::{self, stdout};

use log::{error, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::ui;
use eyre::Result;

mod app;
mod home;
mod inputs;
mod playback;

fn prepare() -> Result<()> {
    let file_url =
        "https://github.com/schultyy/rauschen/blob/add-tui/resources/eurostar-car.ogg?raw=true";
    let app_dir = home::app_dir();
    let local_filename = app_dir.join("eurostar-car.ogg");

    home::create_home_dir_if_not_exist()?;

    if local_filename.exists() {
        return Ok(());
    }

    let mut response = reqwest::blocking::get(file_url)?;
    let mut out = File::create(app_dir.join(local_filename)).expect("failed to create file");
    io::copy(&mut response, &mut out).expect("failed to copy content");

    Ok(())
}

fn init_logger() -> Result<()> {
    let window_size = 3; // log0, log1, log2
    let fixed_window_roller = FixedWindowRoller::builder()
        .build("log{}", window_size)
        .unwrap();

    let size_limit = 5 * 1024; // 5KB as max log file size to roll
    let size_trigger = SizeTrigger::new(size_limit);
    let compound_policy =
        CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

    let logfile_path = home::app_dir().join("output.log");

    let config = Config::builder()
    .appender(
        Appender::builder()
            .filter(Box::new(ThresholdFilter::new(LevelFilter::Debug)))
            .build(
                "logfile",
                Box::new(
                    RollingFileAppender::builder()
                        .encoder(Box::new(PatternEncoder::new("{d} {l} {t}::{m}{n}")))
                        .build(logfile_path, Box::new(compound_policy))?,
                ),
            ),
    )
    .build(
        Root::builder()
            .appender("logfile")
            .build(LevelFilter::Debug),
    )?;

    log4rs::init_config(config)?;
    Ok(())
}

fn main() -> Result<()> {
    init_logger()?;
    if let Err(err) = prepare() {
        error!("{}", err.to_string());
        return Err(err);
    }
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
