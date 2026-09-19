use crate::features::concepts::Concept;
use crate::features::model::storage::ProjectStorage;
use crate::features::model::ModelProject;
use crate::ui::concept_editor::ConceptEditorState;
use crate::ui::concept_table;
use crate::ui::theme::ThemeColors;
use iced::event::{self, Event};
use iced::keyboard::{self, key::Named, Key};
use iced::widget::operation;
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length, Subscription, Task};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

pub use crate::ui::concept_editor::ConceptFormField;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tab {
    Metadata,
    ConceptList,
    ConceptModel,
    InformationModel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveStatus {
    Saved(String),
    Saving,
    Unsaved,
    Error(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileDialogMode {
    Open,
    SaveAs,
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

    // Persistens & Filhåndtering
    SaveProject,
    OpenProjectDialog,
    SaveProjectAsDialog,
    CloseFileDialog,
    FileDialogInputChanged(String),
    ConfirmFileDialog,
    OpenProjectFile(PathBuf),
    SaveProjectToFile(PathBuf),

    // Tastaturnavigation & genveje
    FocusNext,
    FocusPrevious,
    EscapePressed,
}

pub struct App {
    project: ModelProject,
    active_tab: Tab,
    editor_state: Option<ConceptEditorState>,
    search_query: String,
    current_file_path: Option<PathBuf>,
    save_status: SaveStatus,
    file_dialog_mode: Option<FileDialogMode>,
    file_dialog_input: String,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let default_path = ProjectStorage::default_project_path();
        if default_path.exists() {
            match ProjectStorage::load_from_file(&default_path) {
                Ok(proj) => Self {
                    project: proj,
                    active_tab: Tab::Metadata,
                    editor_state: None,
                    search_query: String::new(),
                    current_file_path: Some(default_path.clone()),
                    save_status: SaveStatus::Saved(default_path.display().to_string()),
                    file_dialog_mode: None,
                    file_dialog_input: String::new(),
                },
                Err(err) => Self {
                    project: ModelProject::default(),
                    active_tab: Tab::Metadata,
                    editor_state: None,
                    search_query: String::new(),
                    current_file_path: Some(default_path),
                    save_status: SaveStatus::Error(err.to_string()),
                    file_dialog_mode: None,
                    file_dialog_input: String::new(),
                },
            }
        } else {
            Self::new_with_path(Some(default_path))
        }
    }

    pub fn new_with_path(path: Option<PathBuf>) -> Self {
        let save_status = match &path {
            Some(p) => SaveStatus::Saved(p.display().to_string()),
            None => SaveStatus::Unsaved,
        };

        Self {
            project: ModelProject::default(),
            active_tab: Tab::Metadata,
            editor_state: None,
            search_query: String::new(),
            current_file_path: path,
            save_status,
            file_dialog_mode: None,
            file_dialog_input: String::new(),
        }
    }

    pub fn current_file_path(&self) -> Option<&PathBuf> {
        self.current_file_path.as_ref()
    }

    pub fn save_status(&self) -> &SaveStatus {
        &self.save_status
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

    pub fn is_file_dialog_open(&self) -> bool {
        self.file_dialog_mode.is_some()
    }

    pub fn trigger_autosave(&mut self) {
        if let Some(path) = &self.current_file_path {
            match ProjectStorage::save_to_file(&self.project, path) {
                Ok(()) => {
                    self.save_status = SaveStatus::Saved(path.display().to_string());
                }
                Err(err) => {
                    self.save_status = SaveStatus::Error(err.to_string());
                }
            }
        }
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

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectTab(tab) => {
                self.active_tab = tab;
            }
            Message::NewProject => {
                self.project = ModelProject::default();
                self.active_tab = Tab::Metadata;
                self.editor_state = None;
                self.search_query.clear();
                self.trigger_autosave();
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
                self.trigger_autosave();
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
                                    self.trigger_autosave();
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

            // Persistens & Filhåndtering
            Message::SaveProject => {
                self.trigger_autosave();
            }
            Message::OpenProjectDialog => {
                self.file_dialog_mode = Some(FileDialogMode::Open);
                self.file_dialog_input = self
                    .current_file_path
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "model.edge.json".to_string());
            }
            Message::SaveProjectAsDialog => {
                self.file_dialog_mode = Some(FileDialogMode::SaveAs);
                self.file_dialog_input = self
                    .current_file_path
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "ny_model.edge.json".to_string());
            }
            Message::CloseFileDialog => {
                self.file_dialog_mode = None;
            }
            Message::FileDialogInputChanged(path_str) => {
                self.file_dialog_input = path_str;
            }
            Message::ConfirmFileDialog => {
                if let Some(mode) = self.file_dialog_mode {
                    let path = PathBuf::from(self.file_dialog_input.trim());
                    if !self.file_dialog_input.trim().is_empty() {
                        self.file_dialog_mode = None;
                        return match mode {
                            FileDialogMode::Open => self.update(Message::OpenProjectFile(path)),
                            FileDialogMode::SaveAs => self.update(Message::SaveProjectToFile(path)),
                        };
                    }
                }
                self.file_dialog_mode = None;
            }
            Message::OpenProjectFile(path) => match ProjectStorage::load_from_file(&path) {
                Ok(proj) => {
                    self.project = proj;
                    let display = path.display().to_string();
                    self.current_file_path = Some(path);
                    self.save_status = SaveStatus::Saved(display);
                    self.file_dialog_mode = None;
                }
                Err(err) => {
                    self.save_status =
                        SaveStatus::Error(format!("Kunne ikke åbne {}: {}", path.display(), err));
                }
            },
            Message::SaveProjectToFile(path) => {
                self.current_file_path = Some(path);
                self.trigger_autosave();
                self.file_dialog_mode = None;
            }

            // Tastaturnavigation & genveje
            Message::FocusNext => {
                return operation::focus_next();
            }
            Message::FocusPrevious => {
                return operation::focus_previous();
            }
            Message::EscapePressed => {
                if self.file_dialog_mode.is_some() {
                    self.file_dialog_mode = None;
                } else if self.editor_state.is_some() {
                    self.editor_state = None;
                }
            }
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| {
            if let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event {
                match key.as_ref() {
                    Key::Named(Named::Tab) => {
                        if modifiers.shift() {
                            Some(Message::FocusPrevious)
                        } else {
                            Some(Message::FocusNext)
                        }
                    }
                    Key::Named(Named::Escape) => Some(Message::EscapePressed),
                    Key::Character(c)
                        if (c == "s" || c == "S")
                            && (modifiers.control() || modifiers.command()) =>
                    {
                        Some(Message::SaveProject)
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
    }

    pub fn view(&self) -> Element<'_, Message> {
        let tab_button = |tab: Tab, label: &'static str| {
            let is_active = self.active_tab == tab;
            button(text(label).size(13))
                .style(if is_active {
                    button::primary
                } else {
                    button::secondary
                })
                .on_press(Message::SelectTab(tab))
                .padding([6, 12])
        };

        let nav_bar = row![
            row![
                text("edge").size(24).color(ThemeColors::PRIMARY),
                Space::new().width(6),
                container(text("FDA v2.1").size(10).color(ThemeColors::PRIMARY))
                    .style(|_theme: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color(ThemeColors::PRIMARY_LIGHT)),
                        border: iced::Border {
                            color: ThemeColors::PRIMARY,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    })
                    .padding([2, 6]),
            ]
            .align_y(Alignment::Center),
            Space::new().width(20),
            tab_button(Tab::Metadata, "1. Omslag & Metadata"),
            tab_button(Tab::ConceptList, "2. Begrebsliste (Bilag D & E)"),
            tab_button(Tab::ConceptModel, "3. Begrebsmodel (Graf)"),
            tab_button(Tab::InformationModel, "4. Informationsmodel"),
            Space::new().width(Length::Fill),
            button(text("📁 Åbn...").size(12))
                .style(button::secondary)
                .on_press(Message::OpenProjectDialog)
                .padding([6, 10]),
            button(text("💾 Gem som...").size(12))
                .style(button::secondary)
                .on_press(Message::SaveProjectAsDialog)
                .padding([6, 10]),
            button(text("Nyt Projekt").size(12))
                .style(button::secondary)
                .on_press(Message::NewProject)
                .padding([6, 10]),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        // Valgfri fildialog banner (Open / Save As)
        let file_dialog_banner: Option<Element<Message>> = self.file_dialog_mode.map(|mode| {
            let (mode_label, confirm_label) = match mode {
                FileDialogMode::Open => ("Åbn modelprojekt:", "Åbn"),
                FileDialogMode::SaveAs => ("Gem modelprojekt som:", "Gem"),
            };

            container(
                row![
                    text(mode_label).size(13).color(ThemeColors::PRIMARY),
                    text_input("Filsti (f.eks. model.edge.json)...", &self.file_dialog_input)
                        .on_input(Message::FileDialogInputChanged)
                        .on_submit(Message::ConfirmFileDialog)
                        .padding(6)
                        .width(Length::FillPortion(2)),
                    button(text(confirm_label).size(12))
                        .style(button::primary)
                        .on_press(Message::ConfirmFileDialog)
                        .padding([4, 12]),
                    button(text("✕").size(12))
                        .style(button::secondary)
                        .on_press(Message::CloseFileDialog)
                        .padding([4, 8]),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            )
            .style(container::bordered_box)
            .padding([8, 14])
            .width(Length::Fill)
            .into()
        });

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

        let save_status_text = match &self.save_status {
            SaveStatus::Saved(target) => format!("💾 Gemt i {}", target),
            SaveStatus::Saving => "⏳ Gemmer...".to_string(),
            SaveStatus::Unsaved => "⚠️ Ikke gemt til fil".to_string(),
            SaveStatus::Error(msg) => format!("❌ Fejl ved gemning: {}", msg),
        };

        let status_bar = row![
            text("FDA Modelregler v2.1 • Klar")
                .size(12)
                .color(ThemeColors::TEXT_MUTED),
            Space::new().width(12),
            text(format!("• {} begreber i model", self.project.concepts().len()))
                .size(12)
                .color(ThemeColors::TEXT_MUTED),
            Space::new().width(12),
            text(format!("• {}", save_status_text))
                .size(12)
                .color(match &self.save_status {
                    SaveStatus::Error(_) => ThemeColors::ACCENT_RED,
                    _ => ThemeColors::TEXT_MUTED,
                }),
            Space::new().width(Length::Fill),
            text(format!("Aktiv fane: {:?}", self.active_tab))
                .size(12)
                .color(ThemeColors::TEXT_MUTED),
        ]
        .align_y(Alignment::Center);

        let mut main_col = column![nav_bar].spacing(12);

        if let Some(dialog) = file_dialog_banner {
            main_col = main_col.push(dialog);
        }

        main_col = main_col.push(
            container(content)
                .style(container::bordered_box)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(20),
        );

        main_col = main_col.push(status_bar);

        container(main_col)
            .padding(16)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
