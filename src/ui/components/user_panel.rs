use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::state::UserState;
use crate::models::member::MemberInfo;
use crate::models::sale::Sale;
use crate::utils::formatters::truncate_text;

pub fn render_user_panel(f: &mut Frame, area: Rect, user_state: &UserState) {
    let panel_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", "User Info"))
        .style(Style::default().add_modifier(Modifier::BOLD));

    let inner_area = panel_block.inner(area);

    f.render_widget(panel_block, area);

    if let Some(ref error) = user_state.error {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(5),
                Constraint::Length(1),
                Constraint::Min(1),
            ])
            .split(inner_area);

        let (title, hint) = if error.contains("does not exist") || error.contains("not found") {
            ("User Not Found", "Press 'u' to change username")
        } else {
            ("Error", "Please try again later")
        };

        let title_text = Paragraph::new(title)
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(title_text, chunks[0]);

        let max_line_length = inner_area.width.saturating_sub(6) as usize;
        let formatted_error = split_error_message(error, max_line_length);

        let error_lines: Vec<Line> = formatted_error
            .iter()
            .map(|line| Line::from(Span::styled(line.clone(), Style::default().fg(Color::Red))))
            .collect();

        let error_text = Paragraph::new(Text::from(error_lines))
            .style(Style::default())
            .alignment(Alignment::Center);
        f.render_widget(error_text, chunks[1]);

        let hint_text = Paragraph::new(hint)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        f.render_widget(hint_text, chunks[2]);

        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3)])
        .split(inner_area);

    if let Some(ref member_info) = user_state.member_info {
        render_member_info(f, chunks[0], member_info);
        render_sales(f, chunks[1], &user_state.latest_sales);
    } else {
        let no_user_text = Paragraph::new("No user information available")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        f.render_widget(no_user_text, inner_area);
    }
}

fn render_member_info(f: &mut Frame, area: Rect, member_info: &MemberInfo) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    let balance_color = if member_info.balance >= 5000 {
        Color::Green
    } else if member_info.balance >= 1000 {
        Color::Yellow
    } else {
        Color::Red
    };

    let name_text = Text::from(vec![Line::from(vec![
        Span::styled(
            " Name: ",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(&member_info.name, Style::default().fg(Color::White)),
    ])]);
    f.render_widget(Paragraph::new(name_text), chunks[0]);

    let balance_text = Text::from(vec![Line::from(vec![
        Span::styled(
            " Balance: ",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            member_info.balance.to_string(),
            Style::default()
                .fg(balance_color)
                .add_modifier(Modifier::BOLD),
        ),
    ])]);
    f.render_widget(Paragraph::new(balance_text), chunks[1]);
}

fn render_sales(f: &mut Frame, area: Rect, sales: &[Sale]) {
    let sales_block = Block::default().borders(Borders::NONE).title(Span::styled(
        " Recent Purchases: ",
        Style::default().add_modifier(Modifier::BOLD),
    ));

    let inner_area = sales_block.inner(area);

    f.render_widget(sales_block, area);

    if sales.is_empty() {
        let no_sales_text = Paragraph::new("No recent purchases")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(no_sales_text, inner_area);
        return;
    }

    let max_price_width = sales
        .iter()
        .map(|s| format!("{},{:02} DKK", s.price.kroner(), s.price.cents()).len())
        .max()
        .unwrap_or(0);

    let available_width = inner_area.width as usize;
    let bullet_width = 3;
    let padding = 1;
    let name_width = available_width.saturating_sub(bullet_width + max_price_width + padding);

    let items: Vec<ListItem> = sales
        .iter()
        .map(|sale| {
            let full_price = format!("{},{:02} DKK", sale.price.kroner(), sale.price.cents());

            let date_display = sale.formatted_timestamp();

            let mut sanitized_product = sale
                .product
                .replace("<br>", " ")
                .replace("<br/>", " ")
                .replace("<br />", " ");

            let mut result = String::new();
            let mut in_tag = false;

            for c in sanitized_product.chars() {
                if c == '<' {
                    in_tag = true;
                    result.push(' ');
                } else if c == '>' {
                    in_tag = false;
                } else if !in_tag {
                    result.push(c);
                }
            }
            sanitized_product = result;
            let truncated_product = truncate_text(&sanitized_product, name_width);

            let content = vec![
                Line::from(vec![Span::styled(
                    format!(" {date_display}"),
                    Style::default()
                        .fg(Color::Gray)
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(vec![
                    Span::styled(" ∟ ", Style::default().fg(Color::Gray)),
                    Span::styled(
                        format!("{truncated_product:<name_width$}"),
                        Style::default()
                            .fg(Color::Gray)
                            .remove_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{full_price:>max_price_width$}"),
                        Style::default()
                            .fg(Color::Yellow)
                            .remove_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
            ];

            ListItem::new(content)
        })
        .collect();

    let sales_list = List::new(items)
        .block(Block::default())
        .style(Style::default());

    f.render_widget(sales_list, inner_area);
}

fn split_error_message(error: &str, max_width: usize) -> Vec<String> {
    if error.is_empty() {
        return vec![];
    }

    if max_width < 10 {
        return vec![error.to_string()];
    }

    let mut result = Vec::new();
    let mut current_line = String::new();

    let words = error.split_whitespace();

    for word in words {
        if !current_line.is_empty() && current_line.len() + word.len() + 1 > max_width {
            result.push(current_line);
            current_line = word.to_string();
        } else if current_line.is_empty() {
            current_line = word.to_string();
        } else {
            current_line.push(' ');
            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        result.push(current_line);
    }

    if result.len() > 4 {
        result.truncate(3);
        result.push("...".to_string());
    }

    result
}
