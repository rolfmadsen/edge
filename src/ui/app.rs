use crate::features::concept_model::{NodeId, RelationKind};
use crate::features::concepts::{BelongsToDomain, Concept, ConceptValidator, ValidationError};
use crate::features::information_model::{
    Attribute, InformationClass, Multiplicity, PrimitiveType,
};
use crate::features::model::storage::ProjectStorage;
use crate::features::model::ModelProject;
use crate::ui::concept_editor::ConceptEditorState;
use crate::ui::concept_table;
use crate::ui::graph_canvas::GraphCanvas;
use crate::ui::information_model_view;
use crate::ui::theme::{
    card_container_style, danger_button_style, modal_backdrop_style, modal_card_style,
    modern_input_style, pill_container_style, primary_button_style, secondary_button_style,
    segmented_tab_button, ThemeColors,
};
use iced::event::{self, Event};
use iced::keyboard::{self, key::Named, Key};
use iced::widget::{
    button, canvas, column, container, operation, pick_list, row, scrollable, stack, text,
    text_input, Space,
};
use iced::{Alignment, Element, Length, Point, Subscription, Task};
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConceptOption {
    pub id: Uuid,
    pub term: String,
}

impl std::fmt::Display for ConceptOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.term)
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

#[derive(Debug, Clone)]
pub struct QuickCreateState {
    pub position: (f32, f32),
    pub preferred_term: String,
    pub definition: String,
    pub belongs_to_domain: BelongsToDomain,
    pub validation_error: Option<String>,
}

