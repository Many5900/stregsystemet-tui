use crate::app::state::{HelpModalState, HelpTab, InputMode};
use crate::ui::components::modals::{render_modal_frame, ModalConfig};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_help_modal(
    f: &mut Frame,
    area: Rect,
    help_state: &HelpModalState,
    _input_mode: &InputMode,
) {
    let config = ModalConfig {
        title: format!(
            "Help - ({}/7) {}",
            get_tab_index(help_state.current_tab) + 1,
            help_state.current_tab.title()
        ),
        min_width: 80,
        min_height: 25,
        border_color: Color::Gray,
        bg_color: Color::Black,
    };

    render_modal_frame(f, area, &config, |f, inner_area| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(2),
            ])
            .split(inner_area);

        render_tab_navigation_info(f, chunks[0]);
        render_help_content(f, chunks[1], help_state.current_tab);
        render_modal_navigation_info(f, chunks[2]);
    });
}

fn get_tab_index(tab: HelpTab) -> usize {
    match tab {
        HelpTab::Home => 0,
        HelpTab::Buy => 1,
        HelpTab::Search => 2,
        HelpTab::InsertMoney => 3,
        HelpTab::Parking => 4,
        HelpTab::ChangeUsername => 5,
        HelpTab::HelpModal => 6,
    }
}

fn render_tab_navigation_info(f: &mut Frame, area: Rect) {
    let text = Text::from(vec![
        Line::from(""),
        Line::from("Navigate: → / ctrl+n / tab (Next) | ← / ctrl+p / shift+tab (Previous)"),
        Line::from(""),
    ]);

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(paragraph, area);
}

fn render_modal_navigation_info(f: &mut Frame, area: Rect) {
    let text = Paragraph::new("Press 'esc' or 'q' to close help")
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(text, area);
}

fn render_help_content(f: &mut Frame, area: Rect, current_tab: HelpTab) {
    match current_tab {
        HelpTab::Home => render_home_help(f, area),
        HelpTab::Buy => render_buy_help(f, area),
        HelpTab::Search => render_search_help(f, area),
        HelpTab::InsertMoney => render_insert_money_help(f, area),
        HelpTab::Parking => render_parking_help(f, area),
        HelpTab::ChangeUsername => render_change_username_help(f, area),
        HelpTab::HelpModal => render_help_modal_help(f, area),
    }
}

