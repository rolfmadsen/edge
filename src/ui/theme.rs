use iced::widget::{button, container, text_input};
use iced::{Background, Border, Color, Shadow, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeColors;

impl ThemeColors {
    // Brand & Primary
    pub const PRIMARY: Color = Color::from_rgb(0.08, 0.38, 0.74); // #1461BD FDA Danish Blue
    pub const PRIMARY_HOVER: Color = Color::from_rgb(0.12, 0.45, 0.85);
    pub const PRIMARY_ACTIVE: Color = Color::from_rgb(0.06, 0.30, 0.60);
    pub const PRIMARY_LIGHT: Color = Color::from_rgb(0.92, 0.95, 0.99);

    // Modern Slate Neutrals
    pub const SLATE_50: Color = Color::from_rgb(0.976, 0.980, 0.988); // #F8FAFC
    pub const SLATE_100: Color = Color::from_rgb(0.945, 0.957, 0.973); // #F1F5F9
    pub const SLATE_200: Color = Color::from_rgb(0.886, 0.910, 0.941); // #E2E8F0
    pub const SLATE_300: Color = Color::from_rgb(0.796, 0.835, 0.882); // #CBD5E1
    pub const SLATE_400: Color = Color::from_rgb(0.576, 0.635, 0.722); // #94A3B8
    pub const SLATE_500: Color = Color::from_rgb(0.392, 0.455, 0.545); // #64748B
    pub const SLATE_600: Color = Color::from_rgb(0.278, 0.333, 0.412); // #475569
    pub const SLATE_700: Color = Color::from_rgb(0.200, 0.255, 0.333); // #334155
    pub const SLATE_800: Color = Color::from_rgb(0.118, 0.161, 0.231); // #1E293B
    pub const SLATE_900: Color = Color::from_rgb(0.059, 0.090, 0.165); // #0F172A

    // Surfaces & Backgrounds
    pub const SURFACE_BG: Color = Self::SLATE_50;
    pub const SURFACE_CARD: Color = Color::WHITE;
    pub const SURFACE_HEADER: Color = Color::WHITE;
    pub const SURFACE_ROW_HOVER: Color = Color::from_rgb(0.965, 0.975, 0.99);
    pub const SURFACE_BORDER: Color = Self::SLATE_200;
    pub const SURFACE_BORDER_LIGHT: Color = Color::from_rgb(0.92, 0.94, 0.96);

    // Legacy Aliases for backwards compatibility
    pub const BACKGROUND_LIGHT: Color = Self::SURFACE_BG;
    pub const SURFACE_WHITE: Color = Self::SURFACE_CARD;
    pub const ROW_HOVER: Color = Self::SURFACE_ROW_HOVER;
    pub const BORDER_COLOR: Color = Self::SURFACE_BORDER;
    pub const BORDER_LIGHT: Color = Self::SURFACE_BORDER_LIGHT;

    // Typography
    pub const TEXT_DARK: Color = Self::SLATE_900;
    pub const TEXT_MUTED: Color = Self::SLATE_500;
    pub const TEXT_LIGHT: Color = Color::WHITE;

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

// -----------------------------------------------------------------------------
// Container Styles
// -----------------------------------------------------------------------------

pub fn card_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(ThemeColors::SURFACE_CARD)),
        border: Border {
            color: ThemeColors::SURFACE_BORDER,
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.04),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 4.0,
        },
        ..Default::default()
    }
}

pub fn pill_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(ThemeColors::SLATE_100)),
        border: Border {
            color: ThemeColors::SLATE_200,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn modal_backdrop_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.05, 0.08, 0.15, 0.45))),
        ..Default::default()
    }
}

pub fn modal_card_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(ThemeColors::SURFACE_CARD)),
        border: Border {
            color: ThemeColors::SLATE_300,
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.18),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        },
        ..Default::default()
    }
}

// -----------------------------------------------------------------------------
// Button Styles
// -----------------------------------------------------------------------------

pub fn primary_button_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(ThemeColors::PRIMARY)),
            text_color: ThemeColors::TEXT_LIGHT,
            border: Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(ThemeColors::PRIMARY_HOVER)),
            text_color: ThemeColors::TEXT_LIGHT,
            border: Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(ThemeColors::PRIMARY_ACTIVE)),
            text_color: ThemeColors::TEXT_LIGHT,
            border: Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            shadow: Shadow::default(),
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(ThemeColors::SLATE_200)),
            text_color: ThemeColors::SLATE_400,
            border: Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            shadow: Shadow::default(),
            ..Default::default()
        },
    }
}

