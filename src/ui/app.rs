use crate::features::concept_model::{NodeId, RelationKind};
use crate::features::concepts::Concept;
use crate::features::model::storage::ProjectStorage;
use crate::features::model::ModelProject;
use crate::ui::concept_editor::ConceptEditorState;
use crate::ui::concept_table;
use crate::ui::graph_canvas::GraphCanvas;
use crate::ui::theme::ThemeColors;
use iced::event::{self, Event};
use iced::keyboard::{self, key::Named, Key};
use iced::widget::canvas;
use iced::widget::operation;
use iced::widget::pick_list;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length, Subscription, Task};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeOption {
    pub id: NodeId,
    pub label: String,
}

impl std::fmt::Display for NodeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

#[derive(Debug, Clone)]
pub struct RelationDialogState {
    pub from_node: Option<NodeOption>,
    pub to_node: Option<NodeOption>,
    pub kind: RelationKind,
    pub label: String,
    pub error: Option<String>,
}


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
    OpenDialogCompleted(crate::ui::file_dialog::DialogResult),
    SaveDialogCompleted(crate::ui::file_dialog::DialogResult),
    OpenInlineFileDialog(FileDialogMode),
    CloseFileDialog,
    FileDialogInputChanged(String),
    ConfirmFileDialog,
    OpenProjectFile(PathBuf),
    SaveProjectToFile(PathBuf),

    // Tastaturnavigation & genveje
    FocusNext,
    FocusPrevious,
    EscapePressed,

    // Graf-handlinger (Fase 3)
    GraphNodeSelected(Option<NodeId>),
    GraphNodeMoved(NodeId, f32, f32),
    GraphOpenRelationDialog,
    GraphCloseRelationDialog,
    GraphRelationFromChanged(NodeOption),
    GraphRelationToChanged(NodeOption),
    GraphRelationKindChanged(RelationKind),
    GraphRelationLabelChanged(String),
    GraphCreateRelation,
    GraphDeleteRelation(NodeId, NodeId),
    GraphSyncNodes,
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
    selected_graph_node_id: Option<NodeId>,
    relation_dialog: Option<RelationDialogState>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let default_path = ProjectStorage::default_project_path();
        Self::new_with_path(Some(default_path))
    }

    pub fn new_with_path(path: Option<PathBuf>) -> Self {
        if let Some(p) = &path {
            if p.exists() {
                if let Ok(mut proj) = ProjectStorage::load_from_file(p) {
                    proj.sync_concept_graph();
                    return Self {
                        project: proj,
                        active_tab: Tab::Metadata,
                        editor_state: None,
                        search_query: String::new(),
                        current_file_path: path.clone(),
                        save_status: SaveStatus::Saved(p.display().to_string()),
                        file_dialog_mode: None,
                        file_dialog_input: String::new(),
                        selected_graph_node_id: None,
                        relation_dialog: None,
                    };
                }
            }
        }

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
            selected_graph_node_id: None,
            relation_dialog: None,
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

    pub fn selected_graph_node_id(&self) -> Option<NodeId> {
        self.selected_graph_node_id
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
                if tab == Tab::ConceptModel {
                    self.project.sync_concept_graph();
                }
            }
            Message::NewProject => {
                self.project = ModelProject::default();
                self.active_tab = Tab::Metadata;
                self.editor_state = None;
                self.search_query.clear();
                self.current_file_path = None;
                self.save_status = SaveStatus::Unsaved;
                self.selected_graph_node_id = None;
                self.relation_dialog = None;
            }
            Message::StartNewConcept => {
                self.editor_state = Some(ConceptEditorState::new_empty());
            }
            Message::EditConcept(id) => {
                if let Some(concept) = self.project.get_concept(id) {
                    self.editor_state = Some(ConceptEditorState::from_concept(concept));
                    self.active_tab = Tab::ConceptList;
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
                if self.current_file_path.is_some() {
                    self.trigger_autosave();
                } else {
                    return self.update(Message::SaveProjectAsDialog);
                }
            }
            Message::OpenProjectDialog => {
                return Task::perform(
                    async { crate::ui::file_dialog::pick_file_to_open() },
                    Message::OpenDialogCompleted,
                );
            }
            Message::OpenDialogCompleted(res) => match res {
                crate::ui::file_dialog::DialogResult::Selected(path) => {
                    return self.update(Message::OpenProjectFile(path));
                }
                crate::ui::file_dialog::DialogResult::Cancelled => {}
                crate::ui::file_dialog::DialogResult::Unavailable => {
                    return self.update(Message::OpenInlineFileDialog(FileDialogMode::Open));
                }
            },
            Message::SaveProjectAsDialog => {
                let default_name = self
                    .current_file_path
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .and_then(|f| f.to_str())
                    .unwrap_or("model.edge.json")
                    .to_string();

                return Task::perform(
                    async move { crate::ui::file_dialog::pick_file_to_save(Some(&default_name)) },
                    Message::SaveDialogCompleted,
                );
            }
            Message::SaveDialogCompleted(res) => match res {
                crate::ui::file_dialog::DialogResult::Selected(path) => {
                    return self.update(Message::SaveProjectToFile(path));
                }
                crate::ui::file_dialog::DialogResult::Cancelled => {}
                crate::ui::file_dialog::DialogResult::Unavailable => {
                    return self.update(Message::OpenInlineFileDialog(FileDialogMode::SaveAs));
                }
            },
            Message::OpenInlineFileDialog(mode) => {
                self.file_dialog_mode = Some(mode);
                self.file_dialog_input = self
                    .current_file_path
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "model.edge.json".to_string());
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
                Ok(mut proj) => {
                    proj.sync_concept_graph();
                    self.project = proj;
                    let display = path.display().to_string();
                    self.current_file_path = Some(path);
                    self.save_status = SaveStatus::Saved(display);
                    self.file_dialog_mode = None;
                    self.selected_graph_node_id = None;
                    self.relation_dialog = None;
                    if !self.project.concepts().is_empty() {
                        self.active_tab = Tab::ConceptList;
                    }
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
                if self.relation_dialog.is_some() {
                    self.relation_dialog = None;
                } else if self.file_dialog_mode.is_some() {
                    self.file_dialog_mode = None;
                } else if self.editor_state.is_some() {
                    self.editor_state = None;
                } else if self.selected_graph_node_id.is_some() {
                    self.selected_graph_node_id = None;
                }
            }

            // Graf-handlinger (Fase 3)
            Message::GraphNodeSelected(node_id) => {
                self.selected_graph_node_id = node_id;
            }
            Message::GraphNodeMoved(node_id, x, y) => {
                self.project.concept_graph_mut().update_node_position(node_id, x, y);
                self.trigger_autosave();
            }
            Message::GraphOpenRelationDialog => {
                let node_options: Vec<NodeOption> = self
                    .project
                    .concept_graph()
                    .nodes()
                    .iter()
                    .map(|n| NodeOption {
                        id: n.id(),
                        label: n.label().to_string(),
                    })
                    .collect();

                let from_node = node_options.first().cloned();
                let to_node = node_options.get(1).or_else(|| node_options.first()).cloned();
                self.relation_dialog = Some(RelationDialogState {
                    from_node,
                    to_node,
                    kind: RelationKind::Generalization,
                    label: String::new(),
                    error: None,
                });
            }
            Message::GraphCloseRelationDialog => {
                self.relation_dialog = None;
            }
            Message::GraphRelationFromChanged(opt) => {
                if let Some(dialog) = &mut self.relation_dialog {
                    dialog.from_node = Some(opt);
                    dialog.error = None;
                }
            }
            Message::GraphRelationToChanged(opt) => {
                if let Some(dialog) = &mut self.relation_dialog {
                    dialog.to_node = Some(opt);
                    dialog.error = None;
                }
            }
            Message::GraphRelationKindChanged(kind) => {
                if let Some(dialog) = &mut self.relation_dialog {
                    dialog.kind = kind;
                }
            }
            Message::GraphRelationLabelChanged(label) => {
                if let Some(dialog) = &mut self.relation_dialog {
                    dialog.label = label;
                }
            }
            Message::GraphCreateRelation => {
                if let Some(dialog) = &self.relation_dialog {
                    match (&dialog.from_node, &dialog.to_node) {
                        (Some(from), Some(to)) => {
                            if from.id == to.id {
                                if let Some(d) = &mut self.relation_dialog {
                                    d.error = Some(
                                        "Kilde og mål kan ikke være det samme begreb.".to_string(),
                                    );
                                }
                            } else {
                                let label = if dialog.kind == RelationKind::Association
                                    && !dialog.label.trim().is_empty()
                                {
                                    Some(dialog.label.trim().to_string())
                                } else {
                                    None
                                };
                                self.project.concept_graph_mut().add_relation_with_label(
                                    from.id,
                                    to.id,
                                    dialog.kind,
                                    label,
                                );
                                self.relation_dialog = None;
                                self.trigger_autosave();
                            }
                        }
                        _ => {
                            if let Some(d) = &mut self.relation_dialog {
                                d.error = Some("Vælg venligst både kilde og målbegreb.".to_string());
                            }
                        }
                    }
                }
            }
            Message::GraphDeleteRelation(from, to) => {
                self.project.concept_graph_mut().remove_relation(from, to);
                self.trigger_autosave();
            }
            Message::GraphSyncNodes => {
                self.project.sync_concept_graph();
                self.trigger_autosave();
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
                text("Edge").size(24).color(ThemeColors::PRIMARY),
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
            button(text("💾 Gem").size(12))
                .style(button::secondary)
                .on_press(Message::SaveProject)
                .padding([6, 10]),
            button(text("💾 Gem som...").size(12))
                .style(button::secondary)
                .on_press(Message::SaveProjectAsDialog)
                .padding([6, 10]),
            button(text("+ Nyt Projekt").size(12))
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

            let mut banner_col = column![row![
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
                button(text("🖥️ Gennemse...").size(12))
                    .style(button::secondary)
                    .on_press(match mode {
                        FileDialogMode::Open => Message::OpenProjectDialog,
                        FileDialogMode::SaveAs => Message::SaveProjectAsDialog,
                    })
                    .padding([4, 10]),
                button(text("✕").size(12))
                    .style(button::secondary)
                    .on_press(Message::CloseFileDialog)
                    .padding([4, 8]),
            ]
            .spacing(10)
            .align_y(Alignment::Center)]
            .spacing(8);

            if mode == FileDialogMode::Open {
                let local_files = crate::ui::file_dialog::scan_local_project_files(&PathBuf::from("."));
                if !local_files.is_empty() {
                    let mut chips = row![text("Genveje i mappen:").size(11).color(ThemeColors::TEXT_MUTED)]
                        .spacing(8)
                        .align_y(Alignment::Center);
                    for f in local_files {
                        let name = f.file_name().and_then(|n| n.to_str()).unwrap_or("model.edge.json").to_string();
                        chips = chips.push(
                            button(text(format!("📄 {}", name)).size(11))
                                .style(button::secondary)
                                .on_press(Message::OpenProjectFile(f))
                                .padding([2, 8]),
                        );
                    }
                    banner_col = banner_col.push(chips);
                }
            }

            container(banner_col)
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

            Tab::ConceptModel => {
                let toolbar = row![
                    text("Begrebsmodel (Graf)").size(20),
                    Space::new().width(16),
                    button(text("+ Opret Relation").size(12))
                        .style(button::primary)
                        .on_press(Message::GraphOpenRelationDialog)
                        .padding([6, 12]),
                    button(text("🔄 Synkroniser Begreber").size(12))
                        .style(button::secondary)
                        .on_press(Message::GraphSyncNodes)
                        .padding([6, 10]),
                    Space::new().width(Length::Fill),
                    text(format!(
                        "Noder: {}  •  Relationer: {}",
                        self.project.concept_graph().node_count(),
                        self.project.concept_graph().edge_count()
                    ))
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED),
                ]
                .spacing(10)
                .align_y(Alignment::Center);

                let maybe_dialog_banner: Option<Element<Message>> = self.relation_dialog.as_ref().map(|d| {
                    let node_options: Vec<NodeOption> = self
                        .project
                        .concept_graph()
                        .nodes()
                        .iter()
                        .map(|n| NodeOption {
                            id: n.id(),
                            label: n.label().to_string(),
                        })
                        .collect();

                    let kinds = [RelationKind::Generalization, RelationKind::Association];

                    let mut dialog_col = column![
                        text("Opret Relation").size(14).color(ThemeColors::PRIMARY),
                    ]
                    .spacing(8);

                    if let Some(err) = &d.error {
                        dialog_col = dialog_col.push(text(err).size(12).color(ThemeColors::ACCENT_RED));
                    }

                    let from_pick = pick_list(node_options.clone(), d.from_node.clone(), Message::GraphRelationFromChanged);
                    let to_pick = pick_list(node_options, d.to_node.clone(), Message::GraphRelationToChanged);
                    let kind_pick = pick_list(kinds.to_vec(), Some(d.kind), Message::GraphRelationKindChanged);

                    let fields_row = row![
                        column![text("Kilde (fra):").size(11).color(ThemeColors::TEXT_MUTED), from_pick].spacing(2),
                        column![text("Mål (til):").size(11).color(ThemeColors::TEXT_MUTED), to_pick].spacing(2),
                        column![text("Type:").size(11).color(ThemeColors::TEXT_MUTED), kind_pick].spacing(2),
                    ]
                    .spacing(12)
                    .align_y(Alignment::Center);

                    let action_row = if d.kind == RelationKind::Association {
                        row![
                            text_input("Rolle eller associationstekst (f.eks. ejer)...", &d.label)
                                .on_input(Message::GraphRelationLabelChanged)
                                .padding(6)
                                .width(Length::FillPortion(2)),
                            button(text("Gem relation").size(12))
                                .style(button::primary)
                                .on_press(Message::GraphCreateRelation)
                                .padding([5, 12]),
                            button(text("Annuller").size(12))
                                .style(button::secondary)
                                .on_press(Message::GraphCloseRelationDialog)
                                .padding([5, 10]),
                        ]
                        .spacing(10)
                        .align_y(Alignment::Center)
                    } else {
                        row![
                            text("(Specialisering peger på superklasse med lukket hvid trekant)").size(11).color(ThemeColors::TEXT_MUTED),
                            Space::new().width(Length::Fill),
                            button(text("Gem relation").size(12))
                                .style(button::primary)
                                .on_press(Message::GraphCreateRelation)
                                .padding([5, 12]),
                            button(text("Annuller").size(12))
                                .style(button::secondary)
                                .on_press(Message::GraphCloseRelationDialog)
                                .padding([5, 10]),
                        ]
                        .spacing(10)
                        .align_y(Alignment::Center)
                    };

                    container(column![dialog_col, fields_row, action_row].spacing(10))
                        .style(container::bordered_box)
                        .padding(12)
                        .width(Length::Fill)
                        .into()
                });

                let canvas_widget = canvas(GraphCanvas::new(
                    self.project.concept_graph(),
                    self.selected_graph_node_id,
                    Message::GraphNodeSelected,
                    Message::GraphNodeMoved,
                ))
                .width(Length::Fill)
                .height(Length::Fill);

                let inspector_panel: Element<Message> = if let Some(selected_id) = self.selected_graph_node_id {
                    if let Some(node) = self.project.concept_graph().find_node(selected_id) {
                        let concept = self.project.get_concept(node.concept_id());
                        let title = node.label();
                        let is_local = node.is_local();

                        let (domain_badge, badge_bg, badge_fg) = if is_local {
                            ("Lokalt begreb (Sand farve)", ThemeColors::FDA_SAND, ThemeColors::TEXT_DARK)
                        } else {
                            ("Lånt begreb / ModelRef (Blå farve)", ThemeColors::FDA_BORROWED_BLUE, ThemeColors::TEXT_DARK)
                        };

                        let mut insp = column![
                            row![
                                text(title).size(18),
                                Space::new().width(Length::Fill),
                                button(text("✕").size(11))
                                    .style(button::secondary)
                                    .on_press(Message::GraphNodeSelected(None))
                                    .padding([2, 6]),
                            ]
                            .align_y(Alignment::Center),
                            container(text(domain_badge).size(11).color(badge_fg))
                                .style(move |_| container::Style {
                                    background: Some(iced::Background::Color(badge_bg)),
                                    border: iced::Border {
                                        color: ThemeColors::BORDER_COLOR,
                                        width: 1.0,
                                        radius: 4.0.into(),
                                    },
                                    ..Default::default()
                                })
                                .padding([3, 8]),
                        ]
                        .spacing(8);

                        if let Some(c) = concept {
                            insp = insp.push(
                                column![
                                    text("Definition:").size(11).color(ThemeColors::TEXT_MUTED),
                                    text(c.definition()).size(12),
                                ]
                                .spacing(2),
                            );

                            if let Some(src) = c.source() {
                                insp = insp.push(
                                    column![
                                        text("Kilde:").size(11).color(ThemeColors::TEXT_MUTED),
                                        text(src).size(12),
                                    ]
                                    .spacing(2),
                                );
                            }

                            if let Some(legal) = c.legal_source() {
                                insp = insp.push(
                                    column![
                                        text("Retsgrundlag:").size(11).color(ThemeColors::TEXT_MUTED),
                                        text(legal).size(12),
                                    ]
                                    .spacing(2),
                                );
                            }

                            insp = insp.push(
                                button(text("✏️ Rediger i Begrebsliste").size(12))
                                    .style(button::secondary)
                                    .on_press(Message::EditConcept(c.id()))
                                    .padding([4, 10]),
                            );
                        }

                        let connected_edges: Vec<_> = self
                            .project
                            .concept_graph()
                            .edges()
                            .iter()
                            .filter(|e| e.from() == selected_id || e.to() == selected_id)
                            .collect();

                        if !connected_edges.is_empty() {
                            insp = insp.push(Space::new().height(6));
                            insp = insp.push(
                                text("Tilknyttede relationer:")
                                    .size(12)
                                    .color(ThemeColors::PRIMARY),
                            );

                            for edge in connected_edges {
                                let from_node = self.project.concept_graph().find_node(edge.from());
                                let to_node = self.project.concept_graph().find_node(edge.to());
                                let from_name = from_node.map(|n| n.label()).unwrap_or("?");
                                let to_name = to_node.map(|n| n.label()).unwrap_or("?");
                                let desc = match edge.kind() {
                                    RelationKind::Generalization => format!("{} ⮞ {}", from_name, to_name),
                                    RelationKind::Association => {
                                        if let Some(lbl) = edge.label() {
                                            format!("{} ──({})── {}", from_name, lbl, to_name)
                                        } else {
                                            format!("{} ── {}", from_name, to_name)
                                        }
                                    }
                                    RelationKind::Composition => format!("{} ◆── {}", from_name, to_name),
                                };

                                let edge_from = edge.from();
                                let edge_to = edge.to();
                                let edge_row = row![
                                    text(desc).size(11).width(Length::Fill),
                                    button(text("🗑️").size(11))
                                        .style(button::secondary)
                                        .on_press(Message::GraphDeleteRelation(edge_from, edge_to))
                                        .padding([2, 5]),
                                ]
                                .spacing(4)
                                .align_y(Alignment::Center);

                                insp = insp.push(edge_row);
                            }
                        }

                        container(scrollable(insp.spacing(8)))
                            .style(container::bordered_box)
                            .padding(14)
                            .width(Length::Fixed(280.0))
                            .height(Length::Fill)
                            .into()
                    } else {
                        container(text("Ingen node valgt").size(12).color(ThemeColors::TEXT_MUTED))
                            .width(Length::Fixed(280.0))
                            .height(Length::Fill)
                            .into()
                    }
                } else {
                    container(
                        column![
                            text("💡 Begrebsmodel Inspector").size(14).color(ThemeColors::PRIMARY),
                            text("• Klik på en node for at se definition og relationer.")
                                .size(12)
                                .color(ThemeColors::TEXT_MUTED),
                            text("• Træk en node med musen for at ændre placering.")
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
                        ]
                        .spacing(8),
                    )
                    .style(container::bordered_box)
                    .padding(14)
                    .width(Length::Fixed(280.0))
                    .height(Length::Fill)
                    .into()
                };

                let body = row![
                    container(canvas_widget)
                        .style(|_| container::Style {
                            background: Some(iced::Background::Color(iced::Color::from_rgb(
                                0.98, 0.98, 0.98
                            ))),
                            border: iced::Border {
                                color: ThemeColors::BORDER_COLOR,
                                width: 1.0,
                                radius: 4.0.into(),
                            },
                            ..Default::default()
                        })
                        .width(Length::Fill)
                        .height(Length::Fill),
                    inspector_panel,
                ]
                .spacing(12)
                .height(Length::Fill);

                let mut graph_view = column![toolbar].spacing(10).height(Length::Fill);
                if let Some(banner) = maybe_dialog_banner {
                    graph_view = graph_view.push(banner);
                }
                graph_view = graph_view.push(body);

                graph_view.into()
            }

            Tab::InformationModel => column![
                text("Informationsmodel (UML Klasser)").size(22),
                text("Informationsmodel med attributter, datatyper og multiplicitet etableres i Fase 4.")
                    .color(ThemeColors::TEXT_MUTED),
            ]
            .spacing(10)
            .into(),
        };

        let save_status_text = match &self.save_status {
            SaveStatus::Saved(target) => {
                let full_path = std::fs::canonicalize(target).unwrap_or_else(|_| PathBuf::from(target));
                format!("💾 Gemt: {}", full_path.display())
            }
            SaveStatus::Saving => "⏳ Gemmer...".to_string(),
            SaveStatus::Unsaved => "⚠️ Nyt projekt (ikke gemt til disk - tryk Gem)".to_string(),
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
