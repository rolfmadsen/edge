use crate::features::concept_model::{NodeId, RelationKind};
use crate::features::concepts::Concept;
use crate::features::information_model::{
    is_lower_camel_case, ClassGraph, InformationClass, InformationModel, Multiplicity,
    PrimitiveType,
};
use crate::ui::app::{
    AttributeConceptOption, ConceptOption, Message, NodeOption, RelationDialogState,
};
use crate::ui::diagram_canvas::{render_uml_class_node, CanvasViewport, DiagramCanvas};
use crate::ui::theme::{
    card_container_style, danger_button_style, list_item_button, modern_input_style,
    pill_container_style, primary_button_style, secondary_button_style, ThemeColors,
};
use iced::widget::{
    button, checkbox, column, container, pick_list, row, scrollable, text, text_input, Space,
};
use iced::{Alignment, Color, Element, Length};
use uuid::Uuid;

#[allow(clippy::too_many_arguments)]
pub fn view<'a>(
    info_model: &'a InformationModel,
    class_graph: &'a ClassGraph,
    concepts: &'a [Concept],
    selected_class_id: Option<Uuid>,
    selected_node_id: Option<NodeId>,
    selected_edge: Option<(NodeId, NodeId)>,
    search_query: &'a str,
    viewport: CanvasViewport,
    snap_to_grid: bool,
    is_space_pressed: bool,
    relation_dialog: Option<&'a RelationDialogState>,
    show_left_sidebar: bool,
) -> Element<'a, Message> {
    let existing_class_names: std::collections::HashSet<String> = info_model
        .classes()
        .iter()
        .map(|c| c.name().trim().to_lowercase())
        .collect();

    let concept_options: Vec<ConceptOption> = concepts
        .iter()
        .filter(|c| !existing_class_names.contains(&c.preferred_term().trim().to_lowercase()))
        .map(|c| ConceptOption {
            id: c.id(),
            term: c.preferred_term().to_string(),
        })
        .collect();

    // ==========================================
    // 1. VENSTRE PALET (Repository Browser ~240px)
    // ==========================================
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

    let palette_header = column![
        row![
            text("Klasser").size(15).color(ThemeColors::SLATE_900),
            container(
                text(format!("{}", class_count))
                    .size(11)
                    .color(ThemeColors::PRIMARY)
            )
            .style(pill_container_style)
            .padding([2, 7]),
            Space::new().width(Length::Fill),
            button(text("+ Ny").size(11))
                .style(primary_button_style)
                .on_press(Message::CreateInformationClass)
                .padding([3, 7]),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
        pick_list(
            concept_options.clone(),
            None::<ConceptOption>,
            Message::CreateInformationClassFromConcept,
        )
        .placeholder("+ Fra begreb...")
        .padding(4)
        .width(Length::Fill),
        text_input("🔍 Søg klasser...", search_query)
            .style(modern_input_style)
            .on_input(Message::InformationClassSearchChanged)
            .padding(5),
    ]
    .spacing(8);

    let mut class_items = column![].spacing(4);
    for class in filtered_classes {
        let is_selected = selected_class_id == Some(class.id());
        let class_id = class.id();
        let is_on_canvas = class_graph.is_class_on_diagram(class_id);
        let attr_count = class.attributes().len();

        let action_controls: Element<'a, Message> = if is_on_canvas {
            container(text("✓").size(11).color(ThemeColors::ACCENT_GREEN))
                .style(pill_container_style)
                .padding([1, 5])
                .into()
        } else {
            row![
                button(text("+").size(11).color(ThemeColors::PRIMARY))
                    .style(secondary_button_style)
                    .on_press(Message::AddClassToDiagram(class_id))
                    .padding([2, 5]),
                button(text("🗑️").size(10))
                    .style(danger_button_style)
                    .on_press(Message::DeleteInformationClass(class_id))
                    .padding([2, 4]),
            ]
            .spacing(3)
            .align_y(Alignment::Center)
            .into()
        };

        let display_name = if class.name().trim().is_empty() {
            "NyKlasse"
        } else {
            class.name()
        };

        let is_abstract = class.is_abstract();
        let is_borrowed = !class.is_local();

        let mut name_text = text(display_name).size(13).color(if is_selected {
            ThemeColors::PRIMARY
        } else {
            ThemeColors::SLATE_900
        });
        if is_abstract {
            name_text = name_text.font(iced::Font {
                style: iced::font::Style::Italic,
                ..Default::default()
            });
        }

        let mut title_row = row![name_text].spacing(4).align_y(Alignment::Center);
        if is_borrowed {
            title_row = title_row.push(
                container(
                    text("Indlånt")
                        .size(9)
                        .color(Color::from_rgb(0.08, 0.35, 0.65)),
                )
                .style(|_theme: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(ThemeColors::FDA_BORROWED_BLUE)),
                    border: iced::Border {
                        radius: 3.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .padding([1, 4]),
            );
        }

        let item_btn = button(
            column![
                title_row,
                text(format!("{} attr", attr_count))
                    .size(10)
                    .color(ThemeColors::TEXT_MUTED),
            ]
            .width(Length::Fill),
        )
        .style(list_item_button(is_selected))
        .on_press(Message::SelectInformationClass(Some(class_id)))
        .width(Length::Fill)
        .padding([4, 6]);

        let item_row = container(
            row![item_btn, action_controls]
                .align_y(Alignment::Center)
                .spacing(4),
        )
        .width(Length::Fill);

        class_items = class_items.push(item_row);
    }

    let left_palette = container(
        column![palette_header, scrollable(class_items).height(Length::Fill),]
            .spacing(10)
            .height(Length::Fill),
    )
    .style(card_container_style)
    .padding(12)
    .width(Length::Fixed(240.0))
    .height(Length::Fill);

    // ==========================================
    // 2. CENTER CANVAS (UML Klassediagram)
    // ==========================================
    let toolbar = row![
        button(text("+ Opret klasse").size(11))
            .style(primary_button_style)
            .on_press(Message::CreateInformationClassAtCenter)
            .padding([4, 10]),
        Space::new().width(4),
        button(text("+ Opret Relation").size(11))
            .style(primary_button_style)
            .on_press(Message::OpenInfoRelationDialog)
            .padding([4, 10]),
        Space::new().width(6),
        button(
            text(if snap_to_grid {
                "Snap: Til"
            } else {
                "Snap: Fra"
            })
            .size(11)
        )
        .style(if snap_to_grid {
            primary_button_style
        } else {
            secondary_button_style
        })
        .on_press(Message::ToggleInfoSnapToGrid)
        .padding([4, 10]),
        Space::new().width(Length::Fill),
        text(format!(
            "{} klasser på diagram • {} relationer",
            class_graph.node_count(),
            class_graph.edge_count()
        ))
        .size(11)
        .color(ThemeColors::TEXT_MUTED),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    let canvas_widget = iced::widget::canvas(
        DiagramCanvas::new(
            class_graph.nodes(),
            class_graph.edges(),
            selected_node_id,
            viewport,
            snap_to_grid,
            is_space_pressed,
            |frame, node, is_selected, vp| {
                let class_opt = info_model.get_class(node.class_id());
                let class_name = if class_opt.map(|c| c.name()).unwrap_or("").trim().is_empty() {
                    "NyKlasse"
                } else {
                    class_opt.map(|c| c.name()).unwrap()
                };
                let is_borrowed = class_opt.map(|c| !c.is_local()).unwrap_or(false);
                let is_abstract = class_opt.map(|c| c.is_abstract()).unwrap_or(false);
                let attributes: Vec<(String, String, String, bool)> = class_opt
                    .map(|c| {
                        c.attributes()
                            .iter()
                            .map(|a| {
                                let name = if a.name().trim().is_empty() {
                                    "nyAttribut"
                                } else {
                                    a.name()
                                };
                                (
                                    name.to_string(),
                                    a.data_type().as_str().to_string(),
                                    a.multiplicity().to_string(),
                                    !a.concept_ids().is_empty(),
                                )
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                render_uml_class_node(
                    frame,
                    node,
                    class_name,
                    &attributes,
                    is_borrowed,
                    is_abstract,
                    is_selected,
                    vp,
                );
            },
            Message::SelectInfoGraphNode,
            Message::UpdateClassNodePosition,
            Message::CreateInformationClassAt,
            |node_id| Message::SelectInfoGraphNode(Some(node_id)),
            Message::InfoCanvasViewportChanged,
        )
        .selected_edge(selected_edge)
        .on_edge_selected(Message::InfoEdgeSelected)
        .on_edge_created(Message::InfoEdgeCreated),
    )
    .width(Length::Fill)
    .height(Length::Fill);

    // Relation modal dialog if active
    let center_content: Element<'a, Message> = if let Some(dialog) = relation_dialog {
        let node_options: Vec<NodeOption> = class_graph
            .nodes()
            .iter()
            .map(|n| {
                let name = info_model
                    .get_class(n.class_id())
                    .map(|c| c.name().to_string())
                    .unwrap_or_else(|| "Klasse".to_string());
                NodeOption {
                    id: n.id(),
                    label: name,
                }
            })
            .collect();

        let mut dialog_content = column![
            row![
                text("Opret Relation mellem Klasser")
                    .size(14)
                    .color(ThemeColors::PRIMARY),
                Space::new().width(Length::Fill),
                button(text("✕").size(12))
                    .style(secondary_button_style)
                    .on_press(Message::CloseInfoRelationDialog)
                    .padding([2, 6]),
            ]
            .align_y(Alignment::Center),
            row![
                column![
                    text("Fra klasse:").size(11).color(ThemeColors::SLATE_600),
                    pick_list(
                        node_options.clone(),
                        dialog.from_node.clone(),
                        Message::InfoRelationFromChanged,
                    )
                    .padding(5)
                    .width(Length::Fixed(180.0)),
                ]
                .spacing(4),
                column![
                    text("Til klasse:").size(11).color(ThemeColors::SLATE_600),
                    pick_list(
                        node_options,
                        dialog.to_node.clone(),
                        Message::InfoRelationToChanged,
                    )
                    .padding(5)
                    .width(Length::Fixed(180.0)),
                ]
                .spacing(4),
            ]
            .spacing(10),
            row![
                column![
                    text("Type:").size(11).color(ThemeColors::SLATE_600),
                    pick_list(
                        RelationKind::ALL,
                        Some(dialog.kind),
                        Message::InfoRelationKindChanged,
                    )
                    .padding(5)
                    .width(Length::Fixed(180.0)),
                ]
                .spacing(4),
                column![
                    text("Label (f.eks. rolle):")
                        .size(11)
                        .color(ThemeColors::SLATE_600),
                    text_input("Valgfri rolle...", &dialog.label)
                        .style(modern_input_style)
                        .on_input(Message::InfoRelationLabelChanged)
                        .padding(5)
                        .width(Length::Fixed(180.0)),
                ]
                .spacing(4),
            ]
            .spacing(10),
        ]
        .spacing(8);

        if dialog.kind != RelationKind::Generalization {
            dialog_content = dialog_content.push(
                row![
                    column![
                        text("Kildemultiplicitet:")
                            .size(11)
                            .color(ThemeColors::SLATE_600),
                        pick_list(Multiplicity::PRESETS, dialog.source_multiplicity, |m| {
                            Message::InfoRelationSourceMultiplicityChanged(Some(m))
                        },)
                        .placeholder("Vælg...")
                        .padding(5)
                        .width(Length::Fixed(180.0)),
                    ]
                    .spacing(4),
                    column![
                        text("Målmultiplicitet:")
                            .size(11)
                            .color(ThemeColors::SLATE_600),
                        pick_list(Multiplicity::PRESETS, dialog.target_multiplicity, |m| {
                            Message::InfoRelationTargetMultiplicityChanged(Some(m))
                        },)
                        .placeholder("Vælg...")
                        .padding(5)
                        .width(Length::Fixed(180.0)),
                    ]
                    .spacing(4),
                ]
                .spacing(10),
            );
        }

        if let Some(err) = &dialog.error {
            dialog_content = dialog_content.push(text(err).size(11).color(ThemeColors::ACCENT_RED));
        }

        dialog_content = dialog_content.push(
            row![
                Space::new().width(Length::Fill),
                button(text("Annuller").size(12))
                    .style(secondary_button_style)
                    .on_press(Message::CloseInfoRelationDialog)
                    .padding([4, 10]),
                button(text("Opret Relation").size(12))
                    .style(primary_button_style)
                    .on_press(Message::InfoCreateRelation)
                    .padding([4, 12]),
            ]
            .spacing(8),
        );

        let dialog_card = container(dialog_content)
            .style(card_container_style)
            .padding(14)
            .width(Length::Fixed(400.0));

        column![
            toolbar,
            dialog_card,
            container(canvas_widget)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(
                        0.985, 0.988, 0.992
                    ))),
                    border: iced::Border {
                        color: ThemeColors::SLATE_200,
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    ..Default::default()
                })
                .width(Length::Fill)
                .height(Length::Fill),
        ]
        .spacing(8)
        .height(Length::Fill)
        .into()
    } else {
        column![
            toolbar,
            container(canvas_widget)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(
                        0.985, 0.988, 0.992
                    ))),
                    border: iced::Border {
                        color: ThemeColors::SLATE_200,
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    ..Default::default()
                })
                .width(Length::Fill)
                .height(Length::Fill),
        ]
        .spacing(8)
        .height(Length::Fill)
        .into()
    };

    let center_area = container(center_content)
        .width(Length::Fill)
        .height(Length::Fill);

    // ==========================================
    // 3. HØJRE INSPECTOR (Context Panel ~300px)
    // ==========================================
    let right_inspector: Element<'a, Message> = if let Some((from_id, to_id)) = selected_edge {
        if let Some(edge) = class_graph.find_edge(from_id, to_id) {
            let from_class = class_graph
                .find_node(from_id)
                .and_then(|n| info_model.get_class(n.class_id()))
                .map(|c| c.name())
                .unwrap_or("Kilde");
            let to_class = class_graph
                .find_node(to_id)
                .and_then(|n| info_model.get_class(n.class_id()))
                .map(|c| c.name())
                .unwrap_or("Mål");

            let header = crate::ui::inspector_panel::panel_header(
                crate::ui::inspector_panel::PROPERTIES_TITLE,
                Some(("Relation", ThemeColors::SLATE_100, ThemeColors::SLATE_300)),
                Some(Message::InfoEdgeSelected(None)),
            );

            let nodes_info = column![
                crate::ui::inspector_panel::section_header("Forbindelse"),
                row![
                    text(format!("{} ➔ {}", from_class, to_class))
                        .size(13)
                        .color(ThemeColors::SLATE_900),
                    Space::new().width(Length::Fill),
                    button(text("⇄ Vend").size(11))
                        .style(secondary_button_style)
                        .on_press(Message::InfoReverseEdge(from_id, to_id))
                        .padding([2, 6]),
                ]
                .align_y(Alignment::Center),
            ]
            .spacing(4);

            let kind_selector = column![
                crate::ui::inspector_panel::section_header("Relationstype"),
                row![
                    button(text("Association").size(11))
                        .style(if edge.kind() == RelationKind::Association {
                            primary_button_style
                        } else {
                            secondary_button_style
                        })
                        .on_press(Message::InfoUpdateEdgeKind(
                            from_id,
                            to_id,
                            RelationKind::Association
                        ))
                        .padding([4, 6]),
                    button(text("Generalisering").size(11))
                        .style(if edge.kind() == RelationKind::Generalization {
                            primary_button_style
                        } else {
                            secondary_button_style
                        })
                        .on_press(Message::InfoUpdateEdgeKind(
                            from_id,
                            to_id,
                            RelationKind::Generalization
                        ))
                        .padding([4, 6]),
                    button(text("Komposition").size(11))
                        .style(if edge.kind() == RelationKind::Composition {
                            primary_button_style
                        } else {
                            secondary_button_style
                        })
                        .on_press(Message::InfoUpdateEdgeKind(
                            from_id,
                            to_id,
                            RelationKind::Composition
                        ))
                        .padding([4, 6]),
                ]
                .spacing(4),
            ]
            .spacing(4);

            let directed_selector: Element<'a, Message> = if edge.kind()
                == RelationKind::Association
            {
                column![
                    crate::ui::inspector_panel::section_header("Retning / Navigabilitet"),
                    checkbox(edge.is_directed())
                        .label("Halv pil (rettet)")
                        .size(14)
                        .on_toggle(move |val| Message::InfoToggleEdgeDirected(from_id, to_id, val)),
                ]
                .spacing(4)
                .into()
            } else {
                Space::new().height(0).into()
            };

            let multiplicity_selector: Element<'a, Message> =
                if edge.kind() != RelationKind::Generalization {
                    column![
                        crate::ui::inspector_panel::section_header("Multipliciteter (FDA §6)"),
                        row![
                            column![
                                text(format!("Kilde ({})", from_class))
                                    .size(11)
                                    .color(ThemeColors::SLATE_600),
                                pick_list(
                                    Multiplicity::PRESETS,
                                    edge.source_multiplicity(),
                                    move |m| Message::InfoUpdateEdgeSourceMultiplicity(
                                        from_id,
                                        to_id,
                                        Some(m)
                                    ),
                                )
                                .placeholder("Vælg...")
                                .text_size(11.0)
                                .padding([3, 6])
                                .width(Length::FillPortion(1)),
                            ]
                            .spacing(2)
                            .width(Length::FillPortion(1)),
                            column![
                                text(format!("Mål ({})", to_class))
                                    .size(11)
                                    .color(ThemeColors::SLATE_600),
                                pick_list(
                                    Multiplicity::PRESETS,
                                    edge.target_multiplicity(),
                                    move |m| Message::InfoUpdateEdgeTargetMultiplicity(
                                        from_id,
                                        to_id,
                                        Some(m)
                                    ),
                                )
                                .placeholder("Vælg...")
                                .text_size(11.0)
                                .padding([3, 6])
                                .width(Length::FillPortion(1)),
                            ]
                            .spacing(2)
                            .width(Length::FillPortion(1)),
                        ]
                        .spacing(8),
                    ]
                    .spacing(4)
                    .into()
                } else {
                    Space::new().height(0).into()
                };

            let label_input = column![
                crate::ui::inspector_panel::section_header("Associationsnavn (valgfri)"),
                text_input("f.eks. omfatter, ejer...", edge.label().unwrap_or(""))
                    .id("info_edge_label_input")
                    .style(modern_input_style)
                    .on_input(move |v| Message::InfoUpdateEdgeLabel(from_id, to_id, v))
                    .padding(6)
                    .width(Length::Fill),
            ]
            .spacing(4);

            let actions = row![button(text("🗑️ Slet relation").size(11))
                .style(danger_button_style)
                .on_press(Message::DeleteClassRelation(from_id, to_id))
                .padding([4, 10]),]
            .align_y(Alignment::Center);

            let insp_col = column![
                header,
                nodes_info,
                kind_selector,
                directed_selector,
                multiplicity_selector,
                label_input,
                actions
            ]
            .spacing(12);

            crate::ui::inspector_panel::panel_container(insp_col.into())
        } else {
            crate::ui::inspector_panel::panel_container(
                text("Relation ikke fundet")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED)
                    .into(),
            )
        }
    } else if let Some(class_id) = selected_class_id {
        if let Some(class) = info_model.get_class(class_id) {
            let is_on_canvas = class_graph.is_class_on_diagram(class_id);

            // Klassenavn & beskrivelse
            let name_input = text_input("Klassenavn...", class.name())
                .style(modern_input_style)
                .size(13.0)
                .on_input(move |s| Message::UpdateInformationClassName(class_id, s))
                .padding([4, 6]);

            let desc_input = text_input("Beskrivelse...", class.description().unwrap_or(""))
                .style(modern_input_style)
                .size(12.0)
                .on_input(move |s| Message::UpdateInformationClassDescription(class_id, s))
                .padding([4, 6]);

            let is_abstract = class.is_abstract();
            let is_local = class.is_local();

            let abstract_checkbox = checkbox(is_abstract)
                .label("Abstrakt klasse ({abstract})")
                .size(13)
                .on_toggle(move |val| Message::SetInformationClassAbstract(class_id, val));

            let type_selector = column![
                crate::ui::inspector_panel::section_header("Klassetype (FDA)"),
                row![
                    button(text("Lokal (Sand)").size(11))
                        .style(if is_local {
                            primary_button_style
                        } else {
                            secondary_button_style
                        })
                        .on_press(Message::SetInformationClassLocal(class_id, true))
                        .padding([3, 8]),
                    button(text("Indlånt (Blå)").size(11))
                        .style(if !is_local {
                            primary_button_style
                        } else {
                            secondary_button_style
                        })
                        .on_press(Message::SetInformationClassLocal(class_id, false))
                        .padding([3, 8]),
                ]
                .spacing(6),
            ]
            .spacing(4);

            let origin_section: Element<'a, Message> = if !is_local {
                column![
                    text("Oprindelsesmodel / Kilde URI:")
                        .size(11.0)
                        .color(ThemeColors::SLATE_600),
                    text_input(
                        "https://data.gov.dk/model/...",
                        class.origin_model().unwrap_or(""),
                    )
                    .style(modern_input_style)
                    .size(11.5)
                    .on_input(move |s| Message::SetInformationClassOriginModel(class_id, s))
                    .padding([3, 6]),
                ]
                .spacing(4)
                .into()
            } else {
                Space::new().height(0).into()
            };

            // Begrebssporing (FDA Traceability)
            let mut concept_badges_row = row![].spacing(4);
            for cid in class.concept_ids() {
                let term = concepts
                    .iter()
                    .find(|c| c.id() == *cid)
                    .map(|c| c.preferred_term())
                    .unwrap_or("Ukendt");
                let cid_val = *cid;
                let badge = container(
                    row![
                        text(term).size(11).color(ThemeColors::PRIMARY),
                        button(text("✕").size(9))
                            .style(secondary_button_style)
                            .on_press(Message::RemoveConceptFromInformationClass(
                                class_id, cid_val
                            ))
                            .padding([1, 4]),
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center),
                )
                .style(pill_container_style)
                .padding([2, 6]);

                concept_badges_row = concept_badges_row.push(badge);
            }
            let concept_badges: Element<'a, Message> = concept_badges_row.wrap().into();

            let available_concepts: Vec<ConceptOption> = concepts
                .iter()
                .filter(|c| !class.concept_ids().contains(&c.id()))
                .map(|c| ConceptOption {
                    id: c.id(),
                    term: c.preferred_term().to_string(),
                })
                .collect();

            let add_concept_picker =
                pick_list(available_concepts, None::<ConceptOption>, move |opt| {
                    Message::AddConceptToInformationClass(class_id, opt)
                })
                .placeholder("+ Knyt begreb...")
                .text_size(11.5)
                .padding([3, 6])
                .width(Length::Fill);

            // Attributter sektion
            let mut attr_list = column![].spacing(6);
            for attr in class.attributes() {
                let attr_id = attr.id();
                let name_val = attr.name().to_string();
                let type_val = attr.data_type();
                let mult_val = attr.multiplicity();

                let is_name_valid = is_lower_camel_case(&name_val);

                let linked_concept_id = attr.concept_ids().first().copied();
                let linked_concept =
                    linked_concept_id.and_then(|cid| concepts.iter().find(|c| c.id() == cid));

                let mut attr_concept_options = vec![AttributeConceptOption {
                    id: None,
                    label: "— Intet begreb (teknisk felt) —".to_string(),
                }];
                for c in concepts {
                    attr_concept_options.push(AttributeConceptOption {
                        id: Some(c.id()),
                        label: c.preferred_term().to_string(),
                    });
                }

                let selected_concept_opt = match linked_concept {
                    Some(c) => AttributeConceptOption {
                        id: Some(c.id()),
                        label: c.preferred_term().to_string(),
                    },
                    None => AttributeConceptOption {
                        id: None,
                        label: "— Intet begreb (teknisk felt) —".to_string(),
                    },
                };

                let mut top_row = row![text_input("attributNavn", &name_val)
                    .style(modern_input_style)
                    .size(12.0)
                    .on_input(move |s| { Message::UpdateAttributeName(class_id, attr_id, s) })
                    .padding([3, 6])
                    .width(Length::Fill),];

                if linked_concept.is_some() {
                    top_row = top_row.push(text("🔗").size(11).color(ThemeColors::PRIMARY));
                }

                top_row = top_row.push(
                    button(text("✕").size(10))
                        .style(danger_button_style)
                        .on_press(Message::DeleteAttribute(class_id, attr_id))
                        .padding([2, 5]),
                );

                let mut attr_col = column![
                    top_row.spacing(4).align_y(Alignment::Center),
                    row![
                        pick_list(PrimitiveType::ALL, Some(type_val), move |t| {
                            Message::UpdateAttributeType(class_id, attr_id, t)
                        },)
                        .text_size(11.0)
                        .padding([3, 6])
                        .width(Length::FillPortion(3)),
                        pick_list(Multiplicity::PRESETS, Some(mult_val), move |m| {
                            Message::UpdateAttributeMultiplicity(class_id, attr_id, m)
                        },)
                        .text_size(11.0)
                        .padding([3, 6])
                        .width(Length::FillPortion(2)),
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center),
                    row![
                        text("Begreb:").size(11.0).color(ThemeColors::SLATE_500),
                        pick_list(
                            attr_concept_options,
                            Some(selected_concept_opt),
                            move |opt| Message::SetAttributeConcept(class_id, attr_id, opt.id)
                        )
                        .text_size(11.0)
                        .padding([3, 6])
                        .width(Length::Fill),
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center),
                ]
                .spacing(4);

                if let Some(c) = linked_concept {
                    let badge = container(
                        row![
                            text("🔗").size(10).color(ThemeColors::PRIMARY),
                            text(format!("Lineage: {}", c.preferred_term()))
                                .size(10.5)
                                .color(ThemeColors::PRIMARY),
                        ]
                        .spacing(4)
                        .align_y(Alignment::Center),
                    )
                    .style(pill_container_style)
                    .padding([2, 6]);

                    attr_col = attr_col.push(badge);
                }

                if !is_name_valid && !name_val.is_empty() {
                    attr_col = attr_col.push(
                        text("⚠️ FDA §6.3: Bør være lowerCamelCase")
                            .size(10)
                            .color(Color::from_rgb(0.85, 0.55, 0.1)),
                    );
                }

                let attr_card = container(attr_col)
                    .style(card_container_style)
                    .padding(6)
                    .width(Length::Fill);

                attr_list = attr_list.push(attr_card);
            }

            // Canvas handlinger for denne klasse
            let canvas_action_btn = if is_on_canvas {
                if let Some(node) = class_graph.find_node_by_class(class_id) {
                    let node_id = node.id();
                    button(text("Fjern fra diagram").size(11))
                        .style(secondary_button_style)
                        .on_press(Message::RemoveClassFromDiagram(node_id))
                        .padding([4, 8])
                } else {
                    button(text("Tilføj til diagram").size(11))
                        .style(primary_button_style)
                        .on_press(Message::AddClassToDiagram(class_id))
                        .padding([4, 8])
                }
            } else {
                button(text("+ Tilføj til diagram").size(11))
                    .style(primary_button_style)
                    .on_press(Message::AddClassToDiagram(class_id))
                    .padding([4, 8])
            };

            // Tilknyttede relationer for denne klasse
            let connected_edges: Vec<_> =
                if let Some(node) = class_graph.find_node_by_class(class_id) {
                    let node_id = node.id();
                    class_graph
                        .edges()
                        .iter()
                        .filter(|e| e.from() == node_id || e.to() == node_id)
                        .collect()
                } else {
                    Vec::new()
                };

            let mut relations_col = column![text("Tilknyttede relationer:")
                .size(11)
                .color(ThemeColors::SLATE_700),]
            .spacing(4);

            if connected_edges.is_empty() {
                relations_col = relations_col.push(
                    text("(ingen relationer)")
                        .size(11)
                        .color(ThemeColors::TEXT_MUTED),
                );
            } else {
                for edge in connected_edges {
                    let from_class = class_graph
                        .find_node(edge.from())
                        .and_then(|n| info_model.get_class(n.class_id()))
                        .map(|c| c.name())
                        .unwrap_or("?");
                    let to_class = class_graph
                        .find_node(edge.to())
                        .and_then(|n| info_model.get_class(n.class_id()))
                        .map(|c| c.name())
                        .unwrap_or("?");

                    let desc = match edge.kind() {
                        RelationKind::Generalization => {
                            format!("{} ⮞ {}", from_class, to_class)
                        }
                        RelationKind::Association => {
                            if let Some(lbl) = edge.label() {
                                format!("{} ──({})── {}", from_class, lbl, to_class)
                            } else {
                                format!("{} ── {}", from_class, to_class)
                            }
                        }
                        RelationKind::Composition => {
                            format!("{} ◆── {}", from_class, to_class)
                        }
                    };

                    let edge_from = edge.from();
                    let edge_to = edge.to();
                    let edge_row = row![
                        text(desc)
                            .size(11)
                            .color(ThemeColors::SLATE_800)
                            .width(Length::Fill),
                        button(text("🗑️").size(11))
                            .style(danger_button_style)
                            .on_press(Message::DeleteClassRelation(edge_from, edge_to))
                            .padding([2, 5]),
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center);

                    relations_col = relations_col.push(edge_row);
                }
            }

            let badge_bg = if is_local {
                ThemeColors::FDA_SAND
            } else {
                ThemeColors::FDA_BORROWED_BLUE
            };
            let badge_border = if is_local {
                ThemeColors::FDA_SAND_BORDER
            } else {
                Color::from_rgb(0.35, 0.65, 0.85)
            };
            let badge_text = if is_local { "Klasse" } else { "Indlånt" };

            let header = crate::ui::inspector_panel::panel_header(
                crate::ui::inspector_panel::PROPERTIES_TITLE,
                Some((badge_text, badge_bg, badge_border)),
                Some(Message::SelectInformationClass(None)),
            );

            let actions_row = row![
                canvas_action_btn,
                Space::new().width(Length::Fill),
                button(text("🗑️ Slet klasse").size(11))
                    .style(danger_button_style)
                    .on_press(Message::DeleteInformationClass(class_id))
                    .padding([3, 7]),
            ]
            .align_y(Alignment::Center);

            let inspector_content = column![
                header,
                crate::ui::inspector_panel::section_header("Generelt"),
                name_input,
                desc_input,
                crate::ui::inspector_panel::section_header("Egenskaber (UML & FDA)"),
                abstract_checkbox,
                type_selector,
                origin_section,
                actions_row,
                // Begreber
                column![
                    crate::ui::inspector_panel::section_header("Tilknyttede Begreber"),
                    concept_badges,
                    add_concept_picker,
                ]
                .spacing(4),
                // Tilknyttede Relationer
                relations_col,
                // Attributter
                column![
                    row![
                        text(format!("Attributter ({})", class.attributes().len()))
                            .size(12)
                            .color(ThemeColors::SLATE_700),
                        Space::new().width(Length::Fill),
                        button(text("+ Tilføj").size(11))
                            .style(primary_button_style)
                            .on_press(Message::AddAttributeToClass(class_id))
                            .padding([3, 7]),
                    ]
                    .align_y(Alignment::Center),
                    attr_list,
                ]
                .spacing(6),
            ]
            .spacing(10);

            crate::ui::inspector_panel::panel_container(inspector_content.into())
        } else {
            crate::ui::inspector_panel::panel_container(
                text("Klasse ikke fundet")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED)
                    .into(),
            )
        }
    } else {
        crate::ui::inspector_panel::guidance_panel(
            "Informationsmodel",
            &[
                (
                    "Vælg eller opret klasse",
                    "Klik på en klasse på canvas eller brug [+ Ny Klasse] i venstre palet.",
                ),
                (
                    "Tilknyt begreber",
                    "Tilknyt begreber for at sikre 100% semantisk sporbarhed mod FDA begrebsmodellen.",
                ),
                (
                    "Attributter & Datatyper",
                    "Tilføj attributter med standard primitive typer (String, Integer m.fl.) og multipliciteter.",
                ),
                (
                    "Forbind klasser",
                    "Forbind klasser med associationer, generaliseringer eller kompositioner.",
                ),
                (
                    "Automatisk tilpasning",
                    "UML-kassen udvider automatisk sin højde i grid-intervaller, når attributter tilføjes.",
                ),
                (
                    "FDA Farvekoder",
                    "Sand (#FEFAF7) markerer forretningens egne klasser.",
                ),
            ],
        )
    };

    // ==========================================
    // SAMLET 3-DELT STUDIO LAYOUT
    // ==========================================
    let mut main_row = row![].spacing(10).height(Length::Fill);
    if show_left_sidebar {
        main_row = main_row.push(left_palette);
    }
    main_row.push(center_area).push(right_inspector).into()
}
