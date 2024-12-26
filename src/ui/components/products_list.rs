use std::collections::HashMap;

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::state::AppState;
use crate::models::product::Product;
use crate::ui::components::helpers::error::render_error;
use crate::ui::components::helpers::layout::{
    calculate_product_column_layout, truncate_with_ellipsis, ColumnLayoutConfig,
};

pub fn render_products(
    f: &mut Frame,
    area: Rect,
    products: &HashMap<String, Product>,
    error: &Option<String>,
    list_state: &ListState,
    app_state: &AppState,
) {
    let products_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", "Products"))
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .padding(ratatui::widgets::Padding {
            left: 1,
            right: 1,
            top: 0,
            bottom: 0,
        });

    if let Some(ref error) = error {
        let inner_area = products_block.inner(area);
        f.render_widget(products_block, area);
        render_error(f, inner_area, error, Some("Error loading products"));
    } else if products.is_empty() {
        let empty_text = Paragraph::new("No products available")
            .style(Style::default().fg(Color::Yellow))
            .block(products_block);
        f.render_widget(empty_text, area);
    } else {
        let mut products_vec: Vec<&Product> = products.values().collect();

        products_vec.sort_by(|a, b| match (a.id.parse::<i32>(), b.id.parse::<i32>()) {
            (Ok(id_a), Ok(id_b)) => id_a.cmp(&id_b),
            _ => a.id.cmp(&b.id),
        });

        let content_width = area.width.saturating_sub(4);
        let main_content_width = content_width.saturating_sub(4);

        let layout = calculate_product_column_layout(
            &products_vec,
            ColumnLayoutConfig {
                content_width: main_content_width,
                id_suffix_width: 2,
                right_margin: 2,
            },
        );

        let selected_index = list_state.selected().unwrap_or(0);
        let target_indices = app_state.get_movement_target_indices();

        let items: Vec<ListItem> = products_vec
            .iter()
            .enumerate()
            .map(|(index, product)| {
                let relative_line = if index == selected_index {
                    format!("{}", index + 1)
                } else {
                    format!("{}", ((index as i32) - (selected_index as i32)).abs())
                };

                let price_string = product.price.to_string();

                let truncated_name =
                    truncate_with_ellipsis(&product.name, layout.name_column_width);

                let id_formatted = format!(
                    "{:width$}",
                    format!("{}:", product.id),
                    width = layout.id_column_width
                );

                let name_with_space = format!(
                    "{:<width$}",
                    truncated_name,
                    width = layout.name_column_width as usize
                );

                let (content_style, line_number_style, is_target) = if index == selected_index {
                    let style = Style::default().add_modifier(Modifier::REVERSED);
                    (style, style.add_modifier(Modifier::BOLD), false)
                } else if target_indices.contains(&index) {
                    let style = Style::default()
                        .bg(Color::Blue)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD);
                    (style, style, true)
                } else {
                    (Style::default(), Style::default().fg(Color::Gray), false)
                };

                let line = Line::from(vec![
                    Span::styled(format!("{relative_line:>3} "), line_number_style),
                    Span::styled(id_formatted, content_style),
                    Span::styled(
                        name_with_space,
                        if index == selected_index || is_target {
                            content_style
                        } else {
                            content_style.fg(Color::White)
                        },
                    ),
                    Span::styled("  ", Style::default()),
                    Span::styled(
                        format!(
                            "{:>width$}",
                            price_string,
                            width = layout.price_column_width
                        ),
                        if is_target {
                            content_style
                        } else {
                            content_style.fg(Color::Yellow)
                        },
                    ),
                ]);

                let mut item = ListItem::new(vec![line]);
                if index == selected_index {
                    item = item.style(Style::default().add_modifier(Modifier::REVERSED));
                } else if is_target {
                    item = item.style(
                        Style::default()
                            .bg(Color::Blue)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    );
                }
                item
            })
            .collect();

        let products_list = List::new(items).block(products_block);

        f.render_widget(products_list, area);
    }
}
