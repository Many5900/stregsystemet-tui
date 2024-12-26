use crate::models::product::Product;

pub struct ColumnLayoutConfig {
    pub content_width: u16,
    pub id_suffix_width: usize,
    pub right_margin: u16,
}

pub struct ColumnLayout {
    pub id_column_width: usize,
    pub name_column_width: u16,
    pub price_column_width: usize,
}

pub fn calculate_product_column_layout(
    products: &[&Product],
    config: ColumnLayoutConfig,
) -> ColumnLayout {
    let max_id_length = products
        .iter()
        .map(|product| product.id.len())
        .max()
        .unwrap_or(0);

    let id_column_width = max_id_length + config.id_suffix_width;

    let max_price_length = products
        .iter()
        .map(|product| product.price.to_string().len())
        .max()
        .unwrap_or(0);

    let price_column_width = max_price_length;

    let name_column_width = config
        .content_width
        .saturating_sub(id_column_width as u16 + price_column_width as u16 + config.right_margin);

    ColumnLayout {
        id_column_width,
        name_column_width,
        price_column_width,
    }
}

pub fn truncate_with_ellipsis(text: &str, max_width: u16) -> String {
    let max_width = max_width as usize;
    if text.len() <= max_width {
        text.to_string()
    } else if max_width <= 3 {
        text.chars().take(max_width).collect()
    } else {
        let truncated: String = text.chars().take(max_width - 3).collect();
        format!("{truncated}...")
    }
}
