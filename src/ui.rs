use crate::app::App;
#[allow(unused_imports)]
use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    frame.render_widget(
        Paragraph::new(format!(
            "Hello world!\n
                `char_ticks_left` = {}\n
                `rep_char_cooldown` = {}\n
                `last_char` = {:?}",
            app.char_ticks_left, app.rep_char_cooldown, app.last_char
        ))
        .block(
            Block::default()
                .title("Hello world!")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        frame.size(),
    )
}
