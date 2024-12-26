pub fn centered_rect(
    width: u16,
    height: u16,
    area: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    ratatui::layout::Rect::new(x, y, width, height)
}
