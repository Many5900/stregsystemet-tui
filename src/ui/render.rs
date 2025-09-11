use ratatui::{layout::Rect, Frame};

use crate::app::state::{AppState, InputMode};

use crate::ui::components::header;
use crate::ui::components::modals::{
    error, parking, purchase, qr_payment, search, terminal_size, username,
};
use crate::ui::components::products_list::render_products;
use crate::ui::components::user_panel::render_user_panel;
use crate::ui::layout;

pub struct InstructionsViewState<'a> {
    pub username: Option<&'a String>,
    pub input: &'a str,
    pub input_mode: &'a InputMode,
}

pub fn ui(f: &mut Frame, app: &AppState) {
    if app.modals.terminal_size.visible {
        let size = f.area();
        terminal_size::render_terminal_size_modal(
            f,
            size,
            &app.ui.input_mode,
            size.width,
            size.height,
        );
        return;
    }

    let main_chunks = layout::create_main_layout(f.area());
    let top_chunks = layout::create_top_layout(main_chunks[0]);

    header::render_title(f, top_chunks[0]);
    header::render_username(f, top_chunks[1], app.config.username.as_ref());

    if app.config.username.is_some() {
        render_logged_in_content(f, main_chunks[1], app);
    } else {
        render_welcome_screen(f, main_chunks[1]);
    }

    render_instructions(
        f,
        main_chunks[2],
        &InstructionsViewState {
            username: app.config.username.as_ref(),
            input: &app.ui.input,
            input_mode: &app.ui.input_mode,
        },
    );

    if app.modals.username.visible {
        username::render_username_modal(
            f,
            f.area(),
            &app.modals.username.input,
            &app.ui.input_mode,
        );
    }

    if app.modals.purchase.visible {
        let product_details = app
            .modals
            .purchase
            .selected_product_id
            .as_ref()
            .and_then(|id| app.products.items.get(id).map(|product| (id, product)));

        if let Some((product_id, product)) = product_details {
            purchase::render_buy_modal(
                f,
                f.area(),
                product_id,
                product,
                app.modals.purchase.quantity,
                app.user.member_info.as_ref().map(|info| info.balance),
                app.modals.purchase.error.as_ref(),
                app.modals.purchase.success,
            );
        }
    }

    if app.modals.search.visible {
        search::render_search_modal(
            f,
            f.area(),
            &app.modals.search.input,
            &app.modals.search.results,
            app.modals.search.selected_index,
            &app.ui.input_mode,
        );
    }

    if app.modals.parking.visible {
        parking::render_parking_modal(f, f.area(), &app.modals.parking, &app.ui.input_mode);
    }

    if app.modals.qr_payment.visible {
        qr_payment::render_qr_payment_modal(
            f,
            f.area(),
            &app.modals.qr_payment,
            &app.ui.input_mode,
        );
    }

    if app.modals.error.visible && app.modals.error.message.is_some() {
        error::render_error_modal(
            f,
            f.area(),
            app.modals.error.message.as_ref().unwrap(),
            app.modals.error.title.as_deref(),
            &app.ui.input_mode,
        );
    }
}

fn render_logged_in_content(f: &mut Frame, area: Rect, app: &AppState) {
    let middle_chunks = layout::create_middle_layout(area);

    render_products(
        f,
        middle_chunks[0],
        &app.products.items,
        &app.products.error,
        &app.products.list_state,
        app,
    );

    render_user_panel(f, middle_chunks[1], &app.user);
}

fn render_welcome_screen(f: &mut Frame, area: Rect) {
    let vertical_layout = layout::create_welcome_layout(area);

    let login_message = ratatui::widgets::Paragraph::new(vec![
        ratatui::text::Line::from("Welcome to Stregsystemet-TUI!"),
        ratatui::text::Line::from("Please log in with your username to continue"),
        ratatui::text::Line::from(""),
        ratatui::text::Line::from(
            "For documentation, visit: https://github.com/Many5900/stregsystemet-tui",
        ),
    ])
    .alignment(ratatui::layout::Alignment::Center)
    .block(
        ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::NONE)
            .style(ratatui::style::Style::default()),
    );

    f.render_widget(login_message, vertical_layout[1]);
}

fn render_instructions(f: &mut Frame, area: Rect, view_state: &InstructionsViewState) {
    if view_state.username.is_none() {
        render_login_input(f, area, view_state.input, view_state.input_mode);
    } else {
        render_navigation_help(f, area);
    }
}

fn render_login_input(f: &mut Frame, area: Rect, input: &str, input_mode: &InputMode) {
    let login_input = ratatui::widgets::Paragraph::new(input)
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Blue))
                .title(format!(" {} ", "Enter Username"))
                .padding(ratatui::widgets::Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                }),
        );

    f.render_widget(login_input, area);

    if let InputMode::Editing = input_mode {
        f.set_cursor_position((area.x + input.len() as u16 + 2, area.y + 1));
    }
}

fn render_navigation_help(f: &mut Frame, area: Rect) {
    let instructions = ratatui::widgets::Paragraph::new(
        "'j' or '↓': Down | 'k' or '↑': Up | 'gg': Top | 'G': Bottom | 'enter': Buy | '/' or 's': Search | 'u': Change Username | 'p': Parking | 'm': Insert Money | 'q': Quit",
    )
    .style(ratatui::style::Style::default())
    .block(
        ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .title(format!(" {} ", "Instructions"))
            .padding(ratatui::widgets::Padding {
                left: 1,
                right: 1,
                top: 0,
                bottom: 0,
            }),
    );

    f.render_widget(instructions, area);
}
