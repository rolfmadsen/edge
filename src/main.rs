#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use kant::ui::app::{load_window_icon, App};

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
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
