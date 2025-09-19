use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_qrcode::{Colors, QrCodeWidget};

use super::{render_modal_frame, ModalConfig};
use crate::app::state::{InputMode, QrPaymentModalState};

pub fn render_qr_payment_modal(
    f: &mut Frame,
    area: Rect,
    qr_state: &QrPaymentModalState,
    input_mode: &InputMode,
) {
    if qr_state.showing_qr {
        render_qr_display_modal(f, area, qr_state, input_mode);
    } else {
        render_amount_input_modal(f, area, qr_state, input_mode);
    }
}

fn render_amount_input_modal(
    f: &mut Frame,
    area: Rect,
    qr_state: &QrPaymentModalState,
    input_mode: &InputMode,
) {
    let config = ModalConfig {
        title: "Payment QR Code Generator".to_string(),
        min_width: 43,
        min_height: 10,
        border_color: Color::Gray,
        bg_color: Color::Black,
    };

    render_modal_frame(f, area, &config, |f, inner_area| {
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
            .split(inner_area);

        let instruction_area = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(3)
            .constraints([Constraint::Min(0)])
            .split(content_chunks[0])[0];

        let instructions = Paragraph::new(" Enter amount (min. 50 DKK):")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);
        f.render_widget(instructions, instruction_area);

        let input_area = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(3)
            .constraints([Constraint::Min(0)])
            .split(content_chunks[1])[0];

        let amount_input = Paragraph::new(qr_state.amount_input.as_str())
            .style(match input_mode {
                InputMode::QrPaymentAmount => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue))
                    .padding(ratatui::widgets::Padding {
                        left: 1,
                        right: 1,
                        top: 0,
                        bottom: 0,
                    }),
            );
        f.render_widget(amount_input, input_area);

        if let InputMode::QrPaymentAmount = input_mode {
            f.set_cursor_position((
                input_area.x + qr_state.amount_input.len() as u16 + 2,
                input_area.y + 1,
            ));
        }

        let help_text = Paragraph::new("'enter': Generate QR | 'esc': Cancel")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(help_text, content_chunks[3]);
    });
}

fn render_qr_display_modal(
    f: &mut Frame,
    area: Rect,
    qr_state: &QrPaymentModalState,
    _input_mode: &InputMode,
) {
    if let Some(ref qr_data) = qr_state.qr_data {
        let qr_width = if let Some(ref qr_code) = qr_data.qr_code {
            let module_count = qr_code.width();
            let qr_display_width = (module_count as f32 * 1.30) as usize;
            (qr_display_width + 10) as u16
        } else {
            60u16
        };

        let config = ModalConfig {
            title: format!("Payment QR: {:.2} DKK", qr_data.amount),
            min_width: qr_width,
            min_height: 31,
            border_color: Color::Green,
            bg_color: Color::Black,
        };

        render_modal_frame(f, area, &config, |f, inner_area| {
            let content_chunks = Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin(1)
                .constraints(
                    [
                        Constraint::Min(20),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(inner_area);

            if let Some(ref qr_code) = qr_data.qr_code {
                let module_count = qr_code.width();
                let qr_display_width = ((module_count as f32 * 1.30) as usize) as u16;

                let qr_horizontal = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(5),
                        Constraint::Length(qr_display_width),
                        Constraint::Length(5),
                    ])
                    .split(content_chunks[0]);

                let qr_widget = QrCodeWidget::new(qr_code.clone()).colors(Colors::Inverted);
                f.render_widget(qr_widget, qr_horizontal[1]);
            }

            let amount_info = Paragraph::new(format!(
                "Amount: {:.2} DKK → {} (via MobilePay)",
                qr_data.amount, qr_data.username
            ))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center);
            f.render_widget(amount_info, content_chunks[2]);

            let help_text = Paragraph::new("'b' or 'backspace': Back | 'esc': Close")
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center);
            f.render_widget(help_text, content_chunks[4]);
        });
    }
}
