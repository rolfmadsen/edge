#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use kant::ui::app::{load_window_icon, App};

fn main() -> iced::Result {
    let initial_path = std::env::args().nth(1).map(std::path::PathBuf::from);
    let app_creator = move || match initial_path {
        Some(ref p) => App::new_with_path(Some(p.clone())),
        None => App::new_auto_open(),
    };

    iced::application(app_creator, App::update, App::view)
        .title("Kant - Begrebs- og Informationsmodellering med FDA")
        .theme(App::theme)
        .subscription(App::subscription)
        .window(iced::window::Settings {
            icon: load_window_icon(),
            ..Default::default()
        })
        .antialiasing(false)
        .run()
}

