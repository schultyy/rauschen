use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Sparkline, Gauge};
use tui::Frame;

use crate::app::App;

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(size);


    let volume_level = app.state().volume().unwrap() * 100.00;

    let gauge = Gauge::default()
        .block(Block::default().title("Volume").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(volume_level as u16);
    rect.render_widget(gauge, chunks[0]);

    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("Playing")
                .borders(Borders::LEFT | Borders::RIGHT),
        )
        .data(&app.state().sparkline_data().unwrap())
        .style(Style::default().fg(Color::Yellow));
    rect.render_widget(sparkline, chunks[1]);
}
