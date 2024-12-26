use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::utils::formatters::format_error_message;

pub fn render_error(f: &mut Frame, area: Rect, error: &str, title: Option<&str>) {
    let block_title = title.unwrap_or("Error");

    let max_width = area.width.saturating_sub(10) as usize;
    let max_lines = area.height.saturating_sub(3) as usize;

    let formatted_error = format_error_message(error, max_width, max_lines);

    let error_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .title(Span::styled(
            format!(" {block_title} "),
            Style::default().add_modifier(Modifier::BOLD),
        ));

    let inner_area = error_block.inner(area);

    f.render_widget(error_block, area);

    let error_lines: Vec<Line> = formatted_error
        .lines()
        .map(|line| {
            Line::from(vec![
                if line == formatted_error.lines().next().unwrap() {
                    Span::styled(
                        "Error: ",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw("       ")
                },
                Span::raw(line),
            ])
        })
        .collect();

    let error_text = Paragraph::new(Text::from(error_lines))
        .style(Style::default())
        .alignment(Alignment::Left);

    f.render_widget(error_text, inner_area);
}