pub fn secondary_button_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(ThemeColors::SURFACE_CARD)),
            text_color: ThemeColors::SLATE_700,
            border: Border {
                color: ThemeColors::SLATE_200,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.03),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(ThemeColors::SLATE_50)),
            text_color: ThemeColors::SLATE_900,
            border: Border {
                color: ThemeColors::SLATE_300,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 3.0,
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(ThemeColors::SLATE_100)),
            text_color: ThemeColors::SLATE_900,
            border: Border {
                color: ThemeColors::SLATE_300,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(ThemeColors::SLATE_100)),
            text_color: ThemeColors::SLATE_400,
            border: Border {
                color: ThemeColors::SLATE_200,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            ..Default::default()
        },
    }
}

pub fn danger_button_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(ThemeColors::ACCENT_RED_LIGHT)),
            text_color: ThemeColors::ACCENT_RED,
            border: Border {
                color: ThemeColors::ACCENT_RED,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(ThemeColors::ACCENT_RED)),
            text_color: ThemeColors::TEXT_LIGHT,
            border: Border {
                color: ThemeColors::ACCENT_RED,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(ThemeColors::ACCENT_RED)),
            text_color: ThemeColors::TEXT_LIGHT,
            border: Border {
                color: ThemeColors::ACCENT_RED,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            ..Default::default()
        },
        button::Status::Disabled => secondary_button_style(_theme, status),
    }
}

pub fn segmented_tab_button(
    is_active: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        if is_active {
            button::Style {
                background: Some(Background::Color(ThemeColors::SURFACE_CARD)),
                text_color: ThemeColors::PRIMARY,
                border: Border {
                    color: ThemeColors::SLATE_200,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                },
                ..Default::default()
            }
        } else {
            match status {
                button::Status::Hovered => button::Style {
                    background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.6))),
                    text_color: ThemeColors::SLATE_800,
                    border: Border {
                        radius: 6.0.into(),
                        ..Default::default()
                    },
                    shadow: Shadow::default(),
                    ..Default::default()
                },
                button::Status::Pressed => button::Style {
                    background: Some(Background::Color(ThemeColors::SLATE_200)),
                    text_color: ThemeColors::SLATE_900,
                    border: Border {
                        radius: 6.0.into(),
                        ..Default::default()
                    },
                    shadow: Shadow::default(),
                    ..Default::default()
                },
                _ => button::Style {
                    background: None,
                    text_color: ThemeColors::SLATE_500,
                    border: Border {
                        radius: 6.0.into(),
                        ..Default::default()
                    },
                    shadow: Shadow::default(),
                    ..Default::default()
                },
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Input Styles
// -----------------------------------------------------------------------------

pub fn modern_input_style(_theme: &iced::Theme, status: text_input::Status) -> text_input::Style {
    match status {
        text_input::Status::Active => text_input::Style {
            background: Background::Color(ThemeColors::SURFACE_CARD),
            border: Border {
                color: ThemeColors::SLATE_200,
                width: 1.0,
                radius: 6.0.into(),
            },
            icon: ThemeColors::SLATE_400,
            placeholder: ThemeColors::SLATE_400,
            value: ThemeColors::SLATE_900,
            selection: ThemeColors::PRIMARY_LIGHT,
        },
        text_input::Status::Hovered => text_input::Style {
            background: Background::Color(ThemeColors::SURFACE_CARD),
            border: Border {
                color: ThemeColors::SLATE_300,
                width: 1.0,
                radius: 6.0.into(),
            },
            icon: ThemeColors::SLATE_500,
            placeholder: ThemeColors::SLATE_400,
            value: ThemeColors::SLATE_900,
            selection: ThemeColors::PRIMARY_LIGHT,
        },
        text_input::Status::Focused { .. } => text_input::Style {
            background: Background::Color(ThemeColors::SURFACE_CARD),
            border: Border {
                color: ThemeColors::PRIMARY,
                width: 1.5,
                radius: 6.0.into(),
            },
            icon: ThemeColors::PRIMARY,
            placeholder: ThemeColors::SLATE_400,
            value: ThemeColors::SLATE_900,
            selection: ThemeColors::PRIMARY_LIGHT,
        },
        text_input::Status::Disabled => text_input::Style {
            background: Background::Color(ThemeColors::SLATE_100),
            border: Border {
                color: ThemeColors::SLATE_200,
                width: 1.0,
                radius: 6.0.into(),
            },
            icon: ThemeColors::SLATE_400,
            placeholder: ThemeColors::SLATE_400,
            value: ThemeColors::SLATE_500,
            selection: ThemeColors::PRIMARY_LIGHT,
        },
    }
}
