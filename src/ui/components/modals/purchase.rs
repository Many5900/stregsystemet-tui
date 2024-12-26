use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};

use super::{render_modal_frame, ModalConfig};
use crate::models::product::Product;
use crate::utils::money::Money;

pub fn render_buy_modal(
    f: &mut Frame,
    area: Rect,
    product_id: &str,
    product: &Product,
    quantity: u32,
    current_balance: Option<Money>,
    error: Option<&String>,
    success: bool,
) {
    let (title, border_color) = if success {
        ("Purchase Successful!".to_string(), Color::Green)
    } else if error.is_some() {
        ("Purchase Failed".to_string(), Color::Red)
    } else {
        ("Confirm Purchase".to_string(), Color::Gray)
    };

    let config = ModalConfig {
        title,
        min_width: 65,
        min_height: 14,
        border_color,
        bg_color: Color::Black,
    };

    render_modal_frame(f, area, &config, |f, inner_area| {
        render_purchase_content(
            f,
            inner_area,
            product_id,
            product,
            quantity,
            current_balance,
            error,
            success,
        );
    });
}

fn render_purchase_content(
    f: &mut Frame,
    area: Rect,
    product_id: &str,
    product: &Product,
    quantity: u32,
    current_balance: Option<Money>,
    error: Option<&String>,
    success: bool,
) {
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
                Constraint::Length(1),
                Constraint::Length(if error.is_some() {
                    2
                } else if success {
                    1
                } else {
                    2
                }),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(0),
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

    let total_price = product.price * quantity;

    let price_formatted = product.price.to_string();

    let total_formatted = total_price.to_string();

    let product_id_text = Text::from(vec![Line::from(vec![
        Span::styled(
            "Product ID: ",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(product_id, Style::default().fg(Color::White)),
    ])]);
    f.render_widget(
        Paragraph::new(product_id_text),
        apply_padding(content_chunks[0]),
    );

    let product_name_text = Text::from(vec![Line::from(vec![
        Span::styled(
            "Product: ",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(&product.name, Style::default().fg(Color::White)),
    ])]);
    f.render_widget(
        Paragraph::new(product_name_text),
        apply_padding(content_chunks[1]),
    );

    let price_text = Text::from(vec![Line::from(vec![
        Span::styled(
            "Price: ",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{price_formatted} × {quantity} = {total_formatted}"),
            Style::default().fg(Color::Yellow),
        ),
    ])]);
    f.render_widget(Paragraph::new(price_text), apply_padding(content_chunks[2]));

    let quantity_text = Text::from(vec![Line::from(vec![
        Span::styled(
            "Quantity: ",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{quantity}"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            "[-/+] or [←/→]: Adjust quantity",
            Style::default().fg(Color::Gray),
        ),
    ])]);
    f.render_widget(
        Paragraph::new(quantity_text),
        apply_padding(content_chunks[3]),
    );

    if let Some(bal) = current_balance {
        let sufficient = bal >= total_price;
        let balance_color = if sufficient { Color::Green } else { Color::Red };

        let balance_text = Text::from(vec![Line::from(vec![
            Span::styled(
                "Your balance: ",
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(bal.to_string(), Style::default().fg(balance_color)),
            if !sufficient {
                Span::styled(
                    "  (Insufficient for this purchase)",
                    Style::default().fg(Color::Red),
                )
            } else {
                Span::raw("")
            },
        ])]);
        f.render_widget(
            Paragraph::new(balance_text),
            apply_padding(content_chunks[4]),
        );
    }

    if success {
        let success_text = Text::from(vec![Line::from(vec![Span::styled(
            "Purchase completed successfully!",
            Style::default().fg(Color::Green),
        )])]);
        f.render_widget(
            Paragraph::new(success_text).alignment(Alignment::Center),
            content_chunks[6],
        );
    } else if let Some(err) = error {
        let error_lines: Vec<&str> = err.split('\n').collect();

        if error_lines.len() > 1 {
            let error_area = content_chunks[6];
            let multi_error_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)])
                .split(error_area);

            let first_line = Paragraph::new(error_lines[0])
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center);
            f.render_widget(first_line, multi_error_chunks[0]);

            let second_line = Paragraph::new(error_lines[1])
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center);
            f.render_widget(second_line, multi_error_chunks[1]);
        } else {
            let error_text = Paragraph::new(err.as_str())
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center);
            f.render_widget(error_text, content_chunks[6]);
        }
    } else {
        let confirm_text = Text::from(vec![Line::from(vec![Span::styled(
            "Are you sure you want to purchase this item?",
            Style::default().fg(Color::White),
        )])]);
        f.render_widget(
            Paragraph::new(confirm_text).alignment(Alignment::Center),
            content_chunks[6],
        );
    }

    let help_text = if success || error.is_some() {
        "Press any key to close"
    } else {
        "Press 'y' to confirm or 'n' to cancel"
    };

    let help_paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help_paragraph, content_chunks[8]);
}
