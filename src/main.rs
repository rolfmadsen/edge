use edge::ui::app::App;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("edge - FDA Begrebs- og Informationsmodellering")
        .run()
}
