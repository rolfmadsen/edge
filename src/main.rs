use edge::ui::app::App;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Edge - Begrebs- og Informationsmodellering med FDA")
        .theme(App::theme)
        .subscription(App::subscription)
        .run()
}
