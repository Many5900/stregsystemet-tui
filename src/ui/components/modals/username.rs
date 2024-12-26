use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

use super::{render_modal_frame, ModalConfig};
use crate::app::state::InputMode;

pub fn render_username_modal(f: &mut Frame, area: Rect, username: &str, input_mode: &InputMode) {
    let config = ModalConfig {
        title: "Change Username".to_string(),
        min_width: 40,
        min_height: 10,
        border_color: Color::Gray,
        bg_color: Color::Black,
    };

    render_modal_frame(f, area, &config, |f, inner_area| {
        render_username_content(f, inner_area, username, input_mode);
    });
}

fn render_username_content(f: &mut Frame, area: Rect, username: &str, input_mode: &InputMode) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(area);

    let instruction_area = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .constraints([Constraint::Min(0)])
        .split(content_chunks[0])[0];

    let instructions = Paragraph::new(" Enter new username:")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);
    f.render_widget(instructions, instruction_area);

    let input_area = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .constraints([Constraint::Min(0)])
        .split(content_chunks[1])[0];

    let username_input = Paragraph::new(username)
        .style(match input_mode {
            InputMode::EditingUsername => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .padding(ratatui::widgets::Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                }),
        );
    f.render_widget(username_input, input_area);

    let help_text = Paragraph::new("'enter': Save | 'esc': Cancel")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help_text, content_chunks[3]);

    if let InputMode::EditingUsername = input_mode {
        let cursor_x = input_area.x
            + username
                .len()
                .min((input_area.width as usize).saturating_sub(5)) as u16
            + 2;

        f.set_cursor_position((cursor_x, input_area.y + 1));
    }
}
