use iced::Color;

pub struct ThemeColors;

impl ThemeColors {
    // Brand & Navigation
    pub const PRIMARY: Color = Color::from_rgb(0.08, 0.38, 0.74); // FDA Classic Danish Blue
    pub const PRIMARY_HOVER: Color = Color::from_rgb(0.12, 0.45, 0.85);
    pub const PRIMARY_LIGHT: Color = Color::from_rgb(0.92, 0.95, 0.99);

    // Surfaces & Backgrounds
    pub const BACKGROUND_LIGHT: Color = Color::from_rgb(0.97, 0.975, 0.985);
    pub const SURFACE_WHITE: Color = Color::from_rgb(1.0, 1.0, 1.0);
    pub const ROW_HOVER: Color = Color::from_rgb(0.96, 0.97, 0.99);
    pub const BORDER_COLOR: Color = Color::from_rgb(0.86, 0.89, 0.92);
    pub const BORDER_LIGHT: Color = Color::from_rgb(0.91, 0.93, 0.95);

    // Typography
    pub const TEXT_DARK: Color = Color::from_rgb(0.12, 0.14, 0.18);
    pub const TEXT_MUTED: Color = Color::from_rgb(0.42, 0.46, 0.52);

    // FDA Modelregler Anbefalede Farver (Kapitel 7.3)
    pub const FDA_SAND: Color = Color::from_rgb(0.996, 0.980, 0.969); // #FEFAF7 - Egen klasse/lokalt begreb
    pub const FDA_SAND_BORDER: Color = Color::from_rgb(0.91, 0.86, 0.80);
    pub const FDA_BORROWED_BLUE: Color = Color::from_rgb(0.529, 0.804, 0.922); // #87CDEB - Indlånt/genbrugt klasse
    pub const FDA_BORROWED_BLUE_BG: Color = Color::from_rgb(0.90, 0.96, 0.99);
    pub const FDA_DATA_TYPE_YELLOW: Color = Color::from_rgb(0.984, 0.976, 0.776); // #FBF9C6 - Datatyper
    pub const FDA_ENUM_GREEN: Color = Color::from_rgb(0.910, 0.992, 0.890); // #E8FDE3 - Enumerationer

    // Status & Feedback
    pub const ACCENT_GREEN: Color = Color::from_rgb(0.13, 0.65, 0.42);
    pub const ACCENT_GREEN_LIGHT: Color = Color::from_rgb(0.91, 0.98, 0.93);
    pub const ACCENT_RED: Color = Color::from_rgb(0.84, 0.22, 0.22);
    pub const ACCENT_RED_LIGHT: Color = Color::from_rgb(0.99, 0.92, 0.92);
}
