use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub mod layout_constants {
    pub const HEADER_HEIGHT: u16 = 3;
    pub const INSTRUCTIONS_HEIGHT: u16 = 3;
    pub const MIN_CONTENT_HEIGHT: u16 = 10;
    pub const MIN_PRODUCT_WIDTH: u16 = 50;
    pub const USER_PANEL_WIDTH: u16 = 54;
    pub const USERNAME_AREA_WIDTH: u16 = 24;
    pub const MIN_TITLE_WIDTH: u16 = 20;
    pub const WELCOME_MESSAGE_HEIGHT: u16 = 4;
}

pub fn create_main_layout(frame_size: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(layout_constants::HEADER_HEIGHT),
                Constraint::Min(layout_constants::MIN_CONTENT_HEIGHT),
                Constraint::Length(layout_constants::INSTRUCTIONS_HEIGHT),
            ]
            .as_ref(),
        )
        .split(frame_size)
        .to_vec()
}

pub fn create_top_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Min(layout_constants::MIN_TITLE_WIDTH),
                Constraint::Length(layout_constants::USERNAME_AREA_WIDTH),
            ]
            .as_ref(),
        )
        .split(area)
        .to_vec()
}

pub fn create_middle_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Min(layout_constants::MIN_PRODUCT_WIDTH),
                Constraint::Length(layout_constants::USER_PANEL_WIDTH),
            ]
            .as_ref(),
        )
        .split(area)
        .to_vec()
}

pub fn create_welcome_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Length(layout_constants::WELCOME_MESSAGE_HEIGHT),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(area)
        .to_vec()
}
