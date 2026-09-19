use crate::features::concept_model::{ConceptGraph, NodeId, RelationKind};
use crate::features::concepts::Concept;
use crate::ui::app::{Message, NodeOption, RelationDialogState};
use crate::ui::concept_editor::{ConceptEditorState, ConceptFormField};
use crate::ui::diagram_canvas::{render_concept_node, CanvasViewport, DiagramCanvas};
use crate::ui::theme::{
    card_container_style, danger_button_style, list_item_button, modern_input_style,
    pill_container_style, primary_button_style, secondary_button_style, ThemeColors,
};
use iced::widget::{
    button, checkbox, column, container, pick_list, row, scrollable, text, text_input, Space,
};
use iced::{Alignment, Color, Element, Length};

#[allow(clippy::too_many_arguments)]
pub fn view<'a>(
    concepts: &'a [Concept],
    concept_graph: &'a ConceptGraph,
    selected_node_id: Option<NodeId>,
    selected_edge: Option<(NodeId, NodeId)>,
    search_query: &'a str,
    viewport: CanvasViewport,
    snap_to_grid: bool,
    is_space_pressed: bool,
    is_inline_editing: bool,
    editor_state: Option<&'a ConceptEditorState>,
    relation_dialog: Option<&'a RelationDialogState>,
) -> Element<'a, Message> {
    // ==========================================
    // 1. VENSTRE PALET (Repository Browser ~240px)
    // ==========================================
    let concept_count = concepts.len();
    let search_filter = search_query.trim().to_lowercase();
    let filtered_concepts: Vec<&Concept> = concepts
        .iter()
        .filter(|c| {
            if search_filter.is_empty() {
                true
            } else {
                c.preferred_term().to_lowercase().contains(&search_filter)
                    || c.definition().to_lowercase().contains(&search_filter)
            }
        })
        .collect();

    let palette_header = column![
        row![
            text("Begreber").size(15).color(ThemeColors::SLATE_900),
            Space::new().width(Length::Fill),
            container(
                text(format!("{}", concept_count))
                    .size(11)
                    .color(ThemeColors::PRIMARY)
            )
            .style(pill_container_style)
            .padding([2, 7]),
        ]
        .align_y(Alignment::Center),
        text_input("🔍 Søg begreber...", search_query)
            .style(modern_input_style)
            .on_input(Message::ConceptModelSearchChanged)
            .padding(5),
        button(text("+ Nyt begreb").size(12))
            .style(primary_button_style)
            .on_press(Message::StartNewConcept)
            .padding([4, 10]),
    ]
    .spacing(8);

    let mut concept_items = column![].spacing(4);
    for concept in filtered_concepts {
        let concept_id = concept.id();
        let is_on_canvas = concept_graph.is_concept_on_diagram(concept_id);
        let node_id = concept_graph
            .find_node_by_concept(concept_id)
            .map(|n| n.id());
        let is_selected = selected_node_id.is_some() && selected_node_id == node_id;

        let status_badge: Element<'a, Message> = if is_on_canvas {
            container(text("✓").size(11).color(ThemeColors::ACCENT_GREEN))
                .style(pill_container_style)
                .padding([1, 5])
                .into()
        } else {
            button(text("+").size(11).color(ThemeColors::PRIMARY))
                .style(secondary_button_style)
                .on_press(Message::AddConceptToDiagram(concept_id))
                .padding([1, 5])
                .into()
        };

        let is_local = concept.belongs_to_domain().is_local();
        let subtitle = if is_local { "Lokalt" } else { "Indlånt" };

        let item_btn = button(
            row![
                column![
                    text(concept.preferred_term())
                        .size(13)
                        .color(if is_selected {
                            ThemeColors::PRIMARY
                        } else {
                            ThemeColors::SLATE_900
                        }),
                    text(subtitle).size(10).color(ThemeColors::TEXT_MUTED),
                ]
                .width(Length::Fill),
                status_badge,
            ]
            .align_y(Alignment::Center)
            .spacing(6),
        )
        .style(list_item_button(is_selected))
        .on_press(if let Some(nid) = node_id {
            Message::GraphNodeSelected(Some(nid))
        } else {
            Message::AddConceptToDiagram(concept_id)
        })
        .width(Length::Fill)
        .padding([6, 8]);

        concept_items = concept_items.push(item_btn);
    }

    let left_palette = container(
        column![
            palette_header,
            scrollable(concept_items).height(Length::Fill),
        ]
        .spacing(10)
        .height(Length::Fill),
    )
    .style(card_container_style)
    .padding(12)
    .width(Length::Fixed(230.0))
    .height(Length::Fill);

    // ==========================================
    // 2. CENTER CANVAS (Begrebsmodel Graf)
    // ==========================================
    let zoom_pct = (viewport.zoom() * 100.0).round() as u32;
    let toolbar = row![
        button(text("-").size(13))
            .style(secondary_button_style)
            .on_press(Message::CanvasZoomOut)
            .padding([4, 8]),
        button(text(format!("{}%", zoom_pct)).size(11))
            .style(secondary_button_style)
            .on_press(Message::CanvasResetView)
            .padding([4, 6]),
        button(text("+").size(13))
            .style(secondary_button_style)
            .on_press(Message::CanvasZoomIn)
            .padding([4, 8]),
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
        .on_press(Message::ToggleSnapToGrid)
        .padding([4, 8]),
        Space::new().width(6),
        button(text("+ Opret Relation").size(11))
            .style(primary_button_style)
            .on_press(Message::GraphOpenRelationDialog)
            .padding([4, 10]),
        Space::new().width(Length::Fill),
        text(format!(
            "{} noder på diagram • {} relationer",
            concept_graph.node_count(),
            concept_graph.edge_count()
        ))
        .size(11)
        .color(ThemeColors::TEXT_MUTED),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let canvas_widget = iced::widget::canvas(
        DiagramCanvas::new(
            concept_graph.nodes(),
            concept_graph.edges(),
            selected_node_id,
            viewport,
            snap_to_grid,
            is_space_pressed,
            render_concept_node,
            Message::GraphNodeSelected,
            Message::GraphNodeMoved,
            Message::CanvasDoubleClicked,
            Message::GraphNodeDoubleClicked,
            Message::CanvasViewportChanged,
        )
        .selected_edge(selected_edge)
        .on_edge_selected(Message::GraphEdgeSelected)
        .on_edge_created(Message::GraphEdgeCreated),
    )
    .width(Length::Fill)
    .height(Length::Fill);

    // Relations-dialog modal hvis aktiv
    let center_content: Element<'a, Message> = if let Some(dialog) = relation_dialog {
        let node_options: Vec<NodeOption> = concept_graph
            .nodes()
            .iter()
            .map(|n| NodeOption {
                id: n.id(),
                label: n.label().to_string(),
            })
            .collect();

        let mut dialog_content = column![
            row![
                text("Opret Relation mellem Begreber")
                    .size(14)
                    .color(ThemeColors::PRIMARY),
                Space::new().width(Length::Fill),
                button(text("✕").size(12))
                    .style(secondary_button_style)
                    .on_press(Message::GraphCloseRelationDialog)
                    .padding([2, 6]),
            ]
            .align_y(Alignment::Center),
            row![
                column![
                    text("Fra begreb:").size(11).color(ThemeColors::SLATE_600),
                    pick_list(
                        node_options.clone(),
                        dialog.from_node.clone(),
                        Message::GraphRelationFromChanged,
                    )
                    .placeholder("Vælg kilde...")
                    .padding(5)
                    .width(Length::Fixed(180.0)),
                ]
                .spacing(3),
                column![
                    text("Relationstype:")
                        .size(11)
                        .color(ThemeColors::SLATE_600),
                    pick_list(
                        RelationKind::ALL,
                        Some(dialog.kind),
                        Message::GraphRelationKindChanged,
                    )
                    .padding(5)
                    .width(Length::Fixed(180.0)),
                ]
                .spacing(3),
                column![
                    text("Til begreb:").size(11).color(ThemeColors::SLATE_600),
                    pick_list(
                        node_options,
                        dialog.to_node.clone(),
                        Message::GraphRelationToChanged,
                    )
                    .placeholder("Vælg mål...")
                    .padding(5)
                    .width(Length::Fixed(180.0)),
                ]
                .spacing(3),
            ]
            .spacing(10),
            column![
                text("Rolle / Navn (valgfri):")
                    .size(11)
                    .color(ThemeColors::SLATE_600),
                text_input("f.eks. ejer, omfatter, udsteder...", &dialog.label)
                    .style(modern_input_style)
                    .on_input(Message::GraphRelationLabelChanged)
                    .padding(5),
            ]
            .spacing(3),
            row![
                Space::new().width(Length::Fill),
                button(text("Annuller").size(12))
                    .style(secondary_button_style)
                    .on_press(Message::GraphCloseRelationDialog)
                    .padding([4, 10]),
                button(text("Opret Relation").size(12))
                    .style(primary_button_style)
                    .on_press(Message::GraphCreateRelation)
                    .padding([4, 12]),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(10);

        if let Some(err) = &dialog.error {
            dialog_content = dialog_content.push(
                container(
                    text(format!("⚠️ {}", err))
                        .size(11)
                        .color(ThemeColors::ACCENT_RED),
                )
                .style(card_container_style)
                .padding(6),
            );
        }

        let dialog_card = container(dialog_content)
            .style(card_container_style)
            .padding(14)
            .width(Length::Fixed(590.0));

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
            container(dialog_card).center_x(Length::Fill),
        ]
        .spacing(8)
        .width(Length::Fill)
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
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    };

    // ==========================================
    // 3. HØJRE INSPECTOR (Context Panel ~290px)
    // ==========================================
    let right_inspector: Element<'a, Message> = if let Some((from_id, to_id)) = selected_edge {
        if let Some(edge) = concept_graph.find_edge(from_id, to_id) {
            let from_node = concept_graph.find_node(from_id);
            let to_node = concept_graph.find_node(to_id);
            let from_name = from_node.map(|n| n.label()).unwrap_or("Kilde");
            let to_name = to_node.map(|n| n.label()).unwrap_or("Mål");

            let header = row![
                text("Relation").size(14).color(ThemeColors::PRIMARY),
                Space::new().width(Length::Fill),
                button(text("✕").size(11))
                    .style(secondary_button_style)
                    .on_press(Message::GraphEdgeSelected(None))
                    .padding([2, 5]),
            ]
            .align_y(Alignment::Center);

            let nodes_info = column![
                row![
                    text(format!("{} ➔ {}", from_name, to_name))
                        .size(13)
                        .color(ThemeColors::SLATE_900),
                    Space::new().width(Length::Fill),
                    button(text("⇄ Vend").size(11))
                        .style(secondary_button_style)
                        .on_press(Message::GraphReverseEdge(from_id, to_id))
                        .padding([2, 6]),
                ]
                .align_y(Alignment::Center),
                text("Rediger relationens egenskaber:")
                    .size(11)
                    .color(ThemeColors::TEXT_MUTED),
            ]
            .spacing(4);

            let kind_selector = column![
                text("Relationstype:")
                    .size(11)
                    .color(ThemeColors::SLATE_600),
                row![
                    button(text("Association").size(11))
                        .style(if edge.kind() == RelationKind::Association {
                            primary_button_style
                        } else {
                            secondary_button_style
                        })
                        .on_press(Message::GraphUpdateEdgeKind(
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
                        .on_press(Message::GraphUpdateEdgeKind(
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
                        .on_press(Message::GraphUpdateEdgeKind(
                            from_id,
                            to_id,
                            RelationKind::Composition
                        ))
                        .padding([4, 6]),
                ]
                .spacing(4),
            ]
            .spacing(4);

            let directed_selector: Element<'a, Message> =
                if edge.kind() == RelationKind::Association {
                    column![
                        text("Retning / Navigabilitet:")
                            .size(11)
                            .color(ThemeColors::SLATE_600),
                        checkbox(edge.is_directed())
                            .label("Halv pil (rettet)")
                            .size(14)
                            .on_toggle(move |val| Message::GraphToggleEdgeDirected(
                                from_id, to_id, val
                            )),
                    ]
                    .spacing(4)
                    .into()
                } else {
                    Space::new().height(0).into()
                };

            let label_input = column![
                text("Associationsnavn (valgfri):")
                    .size(11)
                    .color(ThemeColors::SLATE_600),
                text_input("f.eks. omfatter, ejer...", edge.label().unwrap_or(""))
                    .id("edge_label_input")
                    .style(modern_input_style)
                    .on_input(move |v| Message::GraphUpdateEdgeLabel(from_id, to_id, v))
                    .padding(6)
                    .width(Length::Fill),
            ]
            .spacing(4);

            let actions = row![button(text("🗑️ Slet relation").size(11))
                .style(danger_button_style)
                .on_press(Message::GraphDeleteRelation(from_id, to_id))
                .padding([4, 10]),]
            .align_y(Alignment::Center);

            let insp_col = column![
                header,
                nodes_info,
                kind_selector,
                directed_selector,
                label_input,
                actions
            ]
            .spacing(12);

            container(scrollable(insp_col))
                .style(card_container_style)
                .padding(14)
                .width(Length::Fixed(290.0))
                .height(Length::Fill)
                .into()
        } else {
            container(
                text("Relation ikke fundet")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED),
            )
            .width(Length::Fixed(290.0))
            .height(Length::Fill)
            .into()
        }
    } else if let Some(selected_id) = selected_node_id {
        if let Some(node) = concept_graph.find_node(selected_id) {
            match (is_inline_editing, editor_state) {
                (true, Some(editor)) => {
                    let mut edit_col = column![
                        row![
                            text("Hurtigredigering")
                                .size(14)
                                .color(ThemeColors::PRIMARY),
                            Space::new().width(Length::Fill),
                            button(text("✕").size(11))
                                .style(secondary_button_style)
                                .on_press(Message::CancelConceptEdit)
                                .padding([2, 5]),
                        ]
                        .align_y(Alignment::Center),
                        text("Rediger nodens begreb direkte:")
                            .size(11)
                            .color(ThemeColors::TEXT_MUTED),
                    ]
                    .spacing(8);

                    if let Some(err) = &editor.validation_error {
                        edit_col = edit_col.push(
                            container(
                                text(format!("⚠️ {}", err))
                                    .size(11)
                                    .color(ThemeColors::ACCENT_RED),
                            )
                            .style(card_container_style)
                            .padding(6)
                            .width(Length::Fill),
                        );
                    }

                    edit_col = edit_col.push(
                        column![
                            text("Foretrukken term *")
                                .size(11)
                                .color(ThemeColors::SLATE_700),
                            text_input("Foretrukken term...", &editor.preferred_term)
                                .id("preferred_term_input")
                                .style(modern_input_style)
                                .on_input(|v| {
                                    Message::UpdateConceptField(ConceptFormField::PreferredTerm, v)
                                })
                                .on_submit(Message::SaveConcept)
                                .padding(6)
                                .width(Length::Fill),
                        ]
                        .spacing(3),
                    );

                    edit_col = edit_col.push(
                        column![
                            text("Definition (Aristoteles' formel) *")
                                .size(11)
                                .color(ThemeColors::SLATE_700),
                            text_input("Definition...", &editor.definition)
                                .style(modern_input_style)
                                .on_input(|v| {
                                    Message::UpdateConceptField(ConceptFormField::Definition, v)
                                })
                                .on_submit(Message::SaveConcept)
                                .padding(6)
                                .width(Length::Fill),
                        ]
                        .spacing(3),
                    );

                    let edit_actions = row![
                        button(text("Annuller").size(11))
                            .style(secondary_button_style)
                            .on_press(Message::CancelConceptEdit)
                            .padding([5, 10]),
                        Space::new().width(Length::Fill),
                        button(text("Gem Begreb").size(11))
                            .style(primary_button_style)
                            .on_press(Message::SaveConcept)
                            .padding([5, 12]),
                    ]
                    .align_y(Alignment::Center);

                    edit_col = edit_col.push(edit_actions);

                    container(scrollable(edit_col.spacing(10)))
                        .style(card_container_style)
                        .padding(14)
                        .width(Length::Fixed(290.0))
                        .height(Length::Fill)
                        .into()
                }
                _ => {
                    let concept = concepts.iter().find(|c| c.id() == node.concept_id());
                    let title = node.label();
                    let is_local = node.is_local();

                    let (badge_text, badge_bg, badge_border) = if is_local {
                        (
                            "Lokalt begreb",
                            ThemeColors::FDA_SAND,
                            ThemeColors::FDA_SAND_BORDER,
                        )
                    } else {
                        (
                            "Indlånt begreb",
                            ThemeColors::FDA_BORROWED_BLUE_BG,
                            ThemeColors::FDA_BORROWED_BLUE,
                        )
                    };

                    let badge = container(text(badge_text).size(11).color(ThemeColors::SLATE_800))
                        .style(move |_| container::Style {
                            background: Some(iced::Background::Color(badge_bg)),
                            border: iced::Border {
                                color: badge_border,
                                width: 1.0,
                                radius: 4.0.into(),
                            },
                            ..Default::default()
                        })
                        .padding([3, 8]);

                    let mut insp = column![
                        row![
                            text("Inspector").size(11).color(ThemeColors::TEXT_MUTED),
                            Space::new().width(Length::Fill),
                            badge,
                            button(text("✕").size(11))
                                .style(secondary_button_style)
                                .on_press(Message::GraphNodeSelected(None))
                                .padding([2, 5]),
                        ]
                        .align_y(Alignment::Center),
                        text(title).size(18).color(ThemeColors::SLATE_900),
                    ]
                    .spacing(8);

                    if let Some(c) = concept {
                        insp = insp.push(
                            container(
                                column![
                                    text("Definition").size(11).color(ThemeColors::TEXT_MUTED),
                                    text(if c.definition().is_empty() {
                                        "Ingen definition angivet."
                                    } else {
                                        c.definition()
                                    })
                                    .size(12)
                                    .color(ThemeColors::SLATE_800),
                                ]
                                .spacing(4),
                            )
                            .style(card_container_style)
                            .padding(8)
                            .width(Length::Fill),
                        );

                        insp = insp.push(
                            row![
                                button(text("✏️ Hurtigrediger").size(11))
                                    .style(secondary_button_style)
                                    .on_press(Message::EditConcept(c.id()))
                                    .padding([4, 8]),
                                Space::new().width(6),
                                button(text("Fjern fra diagram").size(11))
                                    .style(danger_button_style)
                                    .on_press(Message::RemoveConceptFromDiagram(selected_id))
                                    .padding([4, 8]),
                            ]
                            .align_y(Alignment::Center),
                        );
                    }

                    // Tilknyttede relationer
                    let connected_edges: Vec<_> = concept_graph
                        .edges()
                        .iter()
                        .filter(|e| e.from() == selected_id || e.to() == selected_id)
                        .collect();

                    if !connected_edges.is_empty() {
                        insp = insp.push(Space::new().height(4));
                        insp = insp.push(
                            text("Tilknyttede relationer:")
                                .size(12)
                                .color(ThemeColors::PRIMARY),
                        );

                        for edge in connected_edges {
                            let from_node = concept_graph.find_node(edge.from());
                            let to_node = concept_graph.find_node(edge.to());
                            let from_name = from_node.map(|n| n.label()).unwrap_or("?");
                            let to_name = to_node.map(|n| n.label()).unwrap_or("?");
                            let desc = match edge.kind() {
                                RelationKind::Generalization => {
                                    format!("{} ⮞ {}", from_name, to_name)
                                }
                                RelationKind::Association => {
                                    if let Some(lbl) = edge.label() {
                                        format!("{} ──({})── {}", from_name, lbl, to_name)
                                    } else {
                                        format!("{} ── {}", from_name, to_name)
                                    }
                                }
                                RelationKind::Composition => {
                                    format!("{} ◆── {}", from_name, to_name)
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
                                    .on_press(Message::GraphDeleteRelation(edge_from, edge_to))
                                    .padding([2, 5]),
                            ]
                            .spacing(4)
                            .align_y(Alignment::Center);

                            insp = insp.push(edge_row);
                        }
                    }

                    container(scrollable(insp.spacing(8)))
                        .style(card_container_style)
                        .padding(14)
                        .width(Length::Fixed(290.0))
                        .height(Length::Fill)
                        .into()
                }
            }
        } else {
            container(
                text("Ingen node valgt")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED),
            )
            .width(Length::Fixed(290.0))
            .height(Length::Fill)
            .into()
        }
    } else {
        container(
            column![
                text("💡 Begrebsmodel Studio")
                    .size(14)
                    .color(ThemeColors::PRIMARY),
                text("• Venstre palet: Klik [+] for at tilføje et begreb til diagrammet.")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED),
                text("• Canvas: Træk en node med musen for at ændre placering.")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED),
                text("• Klik '+ Opret Relation' for at forbinde begreber.")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED),
                text("• FDA Sand (#FEFAF7) = Lokalt begreb.")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED),
                text("• FDA Blå (#87CDEB) = Lånt begreb.")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED),
                text("• 'Fjern fra diagram' fjerner kun kassen på canvas - begrebet bevares i projektet.")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED),
            ]
            .spacing(8),
        )
        .style(card_container_style)
        .padding(14)
        .width(Length::Fixed(290.0))
        .height(Length::Fill)
        .into()
    };

    row![left_palette, center_content, right_inspector]
        .spacing(12)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
