use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::{render_modal_frame, ModalConfig};
use crate::app::state::InputMode;
use crate::models::product::Product;
use crate::ui::components::helpers::layout::{
    calculate_product_column_layout, truncate_with_ellipsis, ColumnLayoutConfig,
};

pub fn render_search_modal(
    f: &mut Frame,
    area: Rect,
    search_input: &str,
    search_results: &[Product],
    selected_index: usize,
    input_mode: &InputMode,
) {
    let config = ModalConfig {
        title: "Search Products".to_string(),
        min_width: 96,
        min_height: 21,
        border_color: Color::Gray,
        bg_color: Color::Black,
    };

    render_modal_frame(f, area, &config, |f, inner_area| {
        render_search_content(
            f,
            inner_area,
            search_input,
            search_results,
            selected_index,
            input_mode,
        );
    });
}

fn render_search_content(
    f: &mut Frame,
    area: Rect,
    search_input: &str,
    search_results: &[Product],
    selected_index: usize,
    input_mode: &InputMode,
) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(9),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(area);

    let instructions = Paragraph::new("Type to search by product name, ID, or keyword:")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);
    f.render_widget(instructions, content_chunks[0]);

    let search_input_widget = Paragraph::new(search_input)
        .style(match input_mode {
            InputMode::SearchMode => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(" Search ")
                .padding(ratatui::widgets::Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                }),
        );
    f.render_widget(search_input_widget, content_chunks[1]);

    if let InputMode::SearchMode = input_mode {
        f.set_cursor_position((
            content_chunks[1].x + search_input.len() as u16 + 2,
            content_chunks[1].y + 1,
        ));
    }

    if search_results.is_empty() {
        let no_results_text = if search_input.is_empty() {
            "Start typing to search for products..."
        } else {
            "No matching products found"
        };

        let no_results = Paragraph::new(no_results_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(no_results, content_chunks[2]);
    } else {
        let content_width = content_chunks[2].width.saturating_sub(4);

        let products_refs: Vec<&Product> = search_results.iter().collect();

        let layout = calculate_product_column_layout(
            &products_refs,
            ColumnLayoutConfig {
                content_width,
                id_suffix_width: 2,
                right_margin: 2,
            },
        );

        let items: Vec<ListItem> = search_results
            .iter()
            .map(|product| {
                let price_formatted = product.price.to_string();

                let truncated_name =
                    truncate_with_ellipsis(&product.name, layout.name_column_width);

                let id_formatted = format!(
                    "{:width$}",
                    format!("{}:", product.id),
                    width = layout.id_column_width
                );

                let name_with_padding = format!(
                    "{:<width$}",
                    truncated_name,
                    width = layout.name_column_width as usize
                );

                let content = vec![Line::from(vec![
                    Span::styled(id_formatted, Style::default().fg(Color::Blue)),
                    Span::styled(name_with_padding, Style::default().fg(Color::White)),
                    Span::styled(
                        format!(
                            "{:>width$}",
                            price_formatted,
                            width = layout.price_column_width
                        ),
                        Style::default().fg(Color::Yellow),
                    ),
                ])];

                ListItem::new(content).style(Style::default())
            })
            .collect();

        let results_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" Results "))
            .highlight_style(
                Style::default()
                    .bg(Color::Gray)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        let mut list_state = ratatui::widgets::ListState::default();
        list_state.select(Some(selected_index));
        f.render_stateful_widget(results_list, content_chunks[2], &mut list_state);
    }

    let help_text =
        "'enter': Select | 'ctrl + n' or '↓': Next | 'ctrl + p' or '↑': Previous | 'esc': Cancel";
    let help_paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help_paragraph, content_chunks[3]);
}
