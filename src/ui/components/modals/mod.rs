use crate::utils::helpers::centered_rect;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Clear},
    Frame,
};

pub mod error;
pub mod parking;
pub mod purchase;
pub mod qr_payment;
pub mod search;
pub mod terminal_size;
pub mod username;

pub struct ModalConfig {
    pub title: String,
    pub min_width: u16,
    pub min_height: u16,
    pub border_color: Color,
    pub bg_color: Color,
}

impl Default for ModalConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            min_width: 40,
            min_height: 10,
            border_color: Color::Gray,
            bg_color: Color::Black,
        }
    }
}

pub fn render_modal_frame<F>(f: &mut Frame, area: Rect, config: &ModalConfig, render_content: F)
where
    F: FnOnce(&mut Frame, Rect),
{
    let width = config.min_width.min(area.width.saturating_sub(4));
    let height = config.min_height.min(area.height.saturating_sub(4));

    let modal_area = centered_rect(width, height, area);

    f.render_widget(Clear, modal_area);

    let modal_block = Block::default()
        .title(Span::styled(
            format!(" {} ", config.title),
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(config.border_color))
        .style(Style::default().bg(config.bg_color));

    let inner_area = modal_block.inner(modal_area);

    f.render_widget(modal_block, modal_area);

    render_content(f, inner_area);
}
