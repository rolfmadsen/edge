use crate::features::concepts::{BelongsToDomain, Concept};
use crate::ui::app::Message;
use crate::ui::theme::ThemeColors;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length};

pub fn view<'a>(
    concepts: Vec<&'a Concept>,
    total_count: usize,
    search_query: &'a str,
) -> Element<'a, Message> {
    // 1. Top bar: Søgning, statistik og "Opret"-knap
    let search_bar = text_input("🔍 Søg i termer, definitioner og kilder...", search_query)
        .on_input(Message::SearchQueryChanged)
        .padding(10)
        .width(Length::FillPortion(3));

    let count_badge = container(
        text(format!("{}/{} begreber", concepts.len(), total_count))
            .size(13)
            .color(ThemeColors::TEXT_MUTED),
    )
    .padding([6, 12])
    .align_y(Alignment::Center);

    let create_btn = button(
        row![
            text("+").size(16),
            Space::new().width(6),
            text("Nyt Begreb (Bilag D & E)").size(14),
        ]
        .align_y(Alignment::Center),
    )
    .on_press(Message::StartNewConcept)
    .padding([8, 16]);

    let top_bar = row![search_bar, count_badge, Space::new().width(10), create_btn]
        .spacing(12)
        .align_y(Alignment::Center);

    // 2. Tabelhoved
    let header_row = row![
        container(text("Foretrukken term").size(13).color(ThemeColors::TEXT_MUTED))
            .width(Length::FillPortion(2)),
        container(text("Definition (FDA Bilag D & E)").size(13).color(ThemeColors::TEXT_MUTED))
            .width(Length::FillPortion(4)),
        container(text("Kilder (Juridisk / Almen)").size(13).color(ThemeColors::TEXT_MUTED))
            .width(Length::FillPortion(2)),
        container(text("Emneområde (§26)").size(13).color(ThemeColors::TEXT_MUTED))
            .width(Length::FillPortion(2)),
        container(text("Handlinger").size(13).color(ThemeColors::TEXT_MUTED))
            .width(Length::FillPortion(2)),
    ]
    .spacing(12)
    .padding([8, 12]);

    let header_container = container(header_row)
        .width(Length::Fill);

    // 3. Rækker eller tom tilstand
    let content: Element<Message> = if concepts.is_empty() {
        let empty_msg = if !search_query.trim().is_empty() {
            "Ingen begreber matcher søgningen."
        } else {
            "Der er endnu ingen begreber i projektets begrebsliste."
        };

        container(
            column![
                text("📋").size(40),
                Space::new().height(8),
                text(empty_msg).size(16).color(ThemeColors::TEXT_DARK),
                Space::new().height(4),
                text("Opret dit første begreb med foretrukken term, definition, kilder og emneområde jf. FDA Modelreglerne.")
                    .size(13)
                    .color(ThemeColors::TEXT_MUTED),
                Space::new().height(16),
                button(text("+ Opret Første Begreb").size(14))
                    .on_press(Message::StartNewConcept)
                    .padding([8, 18]),
            ]
            .spacing(6)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    } else {
        let mut rows = column![].spacing(8);

        for concept in concepts {
            let id = concept.id();

            // Foretrukken term kolonne
            let term_content = column![
                text(concept.preferred_term()).size(14),
                if let Some(uri) = concept.identifier() {
                    text(uri).size(11).color(ThemeColors::TEXT_MUTED)
                } else {
                    text("").size(0)
                },
            ]
            .spacing(2);

            // Definition kolonne
            let def_content = text(concept.definition()).size(13);

            // Kilde kolonne
            let sources_content = column![
                if let Some(legal) = concept.legal_source() {
                    text(format!("§ {}", legal)).size(11).color(ThemeColors::PRIMARY)
                } else {
                    text("").size(0)
                },
                if let Some(src) = concept.source() {
                    text(src).size(11).color(ThemeColors::TEXT_MUTED)
                } else if concept.legal_source().is_none() {
                    text("Ingen kilde angivet").size(11).color(ThemeColors::TEXT_MUTED)
                } else {
                    text("").size(0)
                },
            ]
            .spacing(2);

            // Emneområde kolonne - FDA Anbefalede Farver jf. Kapitel 7.3
            let domain_badge = match concept.belongs_to_domain() {
                BelongsToDomain::Yes => container(
                    text("Lokalt begreb")
                        .size(11)
                        .color(ThemeColors::TEXT_DARK),
                )
                .padding([3, 8]),
                BelongsToDomain::No => container(
                    text("Indlånt / Ekstern")
                        .size(11)
                        .color(ThemeColors::PRIMARY),
                )
                .padding([3, 8]),
                BelongsToDomain::ModelRef(model_uri) => container(
                    text(format!("Indlånt: {}", model_uri))
                        .size(11)
                        .color(ThemeColors::PRIMARY),
                )
                .padding([3, 8]),
            };

            // Handlinger kolonne
            let actions = row![
                button(text("Rediger").size(12)).on_press(Message::EditConcept(id)).padding([4, 8]),
                button(text("Slet").size(12)).on_press(Message::DeleteConcept(id)).padding([4, 8]),
            ]
            .spacing(6)
            .align_y(Alignment::Center);

            let row_card = container(
                row![
                    container(term_content).width(Length::FillPortion(2)),
                    container(def_content).width(Length::FillPortion(4)),
                    container(sources_content).width(Length::FillPortion(2)),
                    container(domain_badge).width(Length::FillPortion(2)),
                    container(actions).width(Length::FillPortion(2)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .padding([12, 14])
            .width(Length::Fill);

            rows = rows.push(row_card);
        }

        scrollable(rows).height(Length::Fill).into()
    };

    column![top_bar, Space::new().height(8), header_container, content]
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
