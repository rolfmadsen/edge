use iced::Color;

pub struct ThemeColors;

impl ThemeColors {
    pub const PRIMARY: Color = Color::from_rgb(0.08, 0.38, 0.74); // Classic FDA Danish blue
    pub const PRIMARY_HOVER: Color = Color::from_rgb(0.12, 0.45, 0.85);
    pub const BACKGROUND_LIGHT: Color = Color::from_rgb(0.96, 0.97, 0.98);
    pub const SURFACE_WHITE: Color = Color::from_rgb(1.0, 1.0, 1.0);
    pub const TEXT_DARK: Color = Color::from_rgb(0.12, 0.14, 0.17);
    pub const TEXT_MUTED: Color = Color::from_rgb(0.45, 0.49, 0.55);
    pub const BORDER_COLOR: Color = Color::from_rgb(0.85, 0.88, 0.92);
    pub const ACCENT_GREEN: Color = Color::from_rgb(0.13, 0.65, 0.42);
    pub const ACCENT_RED: Color = Color::from_rgb(0.84, 0.22, 0.22);
}
