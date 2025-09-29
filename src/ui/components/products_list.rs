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
    app_state: &mut AppState,
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
        app_state.products.visible_range = None;
        return;
    } else if products.is_empty() {
        let empty_text = Paragraph::new("No products available")
            .style(Style::default().fg(Color::Yellow))
            .block(products_block);
        f.render_widget(empty_text, area);
        app_state.products.visible_range = None;
        return;
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

        let inner_area = products_block.inner(area);
        let available_height = inner_area.height as usize;
        
        if let Some(selected_idx) = list_state.selected() {
            if products_vec.len() > available_height {
                let max_content_height = available_height.saturating_sub(2);
                
                let scroll_offset = if selected_idx < max_content_height / 2 {
                    0
                } else if selected_idx + max_content_height / 2 >= products_vec.len() {
                    products_vec.len().saturating_sub(max_content_height)
                } else {
                    selected_idx.saturating_sub(max_content_height / 2)
                };
                
                let has_items_above = scroll_offset > 0;
                let remaining_products = products_vec.len() - scroll_offset;
                let has_items_below = remaining_products > max_content_height;
                
                let reserved_lines = match (has_items_above, has_items_below) {
                    (true, true) => 2,
                    (true, false) => 1,
                    (false, true) => 1,
                    (false, false) => 0,
                };
                let content_height = available_height.saturating_sub(reserved_lines);
                
                let final_scroll_offset = if selected_idx < content_height / 2 {
                    0
                } else if selected_idx + content_height / 2 >= products_vec.len() {
                    products_vec.len().saturating_sub(content_height)
                } else {
                    selected_idx.saturating_sub(content_height / 2)
                };
                
                let final_has_items_above = final_scroll_offset > 0;
                let final_remaining_products = products_vec.len() - final_scroll_offset;
                let final_has_items_below = final_remaining_products > content_height;
                
                let content_width = area.width.saturating_sub(4);
                let arrow_spacing = content_width / 4;
                
                let create_scroll_indicator = |arrows: String| -> ListItem {
                    let arrow1_pos = arrow_spacing;
                    let arrow2_pos = arrow_spacing * 2;
                    let arrow3_pos = arrow_spacing * 3;
                    
                    let mut spans = Vec::new();
                    
                    if arrow1_pos > 0 {
                        spans.push(Span::raw(" ".repeat(arrow1_pos as usize)));
                    }
                    spans.push(Span::styled(arrows.clone(), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)));
                    
                    if arrow2_pos > arrow1_pos + 1 {
                        spans.push(Span::raw(" ".repeat((arrow2_pos - arrow1_pos - 1) as usize)));
                    }
                    spans.push(Span::styled(arrows.clone(), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)));
                    
                    if arrow3_pos > arrow2_pos + 1 {
                        spans.push(Span::raw(" ".repeat((arrow3_pos - arrow2_pos - 1) as usize)));
                    }
                    spans.push(Span::styled(arrows, Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)));
                    
                    ListItem::new(Line::from(spans))
                };
                
                let mut final_items = Vec::new();
                
                if final_has_items_above {
                    final_items.push(create_scroll_indicator("↑".to_string()));
                }
                
                let visible_items: Vec<ListItem> = items
                    .into_iter()
                    .skip(final_scroll_offset)
                    .take(content_height)
                    .collect();
                
                final_items.extend(visible_items);
                
                if final_has_items_below {
                    final_items.push(create_scroll_indicator("↓".to_string()));
                }
                
                let mut scroll_state = ListState::default();
                let adjusted_selected = if final_has_items_above {
                    selected_idx.saturating_sub(final_scroll_offset) + 1
                } else {
                    selected_idx.saturating_sub(final_scroll_offset)
                };
                scroll_state.select(Some(adjusted_selected));
                
                let products_list = List::new(final_items)
                    .block(products_block)
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
                
                f.render_stateful_widget(products_list, area, &mut scroll_state);
                app_state.products.visible_range = Some((final_scroll_offset, final_scroll_offset + content_height));
                return;
            }
        }
        
        let products_list = List::new(items)
            .block(products_block)
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
        
        let mut mutable_list_state = list_state.clone();
        f.render_stateful_widget(products_list, area, &mut mutable_list_state);
        app_state.products.visible_range = None;
    }
}
