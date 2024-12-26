use chrono::{Datelike, Local, Timelike};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_title(f: &mut Frame, area: Rect) {
    let now = Local::now();

    let datetime_str = format!(
        "{:02}/{:02} - {}  {:02}:{:02}:{:02}",
        now.day(),
        now.month(),
        now.year(),
        now.hour(),
        now.minute(),
        now.second()
    );

    let title_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(
            " {} ",
            Span::styled(
                "Stregsystemet-TUI v1.0",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            )
        ))
        .style(
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .padding(ratatui::widgets::Padding {
            left: 1,
            right: 1,
            top: 0,
            bottom: 0,
        });

    let title_text = Paragraph::new(datetime_str)
        .style(Style::default().fg(Color::Gray))
        .block(title_block);

    f.render_widget(title_text, area);
}

pub fn render_username(f: &mut Frame, area: Rect, username: Option<&String>) {
    if let Some(username) = username {
        let max_name_len = area.width.saturating_sub(8) as usize;
        let display_name = if username.len() > max_name_len {
            if max_name_len > 3 {
                format!("{}...", &username[0..max_name_len - 3])
            } else {
                username[0..max_name_len.min(username.len())].to_string()
            }
        } else {
            username.clone()
        };

        let username_text = Paragraph::new(display_name)
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" User ")
                    .title_style(Style::default().add_modifier(Modifier::BOLD))
                    .padding(ratatui::widgets::Padding {
                        left: 1,
                        right: 1,
                        top: 0,
                        bottom: 0,
                    }),
            );
        f.render_widget(username_text, area);
    } else {
        let unknown_text = Paragraph::new("not logged in")
            .style(Style::default().fg(Color::Red))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" User ")
                    .title_style(Style::default().add_modifier(Modifier::BOLD))
                    .padding(ratatui::widgets::Padding {
                        left: 1,
                        right: 1,
                        top: 0,
                        bottom: 0,
                    }),
            );
        f.render_widget(unknown_text, area);
    }
}
