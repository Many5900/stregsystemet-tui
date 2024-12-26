use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};

use super::{render_modal_frame, ModalConfig};
use crate::app::modals::terminal_size::{MIN_TERMINAL_HEIGHT, MIN_TERMINAL_WIDTH};
use crate::app::state::InputMode;

pub fn render_terminal_size_modal(
    f: &mut Frame,
    area: Rect,
    input_mode: &InputMode,
    current_width: u16,
    current_height: u16,
) {
    if *input_mode != InputMode::TerminalSizeModal {
        return;
    }

    let title = "Terminal Too Small";

    let content = vec![
        Line::from(vec![Span::styled(
            "Your terminal is too small to display 'stui' properly.",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Current size: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{current_width}×{current_height}"),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("Required minimum: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{MIN_TERMINAL_WIDTH}×{MIN_TERMINAL_HEIGHT}"),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Suggestions:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("• ", Style::default().fg(Color::Gray)),
            Span::styled(
                "Zoom out [Ctrl]/[Cmd] + [-]",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("• ", Style::default().fg(Color::Gray)),
            Span::styled(
                "Increase terminal window size",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("• ", Style::default().fg(Color::Gray)),
            Span::styled("Use a bigger monitor", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "The application will resume when the terminal size is adequate.",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::ITALIC),
        )]),
    ];

    let paragraph = Paragraph::new(Text::from(content))
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::White));

    let config = ModalConfig {
        title: title.to_string(),
        min_width: 70,
        min_height: 13,
        border_color: Color::Red,
        bg_color: Color::Black,
    };

    render_modal_frame(f, area, &config, |f, inner_area| {
        f.render_widget(paragraph, inner_area);
    });
}
