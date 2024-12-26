use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

use super::{render_modal_frame, ModalConfig};
use crate::app::state::{InputMode, ParkingModalState};

pub fn render_parking_modal(
    f: &mut Frame,
    area: Rect,
    parking_state: &ParkingModalState,
    input_mode: &InputMode,
) {
    if parking_state.confirming {
        render_parking_confirmation_modal(f, area, parking_state);
    } else {
        render_parking_input_modal(f, area, parking_state, input_mode);
    }
}

fn render_parking_input_modal(
    f: &mut Frame,
    area: Rect,
    parking_state: &ParkingModalState,
    input_mode: &InputMode,
) {
    let config = ModalConfig {
        title: "Parking Registration".to_string(),
        min_width: 60,
        min_height: 14,
        border_color: Color::Gray,
        bg_color: Color::Black,
    };

    render_modal_frame(f, area, &config, |f, inner_area| {
        render_parking_input_content(f, inner_area, parking_state, input_mode);
    });
}

fn render_parking_input_content(
    f: &mut Frame,
    area: Rect,
    parking_state: &ParkingModalState,
    input_mode: &InputMode,
) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(area);

    let phone_label_area = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .constraints([Constraint::Min(0)])
        .split(content_chunks[0])[0];

    let phone_label = Paragraph::new(" Phone number (8 digits):")
        .style(Style::default().fg(if parking_state.current_field == 0 {
            Color::Yellow
        } else {
            Color::White
        }))
        .alignment(Alignment::Left);
    f.render_widget(phone_label, phone_label_area);

    let phone_input_area = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .constraints([Constraint::Min(0)])
        .split(content_chunks[1])[0];

    let phone_input = Paragraph::new(parking_state.phone_input.as_str())
        .style(match input_mode {
            InputMode::ParkingModal if parking_state.current_field == 0 => {
                Style::default().fg(Color::Yellow)
            }
            _ => Style::default(),
        })
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_style(Style::default().fg(if parking_state.current_field == 0 {
                    Color::Blue
                } else {
                    Color::Gray
                }))
                .padding(ratatui::widgets::Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                }),
        );
    f.render_widget(phone_input, phone_input_area);

    let plate_label_area = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .constraints([Constraint::Min(0)])
        .split(content_chunks[2])[0];

    let plate_label = Paragraph::new(" License plate:")
        .style(Style::default().fg(if parking_state.current_field == 1 {
            Color::Yellow
        } else {
            Color::White
        }))
        .alignment(Alignment::Left);
    f.render_widget(plate_label, plate_label_area);

    let plate_input_area = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .constraints([Constraint::Min(0)])
        .split(content_chunks[3])[0];

    let plate_input = Paragraph::new(parking_state.license_plate_input.as_str())
        .style(match input_mode {
            InputMode::ParkingModal if parking_state.current_field == 1 => {
                Style::default().fg(Color::Yellow)
            }
            _ => Style::default(),
        })
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_style(Style::default().fg(if parking_state.current_field == 1 {
                    Color::Blue
                } else {
                    Color::Gray
                }))
                .padding(ratatui::widgets::Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                }),
        );
    f.render_widget(plate_input, plate_input_area);

    let help_text = Paragraph::new("'tab': Switch field | 'enter': Next | 'esc': Cancel")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help_text, content_chunks[5]);

    if let InputMode::ParkingModal = input_mode {
        let (cursor_area, text) = if parking_state.current_field == 0 {
            (phone_input_area, &parking_state.phone_input)
        } else {
            (plate_input_area, &parking_state.license_plate_input)
        };

        let cursor_x = cursor_area.x
            + text
                .len()
                .min((cursor_area.width as usize).saturating_sub(5)) as u16
            + 2;

        f.set_cursor_position((cursor_x, cursor_area.y + 1));
    }
}

fn render_parking_confirmation_modal(f: &mut Frame, area: Rect, parking_state: &ParkingModalState) {
    if parking_state.success || parking_state.error.is_some() {
        render_parking_result_modal(f, area, parking_state);
    } else {
        render_parking_confirm_modal(f, area, parking_state);
    }
}

fn render_parking_confirm_modal(f: &mut Frame, area: Rect, parking_state: &ParkingModalState) {
    let config = ModalConfig {
        title: "Confirm Parking".to_string(),
        min_width: 50,
        min_height: 9,
        border_color: Color::Yellow,
        bg_color: Color::Black,
    };

    render_modal_frame(f, area, &config, |f, inner_area| {
        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(inner_area);

        let phone_text = Paragraph::new(format!(" Phone: +45 {}", parking_state.phone_input))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);
        f.render_widget(phone_text, content_chunks[0]);

        let plate_text = Paragraph::new(format!(
            " License plate: {}",
            parking_state.license_plate_input
        ))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);
        f.render_widget(plate_text, content_chunks[1]);

        let confirm_text = Paragraph::new("Register parking for 10 hours?")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        f.render_widget(confirm_text, content_chunks[3]);

        let help_text = Paragraph::new("'y': Confirm | 'n': Cancel")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(help_text, content_chunks[4]);
    });
}

fn render_parking_result_modal(f: &mut Frame, area: Rect, parking_state: &ParkingModalState) {
    let (title, color, message) = if parking_state.success {
        (
            "Parking Registered".to_string(),
            Color::Green,
            "Please check your SMS to confirm that the parking registration was successful"
                .to_string(),
        )
    } else if let Some(ref error) = parking_state.error {
        (
            "Parking Failed".to_string(),
            Color::Red,
            format!("Registration failed: {error}"),
        )
    } else {
        (
            "Parking".to_string(),
            Color::Gray,
            "Unknown state".to_string(),
        )
    };

    let config = ModalConfig {
        title,
        min_width: 60,
        min_height: 8,
        border_color: color,
        bg_color: Color::Black,
    };

    render_modal_frame(f, area, &config, |f, inner_area| {
        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(inner_area);

        let message_text = Paragraph::new(message)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(message_text, content_chunks[0]);

        let help_text = Paragraph::new("Press any key to close")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(help_text, content_chunks[2]);
    });
}
