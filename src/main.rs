#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use kant::ui::app::{load_window_icon, App};
use kant::ui::platform::initialize_platform_defaults;

fn main() -> iced::Result {
    initialize_platform_defaults();

    let initial_path = std::env::args().nth(1).map(std::path::PathBuf::from);
    let app_creator = move || match initial_path {
        Some(ref p) => App::new_with_path(Some(p.clone())),
        None => App::new_auto_open(),
    };

    iced::application(app_creator, App::update, App::view)
        .title("Kant - Begrebs- og Informationsmodellering med FDA")
        .theme(App::theme)
        .style(|_state, _theme| iced::theme::Style {
            background_color: kant::ui::theme::ThemeColors::SURFACE_BG,
            text_color: kant::ui::theme::ThemeColors::SLATE_800,
        })
        .subscription(App::subscription)
        .window(iced::window::Settings {
            size: iced::Size::new(1280.0, 800.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            icon: load_window_icon(),
            ..Default::default()
        })
        // 4x MSAA antialiasing giver silkebløde vektorlinjer og kurver på Linux og macOS.
        // På Windows holdes det deaktiveret for at undgå GPU DX12 swapchain-stalls under Aero Snap (ADR 010).
        .antialiasing(cfg!(not(target_os = "windows")))
        .run()
}
