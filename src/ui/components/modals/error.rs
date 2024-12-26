use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};

use super::{render_modal_frame, ModalConfig};
use crate::app::state::InputMode;

pub fn render_error_modal(
    f: &mut Frame,
    area: Rect,
    message: &str,
    title: Option<&str>,
    input_mode: &InputMode,
) {
    if *input_mode != InputMode::ErrorModal {
        return;
    }

    let message_lines: Vec<&str> = message.split('\n').collect();
    let longest_line_len = message_lines
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0);

    let content_width = longest_line_len.saturating_add(10);

    let width = (content_width as u16)
        .max(40)
        .min(area.width.saturating_sub(4));

    let message_height = message_lines.len();
    let calculated_height = (message_height + 5) as u16;

    let height = calculated_height.max(10).min(area.height.saturating_sub(4));

    let config = ModalConfig {
        title: title.unwrap_or("Error").to_string(),
        min_width: width,
        min_height: height,
        border_color: Color::Red,
        bg_color: Color::Black,
    };

    render_modal_frame(f, area, &config, |f, inner_area| {
        render_error_content(f, inner_area, message);
    });
}

fn render_error_content(f: &mut Frame, area: Rect, message: &str) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(3),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(area);

    let horizontal_padding = 3;

    let apply_padding = |chunk: Rect| -> Rect {
        let padded_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(horizontal_padding)
            .constraints([Constraint::Min(0)])
            .split(chunk);
        padded_chunks[0]
    };

    let title_text = Paragraph::new("An error has occurred")
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(title_text, content_chunks[0]);

    let error_lines: Vec<&str> = message.split('\n').collect();

    let lines: Vec<Line> = error_lines
        .iter()
        .map(|line| Line::from(Span::styled(*line, Style::default().fg(Color::White))))
        .collect();

    let error_text = Paragraph::new(Text::from(lines))
        .style(Style::default())
        .alignment(Alignment::Center);
    f.render_widget(error_text, apply_padding(content_chunks[2]));

    let help_text = Paragraph::new("Press any key to continue")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help_text, content_chunks[4]);
}
