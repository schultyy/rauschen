use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, Sparkline, Gauge};
use tui::Frame;

use super::actions::Actions;
use super::state::AppState;
use crate::app::App;

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(size);

    let gauge = Gauge::default()
        .block(Block::default().title("Volume").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(app.state().volume().unwrap());
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

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}
