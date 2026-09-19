use crate::features::concepts::Concept;
use crate::features::model::ModelProject;
use crate::ui::concept_editor::ConceptEditorState;
use crate::ui::concept_table;
use crate::ui::theme::ThemeColors;
use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Element, Length};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use crate::ui::concept_editor::ConceptFormField;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tab {
    Metadata,
    ConceptList,
    ConceptModel,
    InformationModel,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectTab(Tab),
    NewProject,

    // Begrebsliste CRUD-handlinger
    StartNewConcept,
    EditConcept(Uuid),
    DeleteConcept(Uuid),
    SaveConcept,
    CancelConceptEdit,
    UpdateConceptField(ConceptFormField, String),
    SearchQueryChanged(String),
    ToggleShowAllFields,
}

pub struct App {
    project: ModelProject,
    active_tab: Tab,
    editor_state: Option<ConceptEditorState>,
    search_query: String,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            project: ModelProject::default(),
            active_tab: Tab::Metadata,
            editor_state: None,
            search_query: String::new(),
        }
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Light
    }

    pub fn active_tab(&self) -> Tab {
        self.active_tab
    }

    pub fn project(&self) -> &ModelProject {
        &self.project
    }

    pub fn is_editing_concept(&self) -> bool {
        self.editor_state.is_some()
    }

    pub fn filtered_concepts(&self) -> Vec<&Concept> {
        let q = self.search_query.trim().to_lowercase();
        if q.is_empty() {
            self.project.concepts().iter().collect()
        } else {
            self.project
                .concepts()
                .iter()
                .filter(|c| {
                    c.preferred_term().to_lowercase().contains(&q)
                        || c.definition().to_lowercase().contains(&q)
                        || c.accepted_term().is_some_and(|t| t.to_lowercase().contains(&q))
                        || c.source().is_some_and(|s| s.to_lowercase().contains(&q))
                        || c.legal_source().is_some_and(|l| l.to_lowercase().contains(&q))
                        || c.identifier().is_some_and(|i| i.to_lowercase().contains(&q))
                })
                .collect()
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectTab(tab) => {
                self.active_tab = tab;
            }
            Message::NewProject => {
                self.project = ModelProject::default();
                self.active_tab = Tab::Metadata;
                self.editor_state = None;
                self.search_query.clear();
            }
            Message::StartNewConcept => {
                self.editor_state = Some(ConceptEditorState::new_empty());
            }
            Message::EditConcept(id) => {
                if let Some(concept) = self.project.get_concept(id) {
                    self.editor_state = Some(ConceptEditorState::from_concept(concept));
                }
            }
            Message::DeleteConcept(id) => {
                self.project.remove_concept(id);
                if let Some(editor) = &self.editor_state {
                    if editor.editing_id == Some(id) {
                        self.editor_state = None;
                    }
                }
            }
            Message::SaveConcept => {
                if let Some(editor) = &mut self.editor_state {
                    match editor.build_concept() {
                        Ok(concept) => {
                            let result = if editor.editing_id.is_some() {
                                self.project.update_concept(concept)
                            } else {
                                self.project.add_concept(concept).map(|_| ())
                            };

                            match result {
                                Ok(()) => {
                                    self.editor_state = None;
                                }
                                Err(err) => {
                                    editor.validation_error = Some(err.to_string());
                                }
                            }
                        }
                        Err(err) => {
                            editor.validation_error = Some(err.to_string());
                        }
                    }
                }
            }
            Message::CancelConceptEdit => {
                self.editor_state = None;
            }
            Message::UpdateConceptField(field, value) => {
                if let Some(editor) = &mut self.editor_state {
                    editor.update_field(field, value);
                }
            }
            Message::SearchQueryChanged(query) => {
                self.search_query = query;
            }
            Message::ToggleShowAllFields => {
                if let Some(editor) = &mut self.editor_state {
                    editor.show_supplementary = !editor.show_supplementary;
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let tab_button = |tab: Tab, label: &'static str| {
            let is_active = self.active_tab == tab;
            let label_text = if is_active {
                text(format!("● {}", label)).size(13).color(ThemeColors::PRIMARY)
            } else {
                text(label).size(13)
            };
            button(label_text).on_press(Message::SelectTab(tab))
        };

        let nav_bar = row![
            row![
                text("edge")
                    .size(22)
                    .color(ThemeColors::PRIMARY),
                Space::new().width(4),
                container(text("FDA v2.1").size(10).color(ThemeColors::PRIMARY))
                    .padding([2, 6]),
            ]
            .align_y(Alignment::Center),
            Space::new().width(24),
            tab_button(Tab::Metadata, "1. Omslag & Metadata"),
            tab_button(Tab::ConceptList, "2. Begrebsliste (Bilag D & E)"),
            tab_button(Tab::ConceptModel, "3. Begrebsmodel (Graf)"),
            tab_button(Tab::InformationModel, "4. Informationsmodel"),
            Space::new().width(Length::Fill),
            button(text("Nyt Projekt").size(12)).on_press(Message::NewProject),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let content: Element<Message> = match self.active_tab {
            Tab::Metadata => column![
                text(format!("Model: {}", self.project.metadata().name())).size(24),
                text(format!("Emneområde: {}", self.project.metadata().domain_area()))
                    .color(ThemeColors::TEXT_MUTED),
                text(format!("Ansvarlig: {}", self.project.metadata().responsible_org()))
                    .color(ThemeColors::TEXT_MUTED),
                text(format!(
                    "Version: {} ({:?})",
                    self.project.metadata().version(),
                    self.project.metadata().status()
                ))
                .color(ThemeColors::TEXT_MUTED),
                text(format!("URI: {}", self.project.metadata().uri()))
                    .color(ThemeColors::TEXT_MUTED),
                text(format!("Beskrivelse: {}", self.project.metadata().description())),
            ]
            .spacing(12)
            .into(),

            Tab::ConceptList => {
                if let Some(editor) = &self.editor_state {
                    editor.view()
                } else {
                    let filtered = self.filtered_concepts();
                    concept_table::view(filtered, self.project.concepts().len(), &self.search_query)
                }
            }

            Tab::ConceptModel => column![
                text("Begrebsmodel (Graf)").size(22),
                text("Interaktiv grafkomponent med noder, generaliseringer og associationer etableres i Fase 3.")
                    .color(ThemeColors::TEXT_MUTED),
            ]
            .spacing(10)
            .into(),

            Tab::InformationModel => column![
                text("Informationsmodel (UML Klasser)").size(22),
                text("Informationsmodel med attributter, datatyper og multiplicitet etableres i Fase 4.")
                    .color(ThemeColors::TEXT_MUTED),
            ]
            .spacing(10)
            .into(),
        };

        let status_bar = row![
            text("FDA Modelregler v2.1 • Klar")
                .size(12)
                .color(ThemeColors::TEXT_MUTED),
            Space::new().width(12),
            text(format!("• {} begreber i model", self.project.concepts().len()))
                .size(12)
                .color(ThemeColors::TEXT_MUTED),
            Space::new().width(Length::Fill),
            text(format!("Aktiv fane: {:?}", self.active_tab))
                .size(12)
                .color(ThemeColors::TEXT_MUTED),
        ]
        .align_y(Alignment::Center);

        container(
            column![
                nav_bar,
                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(16),
                status_bar,
            ]
            .spacing(12)
            .padding(14),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
