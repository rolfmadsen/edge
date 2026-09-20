use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Background, Border, Color, Element, Length};

use super::app::Message;
use super::theme::{card_container_style, secondary_button_style, ThemeColors};

pub const PROPERTIES_TITLE: &str = "Egenskaber";
pub const GUIDANCE_TITLE: &str = "Vejledning";

/// Ensartet header for Egenskaber-panelet med titel, type-badge og luk-knap.
pub fn panel_header<'a>(
    title: &'a str,
    badge_info: Option<(&'a str, Color, Color)>,
    on_close: Option<Message>,
) -> Element<'a, Message> {
    let mut header_row = row![
        text(title).size(14).color(ThemeColors::PRIMARY),
        Space::new().width(Length::Fill),
    ]
    .align_y(Alignment::Center)
    .spacing(6);

    if let Some((badge_text, bg, border_color)) = badge_info {
        let badge = container(text(badge_text).size(11).color(ThemeColors::SLATE_800))
            .style(move |_| container::Style {
                background: Some(Background::Color(bg)),
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .padding([2, 6]);
        header_row = header_row.push(badge);
    }

    if let Some(msg) = on_close {
        let close_btn = button(text("✕").size(11))
            .style(secondary_button_style)
            .on_press(msg)
            .padding([2, 5]);
        header_row = header_row.push(close_btn);
    }

    header_row.into()
}

/// Sektionstitel med ensartet typografi på tværs af inspectorer.
pub fn section_header<'a>(title: &'a str) -> Element<'a, Message> {
    text(title)
        .size(12)
        .color(ThemeColors::SLATE_700)
        .into()
}

/// Generisk container til højre sidepanel med fast bredde og COSMIC card styling.
pub fn panel_container<'a>(content: Element<'a, Message>) -> Element<'a, Message> {
    container(scrollable(content))
        .style(card_container_style)
        .padding(14)
        .width(Length::Fixed(290.0))
        .height(Length::Fill)
        .into()
}

/// Ensartet vejledningspanel vist når der ikke er noget valgt element.
pub fn guidance_panel<'a>(
    phase_subtitle: &'a str,
    tips: &[(&'a str, &'a str)],
) -> Element<'a, Message> {
    let header = row![
        text(format!("💡 {}", GUIDANCE_TITLE))
            .size(14)
            .color(ThemeColors::PRIMARY),
        Space::new().width(Length::Fill),
        container(text(phase_subtitle).size(10).color(ThemeColors::SLATE_600))
            .style(move |_| container::Style {
                background: Some(Background::Color(ThemeColors::SLATE_100)),
                border: Border {
                    color: ThemeColors::SLATE_200,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .padding([2, 6]),
    ]
    .align_y(Alignment::Center);

    let mut content = column![header].spacing(12);

    for (tip_title, tip_body) in tips {
        let tip_card = container(
            column![
                text(*tip_title).size(12).color(ThemeColors::SLATE_800),
                text(*tip_body).size(11).color(ThemeColors::SLATE_500),
            ]
            .spacing(2),
        )
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.97, 0.98, 0.99, 0.85))),
            border: Border {
                color: ThemeColors::SLATE_200,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .padding(8)
        .width(Length::Fill);

        content = content.push(tip_card);
    }

    panel_container(content.into())
}