impl QuickCreateState {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: (x, y),
            preferred_term: String::new(),
            definition: String::new(),
            belongs_to_domain: BelongsToDomain::Yes,
            validation_error: None,
        }
    }

    pub fn build_concept(&self) -> Result<Concept, ValidationError> {
        let concept = Concept::new(
            self.preferred_term.trim(),
            self.definition.trim(),
            self.belongs_to_domain.clone(),
        );
        ConceptValidator::validate(&concept)?;
        Ok(concept)
    }
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

    // Lynoprettelse & Node-redigering på Canvas (Task 006)
    CanvasDoubleClicked(f32, f32),
    QuickCreateTermChanged(String),
    QuickCreateDefinitionChanged(String),
    QuickCreateDomainChanged(BelongsToDomain),
    QuickCreateSubmit,
    QuickCreateCancel,
    GraphNodeDoubleClicked(NodeId),

    // Canvas ergonomi, zoom, pan & grid (Task 008)
    CanvasViewportChanged(crate::ui::graph_canvas::CanvasViewport),
    ToggleSnapToGrid,
    CanvasZoomIn,
    CanvasZoomOut,
    CanvasResetView,
    CanvasSpacePressed(bool),
    CanvasModifiersChanged(iced::keyboard::Modifiers),

    // Informationsmodel (Task 010)
    SelectInformationClass(Option<Uuid>),
    CreateInformationClass,
    CreateInformationClassFromConcept(ConceptOption),
    UpdateInformationClassName(Uuid, String),
    UpdateInformationClassDescription(Uuid, String),
    AddConceptToInformationClass(Uuid, ConceptOption),
    RemoveConceptFromInformationClass(Uuid, Uuid),
    DeleteInformationClass(Uuid),
    AddAttributeToClass(Uuid),
    UpdateAttributeName(Uuid, Uuid, String),
    UpdateAttributeType(Uuid, Uuid, PrimitiveType),
    UpdateAttributeMultiplicity(Uuid, Uuid, Multiplicity),
    AddConceptToAttribute(Uuid, Uuid, ConceptOption),
    RemoveConceptFromAttribute(Uuid, Uuid, Uuid),
    DeleteAttribute(Uuid, Uuid),
    InformationClassSearchChanged(String),
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
    quick_create: Option<QuickCreateState>,
    is_inline_graph_editing: bool,
    canvas_viewport: crate::ui::graph_canvas::CanvasViewport,
    snap_to_grid: bool,
    is_space_pressed: bool,
    selected_info_class_id: Option<Uuid>,
    info_class_search: String,
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
                        quick_create: None,
                        is_inline_graph_editing: false,
                        canvas_viewport: crate::ui::graph_canvas::CanvasViewport::default(),
                        snap_to_grid: true,
                        is_space_pressed: false,
                        selected_info_class_id: None,
                        info_class_search: String::new(),
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
            quick_create: None,
            is_inline_graph_editing: false,
            canvas_viewport: crate::ui::graph_canvas::CanvasViewport::default(),
            snap_to_grid: true,
            is_space_pressed: false,
            selected_info_class_id: None,
            info_class_search: String::new(),
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

    pub fn project_mut(&mut self) -> &mut ModelProject {
        &mut self.project
    }

    pub fn is_snap_to_grid_enabled(&self) -> bool {
        self.snap_to_grid
    }

    pub fn canvas_zoom(&self) -> f32 {
        self.canvas_viewport.zoom()
    }

    pub fn canvas_viewport(&self) -> crate::ui::graph_canvas::CanvasViewport {
        self.canvas_viewport
    }

    pub fn is_editing_concept(&self) -> bool {
        self.editor_state.is_some()
    }

    pub fn is_file_dialog_open(&self) -> bool {
        self.file_dialog_mode.is_some()
    }

    pub fn is_relation_dialog_open(&self) -> bool {
        self.relation_dialog.is_some()
    }

    pub fn is_quick_create_open(&self) -> bool {
        self.quick_create.is_some()
    }

    pub fn is_node_editing(&self) -> bool {
        self.is_inline_graph_editing && self.editor_state.is_some()
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
                        || c.accepted_term()
                            .is_some_and(|t| t.to_lowercase().contains(&q))
                        || c.source().is_some_and(|s| s.to_lowercase().contains(&q))
                        || c.legal_source()
                            .is_some_and(|l| l.to_lowercase().contains(&q))
                        || c.identifier()
                            .is_some_and(|i| i.to_lowercase().contains(&q))
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
                self.is_inline_graph_editing = false;
                self.search_query.clear();
                self.current_file_path = None;
                self.save_status = SaveStatus::Unsaved;
                self.selected_graph_node_id = None;
                self.relation_dialog = None;
                self.quick_create = None;
                self.selected_info_class_id = None;
                self.info_class_search.clear();
            }
            Message::StartNewConcept => {
                self.editor_state = Some(ConceptEditorState::new_empty());
                self.is_inline_graph_editing = false;
                return operation::focus("preferred_term_input");
            }
            Message::EditConcept(id) => {
                if let Some(concept) = self.project.get_concept(id) {
                    self.editor_state = Some(ConceptEditorState::from_concept(concept));
                    self.is_inline_graph_editing = false;
                    self.active_tab = Tab::ConceptList;
                    return operation::focus("preferred_term_input");
                }
            }
            Message::DeleteConcept(id) => {
                self.project.remove_concept(id);
                if let Some(editor) = &self.editor_state {
                    if editor.editing_id == Some(id) {
                        self.editor_state = None;
                        self.is_inline_graph_editing = false;
                    }
                }
                self.trigger_autosave();
            }
            Message::SaveConcept => {
                if let Some(editor) = &mut self.editor_state {
                    match editor.build_concept() {
                        Ok(concept) => {
                            let concept_id = concept.id();
                            let pref_term = concept.preferred_term().to_string();
                            let is_node_edit = self.is_inline_graph_editing;

                            let result = if editor.editing_id.is_some() {
                                self.project.update_concept(concept)
                            } else {
                                self.project.add_concept(concept).map(|_| ())
                            };

                            match result {
                                Ok(()) => {
                                    self.editor_state = None;
                                    self.is_inline_graph_editing = false;
                                    if is_node_edit {
                                        if let Some(node) = self
                                            .project
                                            .concept_graph_mut()
                                            .find_node_by_concept_mut(concept_id)
                                        {
                                            node.set_label(pref_term);
                                        }
                                    }
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
                self.is_inline_graph_editing = false;
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
                    self.quick_create = None;
                    self.is_inline_graph_editing = false;
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
                if self.quick_create.is_some() {
                    self.quick_create = None;
                } else if self.relation_dialog.is_some() {
                    self.relation_dialog = None;
                } else if self.file_dialog_mode.is_some() {
                    self.file_dialog_mode = None;
                } else if self.is_inline_graph_editing {
                    self.is_inline_graph_editing = false;
                    self.editor_state = None;
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
                let (final_x, final_y) = if self.snap_to_grid {
                    (
                        (x / crate::features::concept_model::GRID_SIZE).round()
                            * crate::features::concept_model::GRID_SIZE,
                        (y / crate::features::concept_model::GRID_SIZE).round()
                            * crate::features::concept_model::GRID_SIZE,
                    )
                } else {
                    (x, y)
                };
                self.project
                    .concept_graph_mut()
                    .update_node_position(node_id, final_x, final_y);
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
                let to_node = node_options
                    .get(1)
                    .or_else(|| node_options.first())
                    .cloned();
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
                                d.error =
                                    Some("Vælg venligst både kilde og målbegreb.".to_string());
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

            // Lynoprettelse & Node-redigering på Canvas (Task 006)
            Message::CanvasDoubleClicked(x, y) => {
                self.quick_create = Some(QuickCreateState::new(x, y));
                return operation::focus("quick_create_term_input");
            }
            Message::QuickCreateTermChanged(term) => {
                if let Some(qc) = &mut self.quick_create {
                    qc.preferred_term = term;
                    qc.validation_error = None;
                }
            }
            Message::QuickCreateDefinitionChanged(def) => {
                if let Some(qc) = &mut self.quick_create {
                    qc.definition = def;
                    qc.validation_error = None;
                }
            }
            Message::QuickCreateDomainChanged(domain) => {
                if let Some(qc) = &mut self.quick_create {
                    qc.belongs_to_domain = domain;
                }
            }
            Message::QuickCreateCancel => {
                self.quick_create = None;
            }
            Message::QuickCreateSubmit => {
                if let Some(qc) = &mut self.quick_create {
                    match qc.build_concept() {
                        Ok(concept) => {
                            let (x, y) = qc.position;
                            match self.project.add_concept(concept.clone()) {
                                Ok(concept_id) => {
                                    if let Some(node) = self
                                        .project
                                        .concept_graph_mut()
                                        .find_node_by_concept_mut(concept_id)
                                    {
                                        node.set_position(x, y);
                                    }
                                    if let Some(node) = self
                                        .project
                                        .concept_graph()
                                        .find_node_by_concept(concept_id)
                                    {
                                        self.selected_graph_node_id = Some(node.id());
                                    }
                                    self.quick_create = None;
                                    self.trigger_autosave();
                                }
                                Err(err) => {
                                    qc.validation_error = Some(err.to_string());
                                }
                            }
                        }
                        Err(err) => {
                            qc.validation_error = Some(err.to_string());
                        }
                    }
                }
            }
            Message::GraphNodeDoubleClicked(node_id) => {
                self.selected_graph_node_id = Some(node_id);
                if let Some(node) = self.project.concept_graph().find_node(node_id) {
                    if let Some(concept) = self.project.get_concept(node.concept_id()) {
                        self.editor_state = Some(ConceptEditorState::from_concept(concept));
                        self.is_inline_graph_editing = true;
                        return operation::focus("preferred_term_input");
                    }
                }
            }

            // Canvas ergonomi (Task 008)
            Message::CanvasViewportChanged(vp) => {
                self.canvas_viewport = vp;
            }
            Message::ToggleSnapToGrid => {
                self.snap_to_grid = !self.snap_to_grid;
            }
            Message::CanvasZoomIn => {
                self.canvas_viewport.zoom_at(Point::new(500.0, 400.0), 1.15);
            }
            Message::CanvasZoomOut => {
                self.canvas_viewport
                    .zoom_at(Point::new(500.0, 400.0), 1.0 / 1.15);
            }
            Message::CanvasResetView => {
                self.canvas_viewport.reset();
            }
            Message::CanvasSpacePressed(pressed) => {
                self.is_space_pressed = pressed;
            }
            Message::CanvasModifiersChanged(_mods) => {}

            // Informationsmodel (Task 010)
            Message::SelectInformationClass(id) => {
                self.selected_info_class_id = id;
            }
            Message::CreateInformationClass => {
                let class = InformationClass::new("NyKlasse");
                let id = self.project.information_model_mut().add_class(class);
                self.selected_info_class_id = Some(id);
                self.trigger_autosave();
            }
            Message::CreateInformationClassFromConcept(opt) => {
                if let Some(concept) = self.project.get_concept(opt.id).cloned() {
                    let id = self
                        .project
                        .information_model_mut()
                        .create_class_from_concept(&concept);
                    self.selected_info_class_id = Some(id);
                    self.trigger_autosave();
                }
            }
            Message::UpdateInformationClassName(class_id, name) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    class.set_name(name);
                    self.trigger_autosave();
                }
            }
            Message::UpdateInformationClassDescription(class_id, desc) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    class.set_description(if desc.trim().is_empty() {
                        None
                    } else {
                        Some(desc)
                    });
                    self.trigger_autosave();
                }
            }
            Message::AddConceptToInformationClass(class_id, opt) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    class.add_concept_id(opt.id);
                    self.trigger_autosave();
                }
            }
            Message::RemoveConceptFromInformationClass(class_id, concept_id) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    class.remove_concept_id(concept_id);
                    self.trigger_autosave();
                }
            }
            Message::DeleteInformationClass(class_id) => {
                self.project.information_model_mut().remove_class(class_id);
                if self.selected_info_class_id == Some(class_id) {
                    self.selected_info_class_id = None;
                }
                self.trigger_autosave();
            }
            Message::AddAttributeToClass(class_id) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    let attr = Attribute::new(
                        "nyAttribut",
                        PrimitiveType::CharacterString,
                        Multiplicity::exactly_one(),
                    );
                    class.add_attribute(attr);
                    self.trigger_autosave();
                }
            }
            Message::UpdateAttributeName(class_id, attr_id, name) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    if let Some(attr) = class
                        .attributes_mut()
                        .iter_mut()
                        .find(|a| a.id() == attr_id)
                    {
                        attr.set_name(name);
                        self.trigger_autosave();
                    }
                }
            }
            Message::UpdateAttributeType(class_id, attr_id, dt) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    if let Some(attr) = class
                        .attributes_mut()
                        .iter_mut()
                        .find(|a| a.id() == attr_id)
                    {
                        attr.set_data_type(dt);
                        self.trigger_autosave();
                    }
                }
            }
            Message::UpdateAttributeMultiplicity(class_id, attr_id, m) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    if let Some(attr) = class
                        .attributes_mut()
                        .iter_mut()
                        .find(|a| a.id() == attr_id)
                    {
                        attr.set_multiplicity(m);
                        self.trigger_autosave();
                    }
                }
            }
            Message::AddConceptToAttribute(class_id, attr_id, opt) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    if let Some(attr) = class
                        .attributes_mut()
                        .iter_mut()
                        .find(|a| a.id() == attr_id)
                    {
                        attr.add_concept_id(opt.id);
                        self.trigger_autosave();
                    }
                }
            }
            Message::RemoveConceptFromAttribute(class_id, attr_id, concept_id) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    if let Some(attr) = class
                        .attributes_mut()
                        .iter_mut()
                        .find(|a| a.id() == attr_id)
                    {
                        attr.remove_concept_id(concept_id);
                        self.trigger_autosave();
                    }
                }
            }
            Message::DeleteAttribute(class_id, attr_id) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    class.remove_attribute(attr_id);
                    self.trigger_autosave();
                }
            }
            Message::InformationClassSearchChanged(q) => {
                self.info_class_search = q;
            }
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                match key.as_ref() {
                    Key::Named(Named::Tab) => {
                        if modifiers.shift() {
                            Some(Message::FocusPrevious)
                        } else {
                            Some(Message::FocusNext)
                        }
                    }
                    Key::Named(Named::Escape) => Some(Message::EscapePressed),
                    Key::Named(Named::Space) => Some(Message::CanvasSpacePressed(true)),
                    Key::Character(c)
                        if (c == "s" || c == "S")
                            && (modifiers.control() || modifiers.command()) =>
                    {
                        Some(Message::SaveProject)
                    }
                    Key::Character(c)
                        if (c == "+" || c == "=")
                            && (modifiers.control() || modifiers.command()) =>
                    {
                        Some(Message::CanvasZoomIn)
                    }
                    Key::Character(c)
                        if c == "-" && (modifiers.control() || modifiers.command()) =>
                    {
                        Some(Message::CanvasZoomOut)
                    }
                    Key::Character(c)
                        if c == "0" && (modifiers.control() || modifiers.command()) =>
                    {
                        Some(Message::CanvasResetView)
                    }
                    _ => None,
                }
            }
            Event::Keyboard(keyboard::Event::KeyReleased { key, .. }) => {
                if let Key::Named(Named::Space) = key.as_ref() {
                    Some(Message::CanvasSpacePressed(false))
                } else {
                    None
                }
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                Some(Message::CanvasModifiersChanged(modifiers))
            }
            _ => None,
        })
    }

    pub fn view(&self) -> Element<'_, Message> {
        use iced::Color;

        // 1. Desktop Header Bar
        let brand_section = row![
            text("Edge").size(20).color(ThemeColors::PRIMARY),
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
        .align_y(Alignment::Center);

        let tab_item = |tab: Tab, label: &'static str| {
            let is_active = self.active_tab == tab;
            button(text(label).size(12))
                .style(segmented_tab_button(is_active))
                .on_press(Message::SelectTab(tab))
                .padding([5, 12])
        };

        let tab_pill_bar = container(
            row![
                tab_item(Tab::Metadata, "1. Omslag & Metadata"),
                tab_item(Tab::ConceptList, "2. Begrebsliste (Bilag D & E)"),
                tab_item(Tab::ConceptModel, "3. Begrebsmodel (Graf)"),
                tab_item(Tab::InformationModel, "4. Informationsmodel"),
            ]
            .spacing(2)
            .align_y(Alignment::Center),
        )
        .style(pill_container_style)
        .padding(3);

        let actions = row![
            button(text("+ Nyt").size(12))
                .style(secondary_button_style)
                .on_press(Message::NewProject)
                .padding([6, 12]),
            button(text("📁 Åbn...").size(12))
                .style(secondary_button_style)
                .on_press(Message::OpenProjectDialog)
                .padding([6, 12]),
            button(text("💾 Gem").size(12))
                .style(primary_button_style)
                .on_press(Message::SaveProject)
                .padding([6, 14]),
            button(text("Gem som...").size(12))
                .style(secondary_button_style)
                .on_press(Message::SaveProjectAsDialog)
                .padding([6, 12]),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        let header_bar = container(
            row![
                brand_section,
                Space::new().width(Length::FillPortion(1)),
                tab_pill_bar,
                Space::new().width(Length::FillPortion(1)),
                actions,
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .style(card_container_style)
        .padding([8, 16])
        .width(Length::Fill);

        // 2. Modale dialoger (Stack Overlay)
        let maybe_file_dialog_modal: Option<Element<Message>> = self.file_dialog_mode.map(|mode| {
            let (mode_title, confirm_label) = match mode {
                FileDialogMode::Open => ("Åbn FDA Modelprojekt", "Åbn"),
                FileDialogMode::SaveAs => ("Gem Modelprojekt som...", "Gem"),
            };

            let mut dialog_body = column![
                row![
                    text(mode_title).size(16).color(ThemeColors::SLATE_900),
                    Space::new().width(Length::Fill),
                    button(text("✕").size(13))
                        .style(secondary_button_style)
                        .on_press(Message::CloseFileDialog)
                        .padding([3, 7]),
                ]
                .align_y(Alignment::Center),
                text("Angiv filsti eller gennemse filer på maskinen:")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED),
                row![
                    text_input(
                        "Filsti (f.eks. model.edge.json)...",
                        &self.file_dialog_input
                    )
                    .style(modern_input_style)
                    .on_input(Message::FileDialogInputChanged)
                    .on_submit(Message::ConfirmFileDialog)
                    .padding(8)
                    .width(Length::Fill),
                    button(text("🖥️ Gennemse...").size(12))
                        .style(secondary_button_style)
                        .on_press(match mode {
                            FileDialogMode::Open => Message::OpenProjectDialog,
                            FileDialogMode::SaveAs => Message::SaveProjectAsDialog,
                        })
                        .padding([8, 12]),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            ]
            .spacing(12);

            if mode == FileDialogMode::Open {
                let local_files =
                    crate::ui::file_dialog::scan_local_project_files(&PathBuf::from("."));
                if !local_files.is_empty() {
                    let mut chips = column![text("Filer i projektmappen:")
                        .size(12)
                        .color(ThemeColors::TEXT_MUTED)]
                    .spacing(6);

                    let mut chip_row = row![].spacing(6);
                    for f in local_files {
                        let name = f
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("model.edge.json")
                            .to_string();
                        chip_row = chip_row.push(
                            button(text(format!("📄 {}", name)).size(11))
                                .style(secondary_button_style)
                                .on_press(Message::OpenProjectFile(f))
                                .padding([3, 8]),
                        );
                    }
                    chips = chips.push(chip_row);
                    dialog_body = dialog_body.push(chips);
                }
            }

            let actions_row = row![
                Space::new().width(Length::Fill),
                button(text("Annuller").size(12))
                    .style(secondary_button_style)
                    .on_press(Message::CloseFileDialog)
                    .padding([6, 14]),
                button(text(confirm_label).size(12))
                    .style(primary_button_style)
                    .on_press(Message::ConfirmFileDialog)
                    .padding([6, 16]),
            ]
            .spacing(8)
            .align_y(Alignment::Center);

            dialog_body = dialog_body.push(actions_row);

            let modal_card = container(dialog_body)
                .style(modal_card_style)
                .padding(24)
                .width(Length::Fixed(520.0));

            container(modal_card)
                .style(modal_backdrop_style)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        });

        let maybe_relation_modal: Option<Element<Message>> =
            self.relation_dialog.as_ref().map(|d| {
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

                let mut dialog_col = column![row![
                    text("Opret Ny Relation i Begrebsmodel")
                        .size(16)
                        .color(ThemeColors::SLATE_900),
                    Space::new().width(Length::Fill),
                    button(text("✕").size(13))
                        .style(secondary_button_style)
                        .on_press(Message::GraphCloseRelationDialog)
                        .padding([3, 7]),
                ]
                .align_y(Alignment::Center),]
                .spacing(12);

                if let Some(err) = &d.error {
                    dialog_col = dialog_col.push(
                        container(
                            text(format!("⚠️ {}", err))
                                .size(12)
                                .color(ThemeColors::ACCENT_RED),
                        )
                        .style(|_theme| container::Style {
                            background: Some(iced::Background::Color(
                                ThemeColors::ACCENT_RED_LIGHT,
                            )),
                            border: iced::Border {
                                color: ThemeColors::ACCENT_RED,
                                width: 1.0,
                                radius: 6.0.into(),
                            },
                            ..Default::default()
                        })
                        .padding([6, 10])
                        .width(Length::Fill),
                    );
                }

                let from_pick = pick_list(
                    node_options.clone(),
                    d.from_node.clone(),
                    Message::GraphRelationFromChanged,
                )
                .placeholder("Vælg kilde...")
                .padding(7)
                .width(Length::Fixed(220.0));

                let to_pick = pick_list(
                    node_options,
                    d.to_node.clone(),
                    Message::GraphRelationToChanged,
                )
                .placeholder("Vælg mål...")
                .padding(7)
                .width(Length::Fixed(220.0));

                let kind_pick = pick_list(
                    kinds.to_vec(),
                    Some(d.kind),
                    Message::GraphRelationKindChanged,
                )
                .padding(7)
                .width(Length::Fixed(220.0));

                let fields = column![
                    row![
                        column![
                            text("Kildebegreb (fra):")
                                .size(12)
                                .color(ThemeColors::TEXT_MUTED),
                            from_pick
                        ]
                        .spacing(4),
                        column![
                            text("Relationstype:")
                                .size(12)
                                .color(ThemeColors::TEXT_MUTED),
                            kind_pick
                        ]
                        .spacing(4),
                    ]
                    .spacing(16),
                    row![
                        column![
                            text("Målbegreb (til):")
                                .size(12)
                                .color(ThemeColors::TEXT_MUTED),
                            to_pick
                        ]
                        .spacing(4),
                        if d.kind == RelationKind::Association {
                            column![
                                text("Associationsnavn (naturligt sprog):")
                                    .size(12)
                                    .color(ThemeColors::TEXT_MUTED),
                                text_input("f.eks. ejer, anvender...", &d.label)
                                    .style(modern_input_style)
                                    .on_input(Message::GraphRelationLabelChanged)
                                    .padding(7)
                                    .width(Length::Fixed(220.0)),
                            ]
                            .spacing(4)
                        } else {
                            column![
                                text("Generaliseringsregel:")
                                    .size(12)
                                    .color(ThemeColors::TEXT_MUTED),
                                text("Specialisering ➔ Superklasse (hvid pil)")
                                    .size(11)
                                    .color(ThemeColors::SLATE_600),
                            ]
                            .spacing(4)
                        },
                    ]
                    .spacing(16),
                ]
                .spacing(12);

                dialog_col = dialog_col.push(fields);

                let actions = row![
                    Space::new().width(Length::Fill),
                    button(text("Annuller").size(12))
                        .style(secondary_button_style)
                        .on_press(Message::GraphCloseRelationDialog)
                        .padding([6, 14]),
                    button(text("Opret Relation").size(12))
                        .style(primary_button_style)
                        .on_press(Message::GraphCreateRelation)
                        .padding([6, 16]),
                ]
                .spacing(8)
                .align_y(Alignment::Center);

                dialog_col = dialog_col.push(actions);

                let modal_card = container(dialog_col)
                    .style(modal_card_style)
                    .padding(24)
                    .width(Length::Fixed(500.0));

                container(modal_card)
                    .style(modal_backdrop_style)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into()
            });

        let maybe_quick_create_modal: Option<Element<Message>> =
            self.quick_create.as_ref().map(|qc| {
                let mut dialog_col = column![
                    row![
                        text("Nyt Begreb på Lærred")
                            .size(16)
                            .color(ThemeColors::SLATE_900),
                        Space::new().width(Length::Fill),
                        button(text("✕").size(13))
                            .style(secondary_button_style)
                            .on_press(Message::QuickCreateCancel)
                            .padding([3, 7]),
                    ]
                    .align_y(Alignment::Center),
                    text(format!(
                        "Opretter begreb ved ({:.0}, {:.0}) på lærredet jf. FDA Modelreglerne:",
                        qc.position.0, qc.position.1
                    ))
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED),
                ]
                .spacing(12);

                if let Some(err) = &qc.validation_error {
                    dialog_col = dialog_col.push(
                        container(
                            text(format!("⚠️ {}", err))
                                .size(12)
                                .color(ThemeColors::ACCENT_RED),
                        )
                        .style(|_theme| container::Style {
                            background: Some(iced::Background::Color(
                                ThemeColors::ACCENT_RED_LIGHT,
                            )),
                            border: iced::Border {
                                color: ThemeColors::ACCENT_RED,
                                width: 1.0,
                                radius: 6.0.into(),
                            },
                            ..Default::default()
                        })
                        .padding([6, 10])
                        .width(Length::Fill),
                    );
                }

                let term_field = column![
                    text("Foretrukken term *")
                        .size(12)
                        .color(ThemeColors::SLATE_700),
                    text_input("Foretrukken term (f.eks. Godsvogn)...", &qc.preferred_term)
                        .id("quick_create_term_input")
                        .style(modern_input_style)
                        .on_input(Message::QuickCreateTermChanged)
                        .on_submit(Message::QuickCreateSubmit)
                        .padding(8)
                        .width(Length::Fill),
                ]
                .spacing(4);

                let def_field = column![
                    text("Definition (Aristoteles' formel) *")
                        .size(12)
                        .color(ThemeColors::SLATE_700),
                    text_input(
                        "Overordnet begreb + adskillende egenskaber...",
                        &qc.definition
                    )
                    .style(modern_input_style)
                    .on_input(Message::QuickCreateDefinitionChanged)
                    .on_submit(Message::QuickCreateSubmit)
                    .padding(8)
                    .width(Length::Fill),
                ]
                .spacing(4);

                let is_local = qc.belongs_to_domain == BelongsToDomain::Yes;
                let domain_row = row![
                    text("Tilknytning:").size(12).color(ThemeColors::SLATE_700),
                    button(text("Lokalt begreb (FDA Sand)").size(11))
                        .style(if is_local {
                            primary_button_style
                        } else {
                            secondary_button_style
                        })
                        .on_press(Message::QuickCreateDomainChanged(BelongsToDomain::Yes))
                        .padding([4, 8]),
                    button(text("Indlånt begreb (FDA Blå)").size(11))
                        .style(if !is_local {
                            primary_button_style
                        } else {
                            secondary_button_style
                        })
                        .on_press(Message::QuickCreateDomainChanged(BelongsToDomain::No))
                        .padding([4, 8]),
                ]
                .spacing(8)
                .align_y(Alignment::Center);

                dialog_col = dialog_col.push(term_field);
                dialog_col = dialog_col.push(def_field);
                dialog_col = dialog_col.push(domain_row);

                let actions = row![
                    Space::new().width(Length::Fill),
                    button(text("Annuller").size(12))
                        .style(secondary_button_style)
                        .on_press(Message::QuickCreateCancel)
                        .padding([6, 14]),
                    button(text("Opret Begreb").size(12))
                        .style(primary_button_style)
                        .on_press(Message::QuickCreateSubmit)
                        .padding([6, 16]),
                ]
                .spacing(8)
                .align_y(Alignment::Center);

                dialog_col = dialog_col.push(actions);

                let modal_card = container(dialog_col)
                    .style(modal_card_style)
                    .padding(24)
                    .width(Length::Fixed(480.0));

                container(modal_card)
                    .style(modal_backdrop_style)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into()
            });

        // 3. Fane Indhold
        let content: Element<Message> = match self.active_tab {
            Tab::Metadata => {
                let meta = self.project.metadata();
                let meta_card = column![
                    row![
                        text(format!("Model: {}", meta.name()))
                            .size(22)
                            .color(ThemeColors::SLATE_900),
                        Space::new().width(8),
                        container(
                            text(format!("{:?}", meta.status()))
                                .size(11)
                                .color(ThemeColors::PRIMARY)
                        )
                        .style(|_| container::Style {
                            background: Some(iced::Background::Color(ThemeColors::PRIMARY_LIGHT)),
                            border: iced::Border {
                                color: ThemeColors::PRIMARY,
                                width: 0.5,
                                radius: 4.0.into(),
                            },
                            ..Default::default()
                        })
                        .padding([2, 6]),
                    ]
                    .align_y(Alignment::Center),
                    Space::new().height(4),
                    text(meta.description())
                        .size(13)
                        .color(ThemeColors::SLATE_700),
                    Space::new().height(8),
                    row![
                        column![
                            text("Emneområde (§26):")
                                .size(12)
                                .color(ThemeColors::TEXT_MUTED),
                            text(meta.domain_area())
                                .size(13)
                                .color(ThemeColors::SLATE_800),
                        ]
                        .spacing(2)
                        .width(Length::FillPortion(1)),
                        column![
                            text("Ansvarlig organisation:")
                                .size(12)
                                .color(ThemeColors::TEXT_MUTED),
                            text(meta.responsible_org())
                                .size(13)
                                .color(ThemeColors::SLATE_800),
                        ]
                        .spacing(2)
                        .width(Length::FillPortion(1)),
                        column![
                            text("Model-URI:").size(12).color(ThemeColors::TEXT_MUTED),
                            text(meta.uri()).size(13).color(ThemeColors::PRIMARY),
                        ]
                        .spacing(2)
                        .width(Length::FillPortion(1)),
                    ]
                    .spacing(16),
                ]
                .spacing(12);

                meta_card.into()
            }

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
                    row![
                        text("Begrebsmodel").size(18).color(ThemeColors::SLATE_900),
                        Space::new().width(8),
                        container(text("Diagram").size(11).color(ThemeColors::SLATE_600))
                            .style(pill_container_style)
                            .padding([2, 8]),
                    ]
                    .align_y(Alignment::Center),
                    Space::new().width(16),
                    button(text("+ Opret Relation").size(12))
                        .style(primary_button_style)
                        .on_press(Message::GraphOpenRelationDialog)
                        .padding([6, 12]),
                    button(text("🔄 Synkroniser Begreber").size(12))
                        .style(secondary_button_style)
                        .on_press(Message::GraphSyncNodes)
                        .padding([6, 12]),
                    button(
                        text(if self.snap_to_grid {
                            "🧲 Snap: Til"
                        } else {
                            "🧲 Snap: Fra"
                        })
                        .size(12)
                    )
                    .style(secondary_button_style)
                    .on_press(Message::ToggleSnapToGrid)
                    .padding([6, 12]),
                    Space::new().width(4),
                    button(text("-").size(12))
                        .style(secondary_button_style)
                        .on_press(Message::CanvasZoomOut)
                        .padding([6, 10]),
                    container(
                        text(format!(
                            "{}%",
                            (self.canvas_viewport.zoom() * 100.0).round() as i32
                        ))
                        .size(12)
                        .color(ThemeColors::SLATE_700)
                    )
                    .style(pill_container_style)
                    .padding([4, 8]),
                    button(text("+").size(12))
                        .style(secondary_button_style)
                        .on_press(Message::CanvasZoomIn)
                        .padding([6, 10]),
                    button(text("⟲ Nulstil").size(12))
                        .style(secondary_button_style)
                        .on_press(Message::CanvasResetView)
                        .padding([6, 12]),
                    Space::new().width(Length::Fill),
                    container(
                        row![
                            text(format!(
                                "Noder: {}",
                                self.project.concept_graph().node_count()
                            ))
                            .size(12)
                            .color(ThemeColors::SLATE_700),
                            text(" • ").size(12).color(ThemeColors::SLATE_400),
                            text(format!(
                                "Relationer: {}",
                                self.project.concept_graph().edge_count()
                            ))
                            .size(12)
                            .color(ThemeColors::SLATE_700),
                        ]
                        .align_y(Alignment::Center)
                    )
                    .style(pill_container_style)
                    .padding([4, 12]),
                ]
                .spacing(10)
                .align_y(Alignment::Center);

                let canvas_widget = canvas(GraphCanvas::new(
                    self.project.concept_graph(),
                    self.selected_graph_node_id,
                    self.canvas_viewport,
                    self.snap_to_grid,
                    self.is_space_pressed,
                    Message::GraphNodeSelected,
                    Message::GraphNodeMoved,
                    Message::CanvasDoubleClicked,
                    Message::GraphNodeDoubleClicked,
                    Message::CanvasViewportChanged,
                ))
                .width(Length::Fill)
                .height(Length::Fill);

                let inspector_panel: Element<Message> = if let Some(selected_id) =
                    self.selected_graph_node_id
                {
                    if let Some(node) = self.project.concept_graph().find_node(selected_id) {
                        match (self.is_inline_graph_editing, &self.editor_state) {
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
                                                Message::UpdateConceptField(
                                                    ConceptFormField::PreferredTerm,
                                                    v,
                                                )
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
                                                Message::UpdateConceptField(
                                                    ConceptFormField::Definition,
                                                    v,
                                                )
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
                                let concept = self.project.get_concept(node.concept_id());
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

                                let badge = container(
                                    text(badge_text).size(11).color(ThemeColors::SLATE_800),
                                )
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
                                                text("Definition (FDA):")
                                                    .size(11)
                                                    .color(ThemeColors::TEXT_MUTED),
                                                text(c.definition())
                                                    .size(12)
                                                    .color(ThemeColors::SLATE_800),
                                            ]
                                            .spacing(3),
                                        )
                                        .style(card_container_style)
                                        .padding(10)
                                        .width(Length::Fill),
                                    );

                                    if let Some(src) = c.source() {
                                        insp = insp.push(
                                            row![
                                                text("Kilde:")
                                                    .size(11)
                                                    .color(ThemeColors::TEXT_MUTED),
                                                Space::new().width(4),
                                                text(src).size(11).color(ThemeColors::SLATE_700),
                                            ]
                                            .align_y(Alignment::Center),
                                        );
                                    }

                                    insp = insp.push(
                                        row![
                                            button(text("✏️ Hurtigrediger").size(11))
                                                .style(primary_button_style)
                                                .on_press(Message::GraphNodeDoubleClicked(
                                                    selected_id
                                                ))
                                                .padding([5, 8]),
                                            button(text("Begrebsliste").size(11))
                                                .style(secondary_button_style)
                                                .on_press(Message::EditConcept(c.id()))
                                                .padding([5, 8]),
                                        ]
                                        .spacing(6),
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
                                    insp = insp.push(Space::new().height(4));
                                    insp = insp.push(
                                        text("Tilknyttede relationer:")
                                            .size(12)
                                            .color(ThemeColors::PRIMARY),
                                    );

                                    for edge in connected_edges {
                                        let from_node =
                                            self.project.concept_graph().find_node(edge.from());
                                        let to_node =
                                            self.project.concept_graph().find_node(edge.to());
                                        let from_name = from_node.map(|n| n.label()).unwrap_or("?");
                                        let to_name = to_node.map(|n| n.label()).unwrap_or("?");
                                        let desc = match edge.kind() {
                                            RelationKind::Generalization => {
                                                format!("{} ⮞ {}", from_name, to_name)
                                            }
                                            RelationKind::Association => {
                                                if let Some(lbl) = edge.label() {
                                                    format!(
                                                        "{} ──({})── {}",
                                                        from_name, lbl, to_name
                                                    )
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
                                                .on_press(Message::GraphDeleteRelation(
                                                    edge_from, edge_to
                                                ))
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
                            text("💡 Begrebsmodel Inspector")
                                .size(14)
                                .color(ThemeColors::PRIMARY),
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
                    .style(card_container_style)
                    .padding(14)
                    .width(Length::Fixed(290.0))
                    .height(Length::Fill)
                    .into()
                };

                let body = row![
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
                    inspector_panel,
                ]
                .spacing(12)
                .height(Length::Fill);

                column![toolbar, body]
                    .spacing(10)
                    .height(Length::Fill)
                    .into()
            }

            Tab::InformationModel => information_model_view::view(
                self.project.information_model(),
                self.project.concepts(),
                self.selected_info_class_id,
                &self.info_class_search,
            ),
        };

        // 4. Status Bar
        let save_status_text = match &self.save_status {
            SaveStatus::Saved(target) => {
                let full_path =
                    std::fs::canonicalize(target).unwrap_or_else(|_| PathBuf::from(target));
                format!("💾 Gemt: {}", full_path.display())
            }
            SaveStatus::Saving => "⏳ Gemmer...".to_string(),
            SaveStatus::Unsaved => "⚠️ Nyt projekt (ikke gemt til disk - tryk Gem)".to_string(),
            SaveStatus::Error(msg) => format!("❌ Fejl ved gemning: {}", msg),
        };

        let status_bar = row![
            text("FDA Modelregler v2.1 • Klar")
                .size(12)
                .color(ThemeColors::SLATE_500),
            Space::new().width(12),
            text(format!("• {} begreber", self.project.concepts().len()))
                .size(12)
                .color(ThemeColors::SLATE_600),
            Space::new().width(12),
            text(format!("• {}", save_status_text))
                .size(12)
                .color(ThemeColors::TEXT_MUTED),
            Space::new().width(Length::Fill),
            text(format!("Aktiv fane: {:?}", self.active_tab))
                .size(12)
                .color(ThemeColors::SLATE_500),
        ]
        .padding([2, 4])
        .align_y(Alignment::Center);

        // 5. Samlet layout med Stack Overlay
        let main_col = column![
            header_bar,
            container(content)
                .style(card_container_style)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(16),
            status_bar,
        ]
        .spacing(12)
        .width(Length::Fill)
        .height(Length::Fill);

        let base_layout: Element<Message> = container(main_col)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(ThemeColors::SURFACE_BG)),
                ..Default::default()
            })
            .padding(16)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        if let Some(modal) = maybe_file_dialog_modal {
            stack![base_layout, modal].into()
        } else if let Some(modal) = maybe_relation_modal {
            stack![base_layout, modal].into()
        } else if let Some(modal) = maybe_quick_create_modal {
            stack![base_layout, modal].into()
        } else {
            base_layout
        }
    }
}
