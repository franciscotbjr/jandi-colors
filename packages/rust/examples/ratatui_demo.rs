use jandi_colors::PALETTE;
use ratatui::{
    Terminal,
    backend::TestBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Color,
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                PALETTE
                    .iter()
                    .map(|_| Constraint::Length(2))
                    .collect::<Vec<_>>(),
            )
            .split(f.area());

        for (i, color) in PALETTE.iter().enumerate() {
            let crossterm_color: Color = (*color).into();
            let block = Block::default()
                .borders(Borders::ALL)
                .style(ratatui::style::Style::default().bg(crossterm_color));
            let text = Span::styled(
                format!(
                    " {}  {}  ({}, {}, {})",
                    color.hex, color.name, color.rgb.r, color.rgb.g, color.rgb.b
                ),
                ratatui::style::Style::default().fg(if color.hsl.l > 50 {
                    Color::Black
                } else {
                    Color::White
                }),
            );
            let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Left);
            f.render_widget(paragraph, chunks[i]);
        }
    })?;
    Ok(())
}