fn render_home_help(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Length(9)])
        .split(area);

    let navigation_text = Text::from(vec![
        Line::from(vec![
            Span::raw("Move down in product list: "),
            Span::styled(
                "j",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "↓",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Move up in product list: "),
            Span::styled(
                "k",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "↑",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Go to top of product list: "),
            Span::styled(
                "gg",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Go to bottom of product list: "),
            Span::styled(
                "G",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Jump relative number of items up/down: "),
            Span::styled(
                "[0-9]+",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" followed by "),
            Span::styled(
                "[jk↓↑]{1}",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let actions_text = Text::from(vec![
        Line::from(vec![
            Span::raw("Buy selected product: "),
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Open search modal: "),
            Span::styled(
                "/",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "s",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Change username: "),
            Span::styled(
                "u",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Open parking modal: "),
            Span::styled(
                "p",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Insert money (QR payment): "),
            Span::styled(
                "m",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Show this help modal: "),
            Span::styled(
                "h",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "?",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("     "),
            Span::styled(
                "NOTE: ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "?",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" is available everywhere"),
        ]),
        Line::from(vec![
            Span::raw("Quit application: "),
            Span::styled(
                "q",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let navigation_paragraph = Paragraph::new(navigation_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Product List Navigation ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(navigation_paragraph, chunks[0]);

    let actions_paragraph = Paragraph::new(actions_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Actions ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(actions_paragraph, chunks[1]);
}

fn render_buy_help(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Length(4)])
        .split(area);

    let purchase_text = Text::from(vec![
        Line::from(vec![
            Span::raw("Confirm purchase: "),
            Span::styled(
                "y",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Cancel purchase: "),
            Span::styled(
                "n",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let quantity_text = Text::from(vec![
        Line::from(vec![
            Span::raw("Increase quantity: "),
            Span::styled(
                "+",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "→",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "=",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Decrease quantity: "),
            Span::styled(
                "-",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "←",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "_",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let purchase_paragraph = Paragraph::new(purchase_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Purchase Actions ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(purchase_paragraph, chunks[0]);

    let quantity_paragraph = Paragraph::new(quantity_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Quantity Controls ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(quantity_paragraph, chunks[1]);
}

fn render_search_help(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Length(5)])
        .split(area);

    let input_text = Text::from(vec![
        Line::from(vec![
            Span::raw("Search for products: "),
            Span::styled(
                "[0-9a-zA-Z]+",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Close search modal: "),
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let navigation_text = Text::from(vec![
        Line::from(vec![
            Span::raw("Next search result: "),
            Span::styled(
                "↓",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "Ctrl + n",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Previous search result: "),
            Span::styled(
                "↑",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "Ctrl + p",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Select highlighted search result: "),
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let input_paragraph = Paragraph::new(input_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Search Input ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(input_paragraph, chunks[0]);

    let navigation_paragraph = Paragraph::new(navigation_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Search Navigation ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(navigation_paragraph, chunks[1]);
}

fn render_insert_money_help(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Length(4)])
        .split(area);

    let amount_text = Text::from(vec![
        Line::from(vec![
            Span::raw("Enter amount to be added to your account: "),
            Span::styled(
                "[0-9]+(\\.[0-9]{1,2})?",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Generate QR code for payment: "),
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Close insert money modal: "),
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let qr_text = Text::from(vec![
        Line::from(vec![
            Span::raw("Back to amount input: "),
            Span::styled(
                "b",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "Backspace",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Close QR display: "),
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let amount_paragraph = Paragraph::new(amount_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Amount Input ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(amount_paragraph, chunks[0]);

    let qr_paragraph = Paragraph::new(qr_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " QR Display ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(qr_paragraph, chunks[1]);
}

fn render_parking_help(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Length(4)])
        .split(area);

    let input_text = Text::from(vec![
        Line::from(vec![
            Span::raw("Enter phone number: "),
            Span::styled(
                "[0-9]{8}",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Enter license plate: "),
            Span::styled(
                "[A-Z]{2}[0-9]{5}",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Next field: "),
            Span::styled(
                "Tab",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Previous field: "),
            Span::styled(
                "Shift + Tab",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Confirm parking input: "),
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Close parking modal: "),
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let confirm_text = Text::from(vec![
        Line::from(vec![
            Span::raw("Confirm parking registration: "),
            Span::styled(
                "y",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Cancel parking registration: "),
            Span::styled(
                "n",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let input_paragraph = Paragraph::new(input_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Parking Input ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(input_paragraph, chunks[0]);

    let confirm_paragraph = Paragraph::new(confirm_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Parking Confirmation ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(confirm_paragraph, chunks[1]);
}

fn render_change_username_help(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(4)])
        .split(area);

    let input_text = Text::from(vec![Line::from(vec![
        Span::raw("Enter username: "),
        Span::styled(
            "[^?]+",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
    ])]);

    let actions_text = Text::from(vec![
        Line::from(vec![
            Span::raw("Confirm username change: "),
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Cancel username change: "),
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let input_paragraph = Paragraph::new(input_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Username Input ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(input_paragraph, chunks[0]);

    let actions_paragraph = Paragraph::new(actions_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Username Actions ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(actions_paragraph, chunks[1]);
}

fn render_help_modal_help(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(9),
        ])
        .split(area);

    let opening_text = Text::from(vec![
        Line::from(vec![
            Span::raw("Show this help modal: "),
            Span::styled(
                "h",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "?",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("     "),
            Span::styled(
                "NOTE: ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "?",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" is available everywhere"),
        ]),
        Line::from(vec![
            Span::raw("Close this help modal: "),
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "q",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let navigation_text = Text::from(vec![
        Line::from(vec![
            Span::raw("Next tab: "),
            Span::styled(
                "→",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "Ctrl + n",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "Tab",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Previous tab: "),
            Span::styled(
                "←",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "Ctrl + p",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "Shift + Tab",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let tabs_text = Text::from(vec![
        Line::from("1. Home - Main navigation and product list controls"),
        Line::from("2. Buy - Purchase confirmation and quantity controls"),
        Line::from("3. Search - Product search controls"),
        Line::from("4. Insert Money - QR payment system"),
        Line::from("5. Parking - Vehicle registration system"),
        Line::from("6. Change Username - User account management"),
        Line::from("7. Help Modal - This help system"),
    ]);

    let opening_paragraph = Paragraph::new(opening_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Help Controls ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(opening_paragraph, chunks[0]);

    let navigation_paragraph = Paragraph::new(navigation_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Tab Navigation ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(navigation_paragraph, chunks[1]);

    let tabs_paragraph = Paragraph::new(tabs_text)
        .block(
            Block::default()
                .title(Span::styled(
                    " Available Tabs ",
                    Style::default().fg(Color::Yellow),
                ))
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(tabs_paragraph, chunks[2]);
}
