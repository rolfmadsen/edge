#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use kant::ui::app::App;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Kant - Begrebs- og Informationsmodellering med FDA")
        .theme(App::theme)
        .subscription(App::subscription)
        .run()
}
