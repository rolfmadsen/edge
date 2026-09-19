use crate::features::concepts::Concept;
use crate::features::information_model::{
    is_lower_camel_case, InformationClass, InformationModel, Multiplicity, PrimitiveType,
};
use crate::ui::app::{ConceptOption, Message};
use crate::ui::theme::{
    card_container_style, danger_button_style, list_item_button, modern_input_style,
    pill_container_style, primary_button_style, secondary_button_style, ThemeColors,
};
use iced::widget::{
    button, column, container, pick_list, row, scrollable, text, text_input, Space,
};
use iced::{Alignment, Element, Length};
use uuid::Uuid;

pub fn view<'a>(
    info_model: &'a InformationModel,
    concepts: &'a [Concept],
    selected_class_id: Option<Uuid>,
    search_query: &'a str,
) -> Element<'a, Message> {
    let concept_options: Vec<ConceptOption> = concepts
        .iter()
        .map(|c| ConceptOption {
            id: c.id(),
            term: c.preferred_term().to_string(),
        })
        .collect();

    // 1. Venstre kolonne: Liste over klasser
    let class_count = info_model.classes().len();
    let search_filter = search_query.trim().to_lowercase();
    let filtered_classes: Vec<&InformationClass> = info_model
        .classes()
        .iter()
        .filter(|c| {
            if search_filter.is_empty() {
                true
            } else {
                c.name().to_lowercase().contains(&search_filter)
                    || c.description()
                        .map(|d| d.to_lowercase().contains(&search_filter))
                        .unwrap_or(false)
            }
        })
        .collect();

    let class_list_header = column![
        row![
            text("Informationsklasser")
                .size(16)
                .color(ThemeColors::SLATE_900),
            Space::new().width(Length::Fill),
            container(
                text(format!("{}", class_count))
                    .size(11)
                    .color(ThemeColors::PRIMARY)
            )
            .style(pill_container_style)
            .padding([2, 8]),
        ]
        .align_y(Alignment::Center),
        text_input("🔍 Søg klasser...", search_query)
            .style(modern_input_style)
            .on_input(Message::InformationClassSearchChanged)
            .padding(6),
        row![
            button(text("+ Ny Klasse").size(12))
                .style(primary_button_style)
                .on_press(Message::CreateInformationClass)
                .padding([6, 12]),
            pick_list(
                concept_options.clone(),
                None::<ConceptOption>,
                Message::CreateInformationClassFromConcept,
            )
            .placeholder("+ Fra Begreb...")
            .padding(5)
            .width(Length::Fill),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    ]
    .spacing(10);

    let mut class_items = column![].spacing(6);
    for class in filtered_classes {
        let is_selected = selected_class_id == Some(class.id());
        let class_id = class.id();
        let attr_count = class.attributes().len();
        let concept_count = class.concept_ids().len();

        let item_btn = button(
            column![
                row![
                    text(class.name()).size(14).color(if is_selected {
                        ThemeColors::PRIMARY
                    } else {
                        ThemeColors::SLATE_900
                    }),
                    Space::new().width(Length::Fill),
                    container(
                        text(format!("{} attr", attr_count))
                            .size(11)
                            .color(ThemeColors::SLATE_600)
                    )
                    .style(pill_container_style)
                    .padding([2, 6]),
                ]
                .align_y(Alignment::Center),
                if concept_count > 0 {
                    row![container(
                        text(format!("{} begreb(er)", concept_count))
                            .size(10)
                            .color(ThemeColors::ACCENT_GREEN)
                    )
                    .style(pill_container_style)
                    .padding([1, 5])]
                } else {
                    row![
                        container(text("selvstændig").size(10).color(ThemeColors::TEXT_MUTED))
                            .style(pill_container_style)
                            .padding([1, 5])
                    ]
                },
            ]
            .spacing(4),
        )
        .style(list_item_button(is_selected))
        .on_press(Message::SelectInformationClass(Some(class_id)))
        .padding([8, 12])
        .width(Length::Fill);

        class_items = class_items.push(item_btn);
    }

    let left_panel = container(
        column![
            class_list_header,
            scrollable(class_items).height(Length::Fill),
        ]
        .spacing(12)
        .height(Length::Fill),
    )
    .style(card_container_style)
    .padding(14)
    .width(Length::Fixed(320.0))
    .height(Length::Fill);

    // 2. Højre kolonne: Master-Detail Visning
    let right_panel: Element<'a, Message> = if let Some(class_id) = selected_class_id {
        if let Some(class) = info_model.get_class(class_id) {
            // Card 1: Klassedetaljer & Begrebssporing
            let mut linked_concept_badges = row![].spacing(6).align_y(Alignment::Center);
            for cid in class.concept_ids() {
                let concept_term = concepts
                    .iter()
                    .find(|c| c.id() == *cid)
                    .map(|c| c.preferred_term())
                    .unwrap_or("Ukendt Begreb");
                let badge = container(
                    row![
                        text(concept_term).size(12).color(ThemeColors::PRIMARY),
                        button(text("✕").size(10).color(ThemeColors::TEXT_MUTED))
                            .style(secondary_button_style)
                            .on_press(Message::RemoveConceptFromInformationClass(class_id, *cid))
                            .padding([2, 5]),
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center),
                )
                .style(pill_container_style)
                .padding([3, 8]);
                linked_concept_badges = linked_concept_badges.push(badge);
            }

            let unlinked_concept_options: Vec<ConceptOption> = concept_options
                .iter()
                .filter(|opt| !class.concept_ids().contains(&opt.id))
                .cloned()
                .collect();

            let class_card = container(
                column![
                    row![
                        text("Klasseoplysninger")
                            .size(16)
                            .color(ThemeColors::SLATE_900),
                        Space::new().width(Length::Fill),
                        button(text("🗑️ Slet Klasse").size(12))
                            .style(danger_button_style)
                            .on_press(Message::DeleteInformationClass(class_id))
                            .padding([5, 10]),
                    ]
                    .align_y(Alignment::Center),
                    row![
                        column![
                            text("Klassenavn (UML UpperCamelCase)")
                                .size(12)
                                .color(ThemeColors::SLATE_600),
                            text_input("F.eks. Person, Koeretoej...", class.name())
                                .style(modern_input_style)
                                .on_input(move |val| {
                                    Message::UpdateInformationClassName(class_id, val)
                                })
                                .padding(7),
                        ]
                        .spacing(4)
                        .width(Length::FillPortion(1)),
                        column![
                            text("Beskrivelse / Definition")
                                .size(12)
                                .color(ThemeColors::SLATE_600),
                            text_input(
                                "Beskriv informationsklassen...",
                                class.description().unwrap_or(""),
                            )
                            .style(modern_input_style)
                            .on_input(move |val| {
                                Message::UpdateInformationClassDescription(class_id, val)
                            })
                            .padding(7),
                        ]
                        .spacing(4)
                        .width(Length::FillPortion(2)),
                    ]
                    .spacing(12),
                    column![
                        text("Tilknyttede Begreber (M:N Sporbarhed):")
                            .size(12)
                            .color(ThemeColors::SLATE_600),
                        row![
                            linked_concept_badges,
                            if !unlinked_concept_options.is_empty() {
                                pick_list(
                                    unlinked_concept_options,
                                    None::<ConceptOption>,
                                    move |opt| Message::AddConceptToInformationClass(class_id, opt),
                                )
                                .placeholder("+ Knyt begreb...")
                                .padding(5)
                                .width(Length::Fixed(180.0))
                            } else {
                                pick_list(vec![], None::<ConceptOption>, move |opt| {
                                    Message::AddConceptToInformationClass(class_id, opt)
                                })
                                .placeholder("Alle begreber knyttet")
                                .padding(5)
                                .width(Length::Fixed(180.0))
                            },
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                    ]
                    .spacing(6),
                ]
                .spacing(12),
            )
            .style(card_container_style)
            .padding(16)
            .width(Length::Fill);

            // Card 2: Attributter
            let attr_count = class.attributes().len();
            let attr_header = row![
                text("Attributter").size(16).color(ThemeColors::SLATE_900),
                container(
                    text(format!("{}", attr_count))
                        .size(11)
                        .color(ThemeColors::PRIMARY)
                )
                .style(pill_container_style)
                .padding([2, 8]),
                Space::new().width(Length::Fill),
                button(text("+ Tilføj Attribut").size(12))
                    .style(primary_button_style)
                    .on_press(Message::AddAttributeToClass(class_id))
                    .padding([6, 12]),
            ]
            .align_y(Alignment::Center);

            let mut attr_rows = column![].spacing(8);

            if class.attributes().is_empty() {
                attr_rows = attr_rows.push(
                    container(
                        text("Ingen attributter defineret for denne klasse endnu. Klik på '+ Tilføj Attribut' ovenfor.")
                            .size(13)
                            .color(ThemeColors::TEXT_MUTED),
                    )
                    .padding([12, 0]),
                );
            } else {
                // Header row
                let col_headers = row![
                    text("Attributnavn (lowerCamelCase)")
                        .size(12)
                        .color(ThemeColors::TEXT_MUTED)
                        .width(Length::Fixed(200.0)),
                    text("FDA Primitiv Type")
                        .size(12)
                        .color(ThemeColors::TEXT_MUTED)
                        .width(Length::Fixed(160.0)),
                    text("Multiplicitet")
                        .size(12)
                        .color(ThemeColors::TEXT_MUTED)
                        .width(Length::Fixed(120.0)),
                    text("Knyttet Begreb (Sporing)")
                        .size(12)
                        .color(ThemeColors::TEXT_MUTED)
                        .width(Length::Fill),
                    text("Slet")
                        .size(12)
                        .color(ThemeColors::TEXT_MUTED)
                        .width(Length::Fixed(40.0)),
                ]
                .spacing(8)
                .align_y(Alignment::Center);

                attr_rows = attr_rows.push(col_headers);

                for attr in class.attributes() {
                    let attr_id = attr.id();
                    let is_valid_case = is_lower_camel_case(attr.name());

                    let name_col = column![
                        text_input("f.eks. fornavn...", attr.name())
                            .style(modern_input_style)
                            .on_input(move |val| {
                                Message::UpdateAttributeName(class_id, attr_id, val)
                            })
                            .padding(6),
                        if !is_valid_case && !attr.name().is_empty() {
                            text("Bør være lowerCamelCase jf. §6.3")
                                .size(10)
                                .color(ThemeColors::ACCENT_RED)
                        } else {
                            text("").size(0)
                        },
                    ]
                    .spacing(2)
                    .width(Length::Fixed(200.0));

                    let type_col =
                        pick_list(PrimitiveType::ALL, Some(attr.data_type()), move |dt| {
                            Message::UpdateAttributeType(class_id, attr_id, dt)
                        })
                        .padding(5)
                        .width(Length::Fixed(160.0));

                    let mult_col =
                        pick_list(Multiplicity::PRESETS, Some(attr.multiplicity()), move |m| {
                            Message::UpdateAttributeMultiplicity(class_id, attr_id, m)
                        })
                        .padding(5)
                        .width(Length::Fixed(120.0));

                    let mut attr_concept_row = row![].spacing(4).align_y(Alignment::Center);
                    for cid in attr.concept_ids() {
                        let c_term = concepts
                            .iter()
                            .find(|c| c.id() == *cid)
                            .map(|c| c.preferred_term())
                            .unwrap_or("Begreb");
                        let badge = container(
                            row![
                                text(c_term).size(11).color(ThemeColors::ACCENT_GREEN),
                                button(text("✕").size(9).color(ThemeColors::TEXT_MUTED))
                                    .style(secondary_button_style)
                                    .on_press(Message::RemoveConceptFromAttribute(
                                        class_id, attr_id, *cid,
                                    ))
                                    .padding([1, 4]),
                            ]
                            .spacing(2)
                            .align_y(Alignment::Center),
                        )
                        .style(pill_container_style)
                        .padding([2, 5]);
                        attr_concept_row = attr_concept_row.push(badge);
                    }

                    let unlinked_for_attr: Vec<ConceptOption> = concept_options
                        .iter()
                        .filter(|opt| !attr.concept_ids().contains(&opt.id))
                        .cloned()
                        .collect();

                    let concept_col = row![
                        attr_concept_row,
                        if !unlinked_for_attr.is_empty() {
                            pick_list(unlinked_for_attr, None::<ConceptOption>, move |opt| {
                                Message::AddConceptToAttribute(class_id, attr_id, opt)
                            })
                            .placeholder("+ Begreb...")
                            .padding(4)
                            .width(Length::Fixed(130.0))
                        } else {
                            pick_list(vec![], None::<ConceptOption>, move |opt| {
                                Message::AddConceptToAttribute(class_id, attr_id, opt)
                            })
                            .placeholder("Alle tilknyttet")
                            .padding(4)
                            .width(Length::Fixed(130.0))
                        }
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                    let del_col = button(text("🗑️").size(12))
                        .style(secondary_button_style)
                        .on_press(Message::DeleteAttribute(class_id, attr_id))
                        .padding([5, 8]);

                    let attr_row = row![name_col, type_col, mult_col, concept_col, del_col]
                        .spacing(8)
                        .align_y(Alignment::Center);

                    attr_rows = attr_rows.push(attr_row);
                }
            }

            let attributes_card = container(
                column![attr_header, scrollable(attr_rows).height(Length::Fill)]
                    .spacing(12)
                    .height(Length::Fill),
            )
            .style(card_container_style)
            .padding(16)
            .width(Length::Fill)
            .height(Length::Fill);

            column![class_card, attributes_card]
                .spacing(12)
                .height(Length::Fill)
                .into()
        } else {
            container(
                column![
                    text("🏛️").size(32),
                    text("Vælg en klasse fra listen til venstre.")
                        .size(14)
                        .color(ThemeColors::TEXT_MUTED),
                ]
                .spacing(8)
                .align_x(Alignment::Center),
            )
            .style(card_container_style)
            .padding(40)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
        }
    } else {
        container(
            column![
                text("🏛️").size(36),
                text("Informationsmodel (UML Klasser & Attributter)")
                    .size(18)
                    .color(ThemeColors::SLATE_900),
                text(
                    "Vælg en informationsklasse fra oversigten til venstre eller opret en ny med '+ Ny Klasse' eller '+ Fra Begreb...'.",
                )
                .size(13)
                .color(ThemeColors::TEXT_MUTED),
            ]
            .spacing(10)
            .align_x(Alignment::Center),
        )
        .style(card_container_style)
        .padding(40)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
    };

    row![left_panel, right_panel]
        .spacing(12)
        .height(Length::Fill)
        .into()
}
