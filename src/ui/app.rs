use crate::features::concept_model::{NodeId, RelationKind};
use crate::features::concepts::{BelongsToDomain, Concept, ConceptValidator, ValidationError};
use crate::features::information_model::{
    Attribute, InformationClass, Multiplicity, PrimitiveType,
};
use crate::features::model::storage::ProjectStorage;
use crate::features::model::{ModelMetadata, ModelProject, ModelStatus};
use crate::ui::concept_editor::ConceptEditorState;
use crate::ui::concept_model_view;
use crate::ui::concept_table;
use crate::ui::information_model_view;
use crate::ui::theme::{
    card_container_style, modal_backdrop_style, modal_card_style, modern_input_style,
    pill_container_style, primary_button_style, secondary_button_style, segmented_tab_button,
    ThemeColors,
};
use iced::event::{self, Event};
use iced::keyboard::{self, key::Named, Key};
use iced::widget::{
    button, checkbox, column, container, mouse_area, operation, pick_list, row, stack, text,
    text_input, tooltip, Space,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeConceptOption {
    pub id: Option<Uuid>,
    pub label: String,
}

impl std::fmt::Display for AttributeConceptOption {
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
    pub source_multiplicity: Option<Multiplicity>,
    pub target_multiplicity: Option<Multiplicity>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataField {
    Name,
    Description,
    DomainArea,
    ResponsibleOrg,
    Uri,
    Version,
}

#[derive(Debug, Clone)]
pub struct ModelMetadataModalState {
    pub name: String,
    pub description: String,
    pub status: ModelStatus,
    pub domain_area: String,
    pub responsible_org: String,
    pub uri: String,
    pub version: String,
}

impl ModelMetadataModalState {
    pub fn from_metadata(meta: &ModelMetadata) -> Self {
        Self {
            name: if meta.name() == "Nyt FDA Modelprojekt" {
                String::new()
            } else {
                meta.name().to_string()
            },
            description: if meta.description() == "Beskrivelse af modelprojektet" {
                String::new()
            } else {
                meta.description().to_string()
            },
            status: meta.status(),
            domain_area: if meta.domain_area() == "Emneområde" {
                String::new()
            } else {
                meta.domain_area().to_string()
            },
            responsible_org: if meta.responsible_org() == "Ansvarlig Myndighed" {
                String::new()
            } else {
                meta.responsible_org().to_string()
            },
            uri: if meta.uri() == "https://data.gov.dk/model/core/new-model" {
                String::new()
            } else {
                meta.uri().to_string()
            },
            version: if meta.version() == "0.1.0" {
                String::new()
            } else {
                meta.version().to_string()
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RelayServerPreset {
    #[default]
    Koyeb,
    InternalOrg,
    LocalDocker,
    Custom,
}

impl RelayServerPreset {
    pub const ALL: [RelayServerPreset; 4] = [
        RelayServerPreset::Koyeb,
        RelayServerPreset::InternalOrg,
        RelayServerPreset::LocalDocker,
        RelayServerPreset::Custom,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Koyeb => "Koyeb Cloud (Standard - Frankfurt)",
            Self::InternalOrg => "Intern Organisation",
            Self::LocalDocker => "Lokal Docker (ws://localhost:8080/ws)",
            Self::Custom => "Brugerdefineret URL...",
        }
    }

    pub fn default_url(&self) -> &'static str {
        match self {
            Self::Koyeb => "wss://edge-relay.koyeb.app/ws",
            Self::InternalOrg => "wss://collab.intern.org/ws",
            Self::LocalDocker => "ws://localhost:8080/ws",
            Self::Custom => "",
        }
    }
}

impl std::fmt::Display for RelayServerPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone)]
pub struct StartSessionModalState {
    pub preset: RelayServerPreset,
    pub custom_url: String,
    pub remember_choice: bool,
    pub ticket: crate::features::collab::SessionTicket,
    pub copied: bool,
}

impl StartSessionModalState {
    pub fn new() -> Self {
        let preset = RelayServerPreset::Koyeb;
        let room = crate::features::collab::RoomId::generate();
        let key = crate::features::collab::CollabKey::generate();
        let ticket = crate::features::collab::SessionTicket::new(preset.default_url(), room, key);
        Self {
            preset,
            custom_url: String::new(),
            remember_choice: false,
            ticket,
            copied: false,
        }
    }

    pub fn current_url(&self) -> &str {
        if (self.preset == RelayServerPreset::Custom
            || self.preset == RelayServerPreset::InternalOrg)
            && !self.custom_url.is_empty()
        {
            &self.custom_url
        } else {
            self.preset.default_url()
        }
    }

    pub fn set_preset(&mut self, preset: RelayServerPreset) {
        self.preset = preset;
        let room = self.ticket.room_id.clone();
        let key = self.ticket.key.clone();
        let target_url = if (preset == RelayServerPreset::Custom
            || preset == RelayServerPreset::InternalOrg)
            && !self.custom_url.is_empty()
        {
            self.custom_url.as_str()
        } else {
            preset.default_url()
        };
        self.ticket = crate::features::collab::SessionTicket::new(target_url, room, key);
    }

    pub fn set_custom_url(&mut self, url: String) {
        self.custom_url = url.clone();
        if self.preset == RelayServerPreset::Custom || self.preset == RelayServerPreset::InternalOrg
        {
            let room = self.ticket.room_id.clone();
            let key = self.ticket.key.clone();
            self.ticket = crate::features::collab::SessionTicket::new(url, room, key);
        }
    }
}

impl Default for StartSessionModalState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct JoinSessionModalState {
    pub token_input: String,
    pub parsed_ticket: Option<crate::features::collab::SessionTicket>,
    pub error_message: Option<String>,
}

impl JoinSessionModalState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_token_input(&mut self, token: String) {
        let trimmed = token.trim().to_string();
        self.token_input = token;
        if trimmed.is_empty() {
            self.parsed_ticket = None;
            self.error_message = None;
        } else {
            match crate::features::collab::SessionTicket::from_token(&trimmed) {
                Ok(ticket) => {
                    self.parsed_ticket = Some(ticket);
                    self.error_message = None;
                }
                Err(err) => {
                    self.parsed_ticket = None;
                    self.error_message = Some(format!("Ugyldig sessionskode: {err}"));
                }
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        self.parsed_ticket.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct GuestEndedNoticeModalState {
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tab {
    ConceptList,
    ConceptModel,
    InformationModel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveStatus {
    Saved { path: String, timestamp: String },
    Saving,
    Unsaved,
    Error(String),
}

impl SaveStatus {
    pub fn is_saved(&self) -> bool {
        matches!(self, SaveStatus::Saved { .. })
    }

    pub fn display_text(&self, current_file_path: Option<&PathBuf>) -> String {
        match self {
            SaveStatus::Saved { path, timestamp } => {
                let filename = std::path::Path::new(path)
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or(path);
                format!("💾 Sidst gemt kl. {} • {}", timestamp, filename)
            }
            SaveStatus::Saving => "⏳ Gemmer...".to_string(),
            SaveStatus::Unsaved => {
                if current_file_path.is_some() {
                    "⚠️ Ikke gemte ændringer".to_string()
                } else {
                    "⚠️ Nyt projekt (ikke gemt til disk - tryk Gem)".to_string()
                }
            }
            SaveStatus::Error(msg) => format!("❌ Fejl ved gemning: {}", msg),
        }
    }
}

pub fn current_timestamp() -> String {
    unsafe {
        let mut now: libc::time_t = 0;
        libc::time(&mut now);
        let mut tm: libc::tm = std::mem::zeroed();
        if !libc::localtime_r(&now, &mut tm).is_null() {
            format!("{:02}:{:02}:{:02}", tm.tm_hour, tm.tm_min, tm.tm_sec)
        } else {
            "00:00:00".to_string()
        }
    }
}

pub fn open_browser(url: &str) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn()?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(url).spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(url).spawn()?;
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Unsupported platform",
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileDialogMode {
    Open,
    SaveAs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuType {
    File,
    Help,
    Collab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CollabState {
    #[default]
    None,
    Host,
    Guest,
}

impl CollabState {
    pub fn is_guest(&self) -> bool {
        matches!(self, Self::Guest)
    }

    pub fn is_host(&self) -> bool {
        matches!(self, Self::Host)
    }

    pub fn is_active(&self) -> bool {
        !matches!(self, Self::None)
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectTab(Tab),
    NewProject,

    // Live Kollaborering (Task 027 & Task 028)
    CollabNetworkEventReceived(crate::features::collab::CollabNetworkEvent),
    CollabApplyMutation(Box<crate::features::collab::protocol::ModelMutation>),
    CollabApplySnapshot(Box<crate::features::model::ModelProject>),

    OpenStartSessionModal,
    CloseCollabModal,
    CollabPresetSelected(RelayServerPreset),
    CollabCustomUrlChanged(String),
    CollabToggleRememberPreset(bool),
    CollabCopyTicket,
    CollabStartSession,
    OpenJoinSessionModal,
    CollabJoinTokenChanged(String),
    CollabJoinSession,
    CollabDisconnect,
    CollabGuestDismissEndedModal,

    // Desktop Menulinje & Sidebar Toggle (Task 024)
    ToggleMenu(MenuType),
    CloseMenu,
    ToggleLeftSidebar,

    // Modelomslag & Metadata modal (Task 017)
    OpenMetadataModal,
    CloseMetadataModal,
    SaveMetadataModal,
    UpdateMetadataField(MetadataField, String),
    UpdateMetadataStatus(ModelStatus),

    // Statusbar / Eksterne links (Task 018)
    OpenModelRules,

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
    AddConceptToDiagram(Uuid),
    RemoveConceptFromDiagram(NodeId),
    ConceptModelSearchChanged(String),

    // Lynoprettelse & Node-redigering på Canvas (Task 006)
    CanvasDoubleClicked(f32, f32),
    CreateConceptAtCenter,
    QuickCreateTermChanged(String),
    QuickCreateDefinitionChanged(String),
    QuickCreateDomainChanged(BelongsToDomain),
    QuickCreateSubmit,
    QuickCreateCancel,
    GraphNodeDoubleClicked(NodeId),

    // Begrebsmodel Edge Interaktivitet & Drag-to-Connect (Task 007 & 014)
    GraphEdgeSelected(Option<(NodeId, NodeId)>),
    GraphEdgeCreated(NodeId, NodeId),
    GraphUpdateEdgeKind(NodeId, NodeId, RelationKind),
    GraphUpdateEdgeLabel(NodeId, NodeId, String),
    GraphToggleEdgeDirected(NodeId, NodeId, bool),
    GraphReverseEdge(NodeId, NodeId),
    GraphDeleteSelected,

    // Canvas ergonomi, zoom, pan & grid (Task 008)
    CanvasViewportChanged(crate::ui::graph_canvas::CanvasViewport),
    ToggleSnapToGrid,
    CanvasZoomIn,
    CanvasZoomOut,
    CanvasResetView,
    CanvasSpacePressed(bool),
    CanvasModifiersChanged(iced::keyboard::Modifiers),

    // Informationsmodel (Task 010 & 011)
    SelectInformationClass(Option<Uuid>),
    CreateInformationClass,
    CreateInformationClassAt(f32, f32),
    CreateInformationClassAtCenter,
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
    SetAttributeConcept(Uuid, Uuid, Option<Uuid>),
    DeleteAttribute(Uuid, Uuid),
    InformationClassSearchChanged(String),

    // Informationsmodel Canvas & Studio (Task 011 & 014)
    AddClassToDiagram(Uuid),
    RemoveClassFromDiagram(NodeId),
    UpdateClassNodePosition(NodeId, f32, f32),
    SelectInfoGraphNode(Option<NodeId>),
    AddClassRelation(NodeId, NodeId, RelationKind, Option<String>),
    DeleteClassRelation(NodeId, NodeId),
    InfoCanvasViewportChanged(crate::ui::graph_canvas::CanvasViewport),
    ToggleInfoSnapToGrid,
    InfoCanvasZoomIn,
    InfoCanvasZoomOut,
    InfoCanvasResetView,
    OpenInfoRelationDialog,
    CloseInfoRelationDialog,
    InfoRelationFromChanged(NodeOption),
    InfoRelationToChanged(NodeOption),
    InfoRelationKindChanged(RelationKind),
    InfoRelationLabelChanged(String),
    InfoRelationSourceMultiplicityChanged(Option<Multiplicity>),
    InfoRelationTargetMultiplicityChanged(Option<Multiplicity>),
    InfoCreateRelation,
    InfoEdgeSelected(Option<(NodeId, NodeId)>),
    InfoEdgeCreated(NodeId, NodeId),
    InfoUpdateEdgeKind(NodeId, NodeId, RelationKind),
    InfoUpdateEdgeLabel(NodeId, NodeId, String),
    InfoUpdateEdgeSourceMultiplicity(NodeId, NodeId, Option<Multiplicity>),
    InfoUpdateEdgeTargetMultiplicity(NodeId, NodeId, Option<Multiplicity>),
    InfoToggleEdgeDirected(NodeId, NodeId, bool),
    InfoReverseEdge(NodeId, NodeId),
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
    selected_edge: Option<(NodeId, NodeId)>,
    relation_dialog: Option<RelationDialogState>,
    quick_create: Option<QuickCreateState>,
    is_inline_graph_editing: bool,
    canvas_viewport: crate::ui::graph_canvas::CanvasViewport,
    snap_to_grid: bool,
    is_space_pressed: bool,
    selected_info_class_id: Option<Uuid>,
    selected_info_graph_node_id: Option<NodeId>,
    selected_info_edge: Option<(NodeId, NodeId)>,
    info_class_search: String,
    info_canvas_viewport: crate::ui::graph_canvas::CanvasViewport,
    info_relation_dialog: Option<RelationDialogState>,
    info_snap_to_grid: bool,
    concept_model_search: String,
    metadata_modal: Option<ModelMetadataModalState>,
    active_menu: Option<MenuType>,
    show_left_sidebar: bool,
    collab_state: CollabState,
    collab_channel: Option<crate::features::collab::CollabChannel>,
    collab_key: Option<crate::features::collab::CollabKey>,
    last_node_broadcast: Option<std::time::Instant>,
    start_session_modal: Option<StartSessionModalState>,
    join_session_modal: Option<JoinSessionModalState>,
    guest_ended_notice: Option<GuestEndedNoticeModalState>,
    collab_participant_count: usize,
    collab_connection_status: crate::features::collab::ConnectionStatus,
    collab_seq: std::sync::atomic::AtomicU64,
    collab_last_seen_seq: u64,
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
                    proj.sync_information_graph();
                    return Self {
                        project: proj,
                        active_tab: Tab::ConceptList,
                        editor_state: None,
                        search_query: String::new(),
                        current_file_path: path.clone(),
                        save_status: SaveStatus::Saved {
                            path: p.display().to_string(),
                            timestamp: current_timestamp(),
                        },
                        file_dialog_mode: None,
                        file_dialog_input: String::new(),
                        selected_graph_node_id: None,
                        selected_edge: None,
                        relation_dialog: None,
                        quick_create: None,
                        is_inline_graph_editing: false,
                        canvas_viewport: crate::ui::graph_canvas::CanvasViewport::default(),
                        snap_to_grid: true,
                        is_space_pressed: false,
                        selected_info_class_id: None,
                        selected_info_graph_node_id: None,
                        selected_info_edge: None,
                        info_class_search: String::new(),
                        info_canvas_viewport: crate::ui::graph_canvas::CanvasViewport::default(),
                        info_relation_dialog: None,
                        info_snap_to_grid: true,
                        concept_model_search: String::new(),
                        metadata_modal: None,
                        active_menu: None,
                        show_left_sidebar: true,
                        collab_state: CollabState::None,
                        collab_channel: None,
                        collab_key: None,
                        last_node_broadcast: None,
                        start_session_modal: None,
                        join_session_modal: None,
                        guest_ended_notice: None,
                        collab_participant_count: 1,
                        collab_connection_status:
                            crate::features::collab::ConnectionStatus::Disconnected,
                        collab_seq: std::sync::atomic::AtomicU64::new(0),
                        collab_last_seen_seq: 0,
                    };
                }
            }
        }

        let save_status = match &path {
            Some(p) => SaveStatus::Saved {
                path: p.display().to_string(),
                timestamp: current_timestamp(),
            },
            None => SaveStatus::Unsaved,
        };

        Self {
            project: ModelProject::default(),
            active_tab: Tab::ConceptList,
            editor_state: None,
            search_query: String::new(),
            current_file_path: path,
            save_status,
            file_dialog_mode: None,
            file_dialog_input: String::new(),
            selected_graph_node_id: None,
            selected_edge: None,
            relation_dialog: None,
            quick_create: None,
            is_inline_graph_editing: false,
            canvas_viewport: crate::ui::graph_canvas::CanvasViewport::default(),
            snap_to_grid: true,
            is_space_pressed: false,
            selected_info_class_id: None,
            selected_info_graph_node_id: None,
            selected_info_edge: None,
            info_class_search: String::new(),
            info_canvas_viewport: crate::ui::graph_canvas::CanvasViewport::default(),
            info_relation_dialog: None,
            info_snap_to_grid: true,
            concept_model_search: String::new(),
            metadata_modal: None,
            active_menu: None,
            show_left_sidebar: true,
            collab_state: CollabState::None,
            collab_channel: None,
            collab_key: None,
            last_node_broadcast: None,
            start_session_modal: None,
            join_session_modal: None,
            guest_ended_notice: None,
            collab_participant_count: 1,
            collab_connection_status: crate::features::collab::ConnectionStatus::Disconnected,
            collab_seq: std::sync::atomic::AtomicU64::new(0),
            collab_last_seen_seq: 0,
        }
    }

    pub fn current_file_path(&self) -> Option<&PathBuf> {
        self.current_file_path.as_ref()
    }

    pub fn save_status(&self) -> &SaveStatus {
        &self.save_status
    }

    pub fn footer_status_text(&self) -> String {
        if self.collab_state.is_guest() {
            "👥 Live Session (Guest) - Autosave deaktiveret (Brug 'Gem som...')".to_string()
        } else if self.collab_state.is_host() {
            format!(
                "👑 Live Session (Host) • {}",
                self.save_status
                    .display_text(self.current_file_path.as_ref())
            )
        } else {
            self.save_status
                .display_text(self.current_file_path.as_ref())
        }
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Light
    }

    pub fn active_tab(&self) -> Tab {
        self.active_tab
    }

    pub fn metadata_modal(&self) -> Option<&ModelMetadataModalState> {
        self.metadata_modal.as_ref()
    }

    pub fn active_menu(&self) -> Option<MenuType> {
        self.active_menu
    }

    pub fn is_left_sidebar_visible(&self) -> bool {
        self.show_left_sidebar
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

    pub fn selected_edge(&self) -> Option<(NodeId, NodeId)> {
        self.selected_edge
    }

    pub fn selected_info_class_id(&self) -> Option<Uuid> {
        self.selected_info_class_id
    }

    pub fn selected_info_graph_node_id(&self) -> Option<NodeId> {
        self.selected_info_graph_node_id
    }

    pub fn selected_info_edge(&self) -> Option<(NodeId, NodeId)> {
        self.selected_info_edge
    }

    pub fn concept_model_search(&self) -> &str {
        &self.concept_model_search
    }

    pub fn information_model_search(&self) -> &str {
        &self.info_class_search
    }

    pub fn concept_editor(&self) -> Option<&ConceptEditorState> {
        self.editor_state.as_ref()
    }

    pub fn collab_state(&self) -> CollabState {
        self.collab_state
    }

    pub fn set_collab_state(&mut self, state: CollabState) {
        self.collab_state = state;
    }

    pub fn collab_channel(&self) -> Option<&crate::features::collab::CollabChannel> {
        self.collab_channel.as_ref()
    }

    pub fn set_collab_session(
        &mut self,
        channel: crate::features::collab::CollabChannel,
        key: crate::features::collab::CollabKey,
        role: CollabState,
    ) {
        self.collab_channel = Some(channel);
        self.collab_key = Some(key);
        self.collab_state = role;
        self.collab_last_seen_seq = 0;
    }

    pub fn disconnect_collab(&mut self) {
        if let Some(channel) = &self.collab_channel {
            if self.collab_state.is_host() {
                let _ = channel.send_host_left();
            }
            channel.disconnect();
        }
        self.collab_channel = None;
        self.collab_key = None;
        self.collab_state = CollabState::None;
        self.collab_connection_status = crate::features::collab::ConnectionStatus::Disconnected;
        self.collab_last_seen_seq = 0;
    }

    pub fn start_session_modal(&self) -> Option<&StartSessionModalState> {
        self.start_session_modal.as_ref()
    }

    pub fn join_session_modal(&self) -> Option<&JoinSessionModalState> {
        self.join_session_modal.as_ref()
    }

    pub fn guest_ended_notice(&self) -> Option<&GuestEndedNoticeModalState> {
        self.guest_ended_notice.as_ref()
    }

    pub fn collab_participant_count(&self) -> usize {
        self.collab_participant_count
    }

    pub fn set_collab_participant_count(&mut self, count: usize) {
        self.collab_participant_count = count;
    }

    pub fn collab_connection_status(&self) -> crate::features::collab::ConnectionStatus {
        self.collab_connection_status
    }

    pub fn set_collab_connection_status(
        &mut self,
        status: crate::features::collab::ConnectionStatus,
    ) {
        self.collab_connection_status = status;
    }

    pub fn notify_host_ended_session(&mut self) {
        if self.collab_state.is_guest() {
            self.collab_state = CollabState::None;
            self.collab_channel = None;
            self.collab_key = None;
            self.collab_connection_status = crate::features::collab::ConnectionStatus::Disconnected;
            self.guest_ended_notice = Some(GuestEndedNoticeModalState {
                message: "Værten har afsluttet sessionen. Vil du gemme en kopi af modellen lokalt?"
                    .to_string(),
            });
        }
    }

    pub fn collab_status_summary(&self) -> String {
        match self.collab_state {
            CollabState::None => "Offline".to_string(),
            CollabState::Host => {
                let suffix = if self.collab_participant_count == 1 {
                    "1 deltager".to_string()
                } else {
                    format!("{} deltagere", self.collab_participant_count)
                };
                match self.collab_connection_status {
                    crate::features::collab::ConnectionStatus::Connected => {
                        format!("Live: Vært ({suffix})")
                    }
                    crate::features::collab::ConnectionStatus::Reconnecting => {
                        "Genforbinder...".to_string()
                    }
                    _ => format!("Live: Vært ({})", self.collab_connection_status),
                }
            }
            CollabState::Guest => match self.collab_connection_status {
                crate::features::collab::ConnectionStatus::Connected => {
                    "Live: Gæst (Forbundet til Vært)".to_string()
                }
                crate::features::collab::ConnectionStatus::Reconnecting => {
                    "Genforbinder...".to_string()
                }
                _ => format!("Live: Gæst ({})", self.collab_connection_status),
            },
        }
    }

    pub fn broadcast_mutation(&self, mutation: &crate::features::collab::protocol::ModelMutation) {
        if !self.collab_state.is_active() {
            return;
        }
        if let (Some(channel), Some(key)) = (&self.collab_channel, &self.collab_key) {
            let seq = self
                .collab_seq
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                + 1;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let envelope = crate::features::collab::protocol::CollabEnvelope::new(
                seq,
                now,
                crate::features::collab::protocol::CollabPayload::Mutation(mutation.clone()),
            );
            if let Ok(json_bytes) = serde_json::to_vec(&envelope) {
                if let Ok(encrypted) = crate::features::collab::crypto::encrypt(key, &json_bytes) {
                    let _ = channel.send_mutation(encrypted);
                }
            }
        }
    }

    pub fn broadcast_snapshot(&self) {
        if !self.collab_state.is_active() {
            return;
        }
        if let (Some(channel), Some(key)) = (&self.collab_channel, &self.collab_key) {
            let seq = self
                .collab_seq
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                + 1;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let envelope = crate::features::collab::protocol::CollabEnvelope::new(
                seq,
                now,
                crate::features::collab::protocol::CollabPayload::Snapshot(self.project.clone()),
            );
            if let Ok(json_bytes) = serde_json::to_vec(&envelope) {
                if let Ok(encrypted) = crate::features::collab::crypto::encrypt(key, &json_bytes) {
                    let _ = channel.send_snapshot(encrypted);
                }
            }
        }
    }

    pub fn broadcast_node_moved_throttled(&mut self, id: Uuid, x: f32, y: f32) {
        if !self.collab_state.is_active() {
            return;
        }
        let now = std::time::Instant::now();
        let should_send = match self.last_node_broadcast {
            Some(last) => now.duration_since(last) >= std::time::Duration::from_millis(66), // ~15 Hz
            None => true,
        };
        if should_send {
            self.last_node_broadcast = Some(now);
            self.broadcast_mutation(
                &crate::features::collab::protocol::ModelMutation::NodeMoved { id, x, y },
            );
        }
    }

    pub fn apply_mutation(&mut self, mutation: crate::features::collab::protocol::ModelMutation) {
        use crate::features::collab::protocol::ModelMutation::*;
        match mutation {
            ConceptAdded(concept) => {
                let c_id = concept.id();
                if !self.project.concepts().iter().any(|c| c.id() == c_id) {
                    self.project.concepts_mut().push(concept.clone());
                    if !self.project.concept_graph().is_concept_on_diagram(c_id) {
                        self.project.concept_graph_mut().add_node(&concept);
                    }
                }
            }
            ConceptUpdated(concept) => {
                let c_id = concept.id();
                if let Some(existing) = self
                    .project
                    .concepts_mut()
                    .iter_mut()
                    .find(|c| c.id() == c_id)
                {
                    *existing = concept.clone();
                }
                if let Some(node) = self
                    .project
                    .concept_graph_mut()
                    .find_node_by_concept_mut(c_id)
                {
                    node.set_label(concept.preferred_term().to_string());
                }
            }
            ConceptDeleted(c_id) => {
                self.project.concepts_mut().retain(|c| c.id() != c_id);
                if let Some(node) = self.project.concept_graph().find_node_by_concept(c_id) {
                    let node_id = node.id();
                    self.project.concept_graph_mut().remove_node(node_id);
                }
                if self
                    .selected_graph_node_id
                    .is_some_and(|nid| self.project.concept_graph().find_node(nid).is_none())
                {
                    self.selected_graph_node_id = None;
                }
            }
            InformationClassAdded(class) => {
                let class_id = class.id();
                if !self
                    .project
                    .information_model()
                    .classes()
                    .iter()
                    .any(|c| c.id() == class_id)
                {
                    let attr_count = class.attributes().len();
                    self.project
                        .information_model_mut()
                        .classes_mut()
                        .push(class);
                    if !self
                        .project
                        .information_graph()
                        .is_class_on_diagram(class_id)
                    {
                        self.project
                            .information_graph_mut()
                            .add_node(class_id, attr_count);
                    }
                }
            }
            InformationClassUpdated(class) => {
                let class_id = class.id();
                if let Some(existing) = self
                    .project
                    .information_model_mut()
                    .classes_mut()
                    .iter_mut()
                    .find(|c| c.id() == class_id)
                {
                    *existing = class;
                }
            }
            InformationClassDeleted(class_id) => {
                self.project.remove_information_class(class_id);
                if self.selected_info_class_id == Some(class_id) {
                    self.selected_info_class_id = None;
                }
            }
            RelationAdded(rel) => {
                self.project
                    .concept_graph_mut()
                    .add_relation_full(rel.id, rel.from, rel.to, rel.kind, rel.label);
            }
            RelationDeleted(rel_id) => {
                self.project
                    .concept_graph_mut()
                    .remove_relation_by_id(rel_id);
            }
            NodeMoved { id, x, y } => {
                if let Some(node) = self.project.concept_graph_mut().find_node_mut(id) {
                    node.set_position(x, y);
                } else if let Some(node) = self
                    .project
                    .concept_graph_mut()
                    .find_node_by_concept_mut(id)
                {
                    node.set_position(x, y);
                } else if let Some(class_node) =
                    self.project.information_graph_mut().find_node_mut(id)
                {
                    class_node.set_position(x, y);
                }
            }
        }
        self.trigger_autosave();
    }

    pub fn apply_snapshot(&mut self, snapshot: crate::features::model::ModelProject) {
        self.project = snapshot;
        self.selected_graph_node_id = None;
        self.selected_edge = None;
        self.selected_info_class_id = None;
        self.selected_info_graph_node_id = None;
        self.selected_info_edge = None;
        self.trigger_autosave();
    }

    pub fn trigger_autosave(&mut self) {
        if self.collab_state.is_guest() {
            return;
        }
        if let Some(path) = &self.current_file_path {
            match ProjectStorage::save_to_file(&self.project, path) {
                Ok(()) => {
                    self.save_status = SaveStatus::Saved {
                        path: path.display().to_string(),
                        timestamp: current_timestamp(),
                    };
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
            Message::CollabNetworkEventReceived(event) => {
                match event {
                    crate::features::collab::CollabNetworkEvent::StatusChanged(status) => {
                        self.set_collab_connection_status(status);
                    }
                    crate::features::collab::CollabNetworkEvent::PresenceUpdated(count) => {
                        self.set_collab_participant_count(count);
                    }
                    crate::features::collab::CollabNetworkEvent::HostEndedSession => {
                        self.notify_host_ended_session();
                    }
                    crate::features::collab::CollabNetworkEvent::MessageReceived(bytes) => {
                        if let Some(key) = &self.collab_key {
                            if let Ok(decrypted) =
                                crate::features::collab::crypto::decrypt(key, &bytes)
                            {
                                if let Ok(envelope) = serde_json::from_slice::<
                                    crate::features::collab::protocol::CollabEnvelope,
                                >(&decrypted)
                                {
                                    if envelope.is_newer_than(self.collab_last_seen_seq) {
                                        self.collab_last_seen_seq = envelope.seq;
                                        match envelope.payload {
                                            crate::features::collab::protocol::CollabPayload::Snapshot(proj) => {
                                                self.apply_snapshot(proj);
                                            }
                                            crate::features::collab::protocol::CollabPayload::Mutation(mutation) => {
                                                self.apply_mutation(mutation);
                                            }
                                        }
                                    }
                                } else if let Ok(payload) =
                                    serde_json::from_slice::<
                                        crate::features::collab::protocol::CollabPayload,
                                    >(&decrypted)
                                {
                                    match payload {
                                        crate::features::collab::protocol::CollabPayload::Snapshot(proj) => {
                                            self.apply_snapshot(proj);
                                        }
                                        crate::features::collab::protocol::CollabPayload::Mutation(mutation) => {
                                            self.apply_mutation(mutation);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    crate::features::collab::CollabNetworkEvent::Error(err) => {
                        eprintln!("Kollaborations netværksfejl: {err}");
                    }
                }
            }
            Message::CollabApplyMutation(mutation) => {
                self.apply_mutation(*mutation);
            }
            Message::CollabApplySnapshot(snapshot) => {
                self.apply_snapshot(*snapshot);
            }
            Message::OpenStartSessionModal => {
                if !self.collab_state.is_active() {
                    self.active_menu = None;
                    self.start_session_modal = Some(StartSessionModalState::new());
                }
            }
            Message::CloseCollabModal => {
                self.start_session_modal = None;
                self.join_session_modal = None;
                self.guest_ended_notice = None;
            }
            Message::CollabPresetSelected(preset) => {
                if let Some(modal) = &mut self.start_session_modal {
                    modal.set_preset(preset);
                }
            }
            Message::CollabCustomUrlChanged(url) => {
                if let Some(modal) = &mut self.start_session_modal {
                    modal.set_custom_url(url);
                }
            }
            Message::CollabToggleRememberPreset(rem) => {
                if let Some(modal) = &mut self.start_session_modal {
                    modal.remember_choice = rem;
                }
            }
            Message::CollabCopyTicket => {
                if let Some(modal) = &mut self.start_session_modal {
                    modal.copied = true;
                    let token = modal.ticket.to_token();
                    return iced::clipboard::write(token);
                }
            }
            Message::CollabStartSession => {
                if let Some(modal) = self.start_session_modal.take() {
                    self.active_menu = None;
                    let ticket = modal.ticket;
                    let channel = crate::features::collab::CollabChannel::connect_registered(
                        &ticket.relay_url,
                        &ticket.room_id,
                    );
                    self.set_collab_session(channel, ticket.key, CollabState::Host);
                    self.set_collab_connection_status(
                        crate::features::collab::ConnectionStatus::Connected,
                    );
                    self.set_collab_participant_count(1);
                    self.broadcast_snapshot();
                }
            }
            Message::OpenJoinSessionModal => {
                if !self.collab_state.is_active() {
                    self.active_menu = None;
                    self.join_session_modal = Some(JoinSessionModalState::new());
                }
            }
            Message::CollabJoinTokenChanged(tok) => {
                if let Some(modal) = &mut self.join_session_modal {
                    modal.set_token_input(tok);
                }
            }
            Message::CollabJoinSession => {
                if let Some(modal) = self.join_session_modal.take() {
                    if let Some(ticket) = modal.parsed_ticket {
                        self.active_menu = None;
                        let channel = crate::features::collab::CollabChannel::connect_registered(
                            &ticket.relay_url,
                            &ticket.room_id,
                        );
                        self.set_collab_session(channel, ticket.key, CollabState::Guest);
                        self.set_collab_connection_status(
                            crate::features::collab::ConnectionStatus::Connected,
                        );
                    }
                }
            }

            Message::CollabDisconnect => {
                self.active_menu = None;
                self.disconnect_collab();
            }
            Message::CollabGuestDismissEndedModal => {
                self.guest_ended_notice = None;
            }
            Message::SelectTab(tab) => {
                self.active_tab = tab;
                if tab == Tab::ConceptModel {
                    self.project.sync_concept_graph();
                } else if tab == Tab::InformationModel {
                    self.project.sync_information_graph();
                }
            }
            Message::ToggleMenu(menu) => {
                if self.active_menu == Some(menu) {
                    self.active_menu = None;
                } else {
                    self.active_menu = Some(menu);
                }
            }
            Message::CloseMenu => {
                self.active_menu = None;
            }
            Message::ToggleLeftSidebar => {
                self.show_left_sidebar = !self.show_left_sidebar;
            }
            Message::OpenMetadataModal => {
                self.active_menu = None;
                self.metadata_modal = Some(ModelMetadataModalState::from_metadata(
                    self.project.metadata(),
                ));
            }
            Message::CloseMetadataModal => {
                self.metadata_modal = None;
            }
            Message::SaveMetadataModal => {
                if let Some(state) = self.metadata_modal.take() {
                    let meta = self.project.metadata_mut();
                    let name = if state.name.trim().is_empty() {
                        "Nyt FDA Modelprojekt".to_string()
                    } else {
                        state.name
                    };
                    let version = if state.version.trim().is_empty() {
                        "0.1.0".to_string()
                    } else {
                        state.version
                    };
                    meta.set_name(name);
                    meta.set_description(state.description);
                    meta.set_status(state.status);
                    meta.set_domain_area(state.domain_area);
                    meta.set_responsible_org(state.responsible_org);
                    meta.set_uri(state.uri);
                    meta.set_version(version);
                    self.save_status = SaveStatus::Unsaved;
                }
            }
            Message::UpdateMetadataField(field, val) => {
                if let Some(modal) = &mut self.metadata_modal {
                    match field {
                        MetadataField::Name => modal.name = val,
                        MetadataField::Description => modal.description = val,
                        MetadataField::DomainArea => modal.domain_area = val,
                        MetadataField::ResponsibleOrg => modal.responsible_org = val,
                        MetadataField::Uri => modal.uri = val,
                        MetadataField::Version => modal.version = val,
                    }
                }
            }
            Message::UpdateMetadataStatus(status) => {
                if let Some(modal) = &mut self.metadata_modal {
                    modal.status = status;
                }
            }
            Message::NewProject => {
                self.active_menu = None;
                self.project = ModelProject::default();
                self.active_tab = Tab::ConceptList;
                self.metadata_modal = Some(ModelMetadataModalState::from_metadata(
                    self.project.metadata(),
                ));
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
                self.broadcast_mutation(
                    &crate::features::collab::protocol::ModelMutation::ConceptDeleted(id),
                );
                self.trigger_autosave();
            }
            Message::SaveConcept => {
                if let Some(editor) = &mut self.editor_state {
                    match editor.build_concept() {
                        Ok(concept) => {
                            let concept_id = concept.id();
                            let pref_term = concept.preferred_term().to_string();
                            let is_node_edit = self.is_inline_graph_editing;
                            let is_edit = editor.editing_id.is_some();

                            let result = if is_edit {
                                self.project.update_concept(concept.clone())
                            } else {
                                self.project.add_concept(concept.clone()).map(|_| ())
                            };

                            match result {
                                Ok(()) => {
                                    if is_edit {
                                        self.broadcast_mutation(
                                            &crate::features::collab::protocol::ModelMutation::ConceptUpdated(
                                                concept,
                                            ),
                                        );
                                    } else {
                                        self.broadcast_mutation(
                                            &crate::features::collab::protocol::ModelMutation::ConceptAdded(
                                                concept,
                                            ),
                                        );
                                    }
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
                self.active_menu = None;
                if self.collab_state.is_guest() {
                    return self.update(Message::SaveProjectAsDialog);
                }
                if self.current_file_path.is_some() {
                    self.trigger_autosave();
                } else {
                    return self.update(Message::SaveProjectAsDialog);
                }
            }
            Message::OpenProjectDialog => {
                self.active_menu = None;
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
                self.active_menu = None;
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
                    proj.sync_information_graph();
                    self.project = proj;
                    let display = path.display().to_string();
                    self.current_file_path = Some(path);
                    self.save_status = SaveStatus::Saved {
                        path: display,
                        timestamp: current_timestamp(),
                    };
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
            Message::OpenModelRules => {
                self.active_menu = None;
                let url = "https://arkitektur.digst.dk/node/770";
                if let Err(err) = open_browser(url) {
                    eprintln!("Kunne ikke åbne browser for {}: {}", url, err);
                }
            }

            // Tastaturnavigation & genveje
            Message::FocusNext => {
                return operation::focus_next();
            }
            Message::FocusPrevious => {
                return operation::focus_previous();
            }
            Message::EscapePressed => {
                if self.active_menu.is_some() {
                    self.active_menu = None;
                } else if self.start_session_modal.is_some() {
                    self.start_session_modal = None;
                } else if self.join_session_modal.is_some() {
                    self.join_session_modal = None;
                } else if self.guest_ended_notice.is_some() {
                    self.guest_ended_notice = None;
                } else if self.metadata_modal.is_some() {
                    self.metadata_modal = None;
                } else if self.quick_create.is_some() {
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
                } else if self.selected_edge.is_some() {
                    self.selected_edge = None;
                } else if self.selected_info_edge.is_some() {
                    self.selected_info_edge = None;
                } else if self.selected_graph_node_id.is_some() {
                    self.selected_graph_node_id = None;
                } else if self.selected_info_graph_node_id.is_some() {
                    self.selected_info_graph_node_id = None;
                }
            }

            // Graf-handlinger (Fase 3)
            Message::GraphNodeSelected(node_id) => {
                self.selected_graph_node_id = node_id;
                self.selected_edge = None;
            }
            Message::GraphEdgeSelected(edge) => {
                self.selected_edge = edge;
                if edge.is_some() {
                    self.selected_graph_node_id = None;
                }
            }
            Message::GraphEdgeCreated(from, to) => {
                if from == to {
                    return Task::none();
                }
                let graph = self.project.concept_graph();
                if graph.find_node(from).is_some() && graph.find_node(to).is_some() {
                    if graph.find_edge(from, to).is_none() {
                        self.project.concept_graph_mut().add_relation(
                            from,
                            to,
                            RelationKind::Association,
                        );
                        self.trigger_autosave();
                    }
                    self.selected_graph_node_id = None;
                    self.selected_edge = Some((from, to));
                    return operation::focus("edge_label_input");
                }
            }
            Message::GraphUpdateEdgeKind(from, to, kind) => {
                if self
                    .project
                    .concept_graph_mut()
                    .update_edge_kind(from, to, kind)
                {
                    self.trigger_autosave();
                }
            }
            Message::GraphUpdateEdgeLabel(from, to, label) => {
                let lbl = if label.trim().is_empty() {
                    None
                } else {
                    Some(label)
                };
                if self
                    .project
                    .concept_graph_mut()
                    .update_edge_label(from, to, lbl)
                {
                    self.trigger_autosave();
                }
            }
            Message::GraphToggleEdgeDirected(from, to, directed) => {
                if self
                    .project
                    .concept_graph_mut()
                    .update_edge_directed(from, to, directed)
                {
                    self.trigger_autosave();
                }
            }
            Message::GraphReverseEdge(from, to) => {
                if self.project.concept_graph_mut().reverse_relation(from, to) {
                    self.selected_edge = Some((to, from));
                    self.trigger_autosave();
                }
            }
            Message::GraphDeleteSelected => match self.active_tab {
                Tab::ConceptModel => {
                    if let Some((from, to)) = self.selected_edge.take() {
                        self.project.concept_graph_mut().remove_relation(from, to);
                        self.trigger_autosave();
                    } else if let Some(node_id) = self.selected_graph_node_id.take() {
                        self.project.concept_graph_mut().remove_node(node_id);
                        self.trigger_autosave();
                    }
                }
                Tab::InformationModel => {
                    if let Some((from, to)) = self.selected_info_edge.take() {
                        self.project
                            .information_graph_mut()
                            .remove_relation(from, to);
                        self.trigger_autosave();
                    } else if let Some(node_id) = self.selected_info_graph_node_id.take() {
                        self.project.information_graph_mut().remove_node(node_id);
                        self.trigger_autosave();
                    }
                }
                _ => {}
            },
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
                let cg = self.project.concept_graph_mut();
                cg.update_node_position(node_id, final_x, final_y);

                let nodes = cg.nodes().to_vec();
                let edges = cg.edges().to_vec();
                let routes = crate::ui::edge_router::EdgeRouter::route_edges(&nodes, &edges);
                for r in routes {
                    cg.update_edge_ports(r.from, r.to, Some(r.from_side), Some(r.to_side));
                }
                self.broadcast_node_moved_throttled(node_id, final_x, final_y);
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
                    source_multiplicity: None,
                    target_multiplicity: None,
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
                                let rel = crate::features::collab::protocol::Relation::with_label(
                                    from.id,
                                    to.id,
                                    dialog.kind,
                                    label.clone(),
                                );
                                self.broadcast_mutation(
                                    &crate::features::collab::protocol::ModelMutation::RelationAdded(
                                        rel,
                                    ),
                                );
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
                if self.selected_edge == Some((from, to)) || self.selected_edge == Some((to, from))
                {
                    self.selected_edge = None;
                }
                self.broadcast_mutation(
                    &crate::features::collab::protocol::ModelMutation::RelationDeleted(from),
                );
                self.trigger_autosave();
            }
            Message::GraphSyncNodes => {
                self.project.sync_concept_graph();
                self.trigger_autosave();
            }
            Message::AddConceptToDiagram(concept_id) => {
                if !self
                    .project
                    .concept_graph()
                    .is_concept_on_diagram(concept_id)
                {
                    if let Some(concept) = self.project.get_concept(concept_id).cloned() {
                        let new_id = self.project.concept_graph_mut().add_node(&concept);
                        self.selected_graph_node_id = Some(new_id);
                        self.trigger_autosave();
                    }
                }
            }
            Message::RemoveConceptFromDiagram(node_id) => {
                self.project.concept_graph_mut().remove_node(node_id);
                if self.selected_graph_node_id == Some(node_id) {
                    self.selected_graph_node_id = None;
                }
                self.trigger_autosave();
            }
            Message::ConceptModelSearchChanged(query) => {
                self.concept_model_search = query;
            }

            // Lynoprettelse & Node-redigering på Canvas (Task 006 & 030)
            Message::CanvasDoubleClicked(x, y) => {
                self.quick_create = Some(QuickCreateState::new(x, y));
                return operation::focus("quick_create_term_input");
            }
            Message::CreateConceptAtCenter => {
                let center_world = self.canvas_viewport.to_world(Point::new(500.0, 350.0));
                let cx = center_world.x - crate::features::concept_model::DEFAULT_NODE_WIDTH / 2.0;
                let cy = center_world.y - crate::features::concept_model::DEFAULT_NODE_HEIGHT / 2.0;
                self.quick_create = Some(QuickCreateState::new(cx, cy));
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
                                    self.broadcast_mutation(
                                        &crate::features::collab::protocol::ModelMutation::ConceptAdded(
                                            concept,
                                        ),
                                    );
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

            // Informationsmodel (Task 010 & 011)
            Message::SelectInformationClass(id) => {
                self.selected_info_class_id = id;
                if id.is_some() {
                    self.selected_info_edge = None;
                }
                if let Some(cid) = id {
                    if let Some(node) = self.project.information_graph().find_node_by_class(cid) {
                        self.selected_info_graph_node_id = Some(node.id());
                    } else {
                        self.selected_info_graph_node_id = None;
                    }
                } else {
                    self.selected_info_graph_node_id = None;
                }
            }
            Message::CreateInformationClass => {
                let class = InformationClass::new("");
                let id = self.project.information_model_mut().add_class(class);
                let node_id = self.project.information_graph_mut().add_node(id, 0);
                self.selected_info_class_id = Some(id);
                self.selected_info_graph_node_id = Some(node_id);
                self.selected_info_edge = None;
                if let Some(cls) = self.project.information_model().get_class(id).cloned() {
                    self.broadcast_mutation(
                        &crate::features::collab::protocol::ModelMutation::InformationClassAdded(
                            cls,
                        ),
                    );
                }
                self.trigger_autosave();
            }
            Message::CreateInformationClassAt(x, y) => {
                let class = InformationClass::new("");
                let id = self.project.information_model_mut().add_class(class);
                let (nx, ny) = if self.info_snap_to_grid {
                    (
                        (x / crate::features::concept_model::GRID_SIZE).round()
                            * crate::features::concept_model::GRID_SIZE,
                        (y / crate::features::concept_model::GRID_SIZE).round()
                            * crate::features::concept_model::GRID_SIZE,
                    )
                } else {
                    (x, y)
                };
                let node_id = self
                    .project
                    .information_graph_mut()
                    .add_node_at(id, nx, ny, 0);
                self.selected_info_class_id = Some(id);
                self.selected_info_graph_node_id = Some(node_id);
                self.selected_info_edge = None;
                if let Some(cls) = self.project.information_model().get_class(id).cloned() {
                    self.broadcast_mutation(
                        &crate::features::collab::protocol::ModelMutation::InformationClassAdded(
                            cls,
                        ),
                    );
                }
                self.trigger_autosave();
            }
            Message::CreateInformationClassAtCenter => {
                let center_world = self.info_canvas_viewport.to_world(Point::new(500.0, 350.0));
                let cx = center_world.x
                    - crate::features::information_model::DEFAULT_CLASS_NODE_WIDTH / 2.0;
                let cy = center_world.y
                    - crate::features::information_model::calculate_class_node_height(0) / 2.0;
                return self.update(Message::CreateInformationClassAt(cx, cy));
            }
            Message::CreateInformationClassFromConcept(opt) => {
                self.selected_info_edge = None;
                if let Some(concept) = self.project.get_concept(opt.id).cloned() {
                    let id = self
                        .project
                        .information_model_mut()
                        .create_class_from_concept(&concept);
                    let attr_count = self
                        .project
                        .information_model()
                        .get_class(id)
                        .map(|c| c.attributes().len())
                        .unwrap_or(0);
                    let node_id = self
                        .project
                        .information_graph_mut()
                        .add_node(id, attr_count);
                    self.selected_info_class_id = Some(id);
                    self.selected_info_graph_node_id = Some(node_id);
                    if let Some(cls) = self.project.information_model().get_class(id).cloned() {
                        self.broadcast_mutation(
                            &crate::features::collab::protocol::ModelMutation::InformationClassAdded(
                                cls,
                            ),
                        );
                    }
                    self.trigger_autosave();
                }
            }
            Message::UpdateInformationClassName(class_id, name) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    let final_name = if class.name() == "NyKlasse" && name != "NyKlasse" {
                        name.replace("NyKlasse", "")
                    } else {
                        name
                    };
                    class.set_name(final_name);
                    let cls = class.clone();
                    self.broadcast_mutation(
                        &crate::features::collab::protocol::ModelMutation::InformationClassUpdated(
                            cls,
                        ),
                    );
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
                    let cls = class.clone();
                    self.broadcast_mutation(
                        &crate::features::collab::protocol::ModelMutation::InformationClassUpdated(
                            cls,
                        ),
                    );
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
                self.project.remove_information_class(class_id);
                if self.selected_info_class_id == Some(class_id) {
                    self.selected_info_class_id = None;
                }
                if let Some(nid) = self.selected_info_graph_node_id {
                    if self.project.information_graph().find_node(nid).is_none() {
                        self.selected_info_graph_node_id = None;
                    }
                }
                self.broadcast_mutation(
                    &crate::features::collab::protocol::ModelMutation::InformationClassDeleted(
                        class_id,
                    ),
                );
                self.trigger_autosave();
            }
            Message::AddAttributeToClass(class_id) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    let attr = Attribute::new(
                        "",
                        PrimitiveType::CharacterString,
                        Multiplicity::exactly_one(),
                    );
                    class.add_attribute(attr);
                    let attr_count = class.attributes().len();
                    self.project
                        .information_graph_mut()
                        .update_class_dimensions(class_id, attr_count);
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
                        let final_name = if attr.name() == "nyAttribut" && name != "nyAttribut" {
                            name.replace("nyAttribut", "")
                        } else {
                            name
                        };
                        attr.set_name(final_name);
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
            Message::SetAttributeConcept(class_id, attr_id, maybe_concept_id) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    if let Some(attr) = class
                        .attributes_mut()
                        .iter_mut()
                        .find(|a| a.id() == attr_id)
                    {
                        if let Some(cid) = maybe_concept_id {
                            attr.set_concept_ids(vec![cid]);
                        } else {
                            attr.clear_concept_ids();
                        }
                        self.trigger_autosave();
                    }
                }
            }
            Message::DeleteAttribute(class_id, attr_id) => {
                if let Some(class) = self.project.information_model_mut().get_class_mut(class_id) {
                    class.remove_attribute(attr_id);
                    let attr_count = class.attributes().len();
                    self.project
                        .information_graph_mut()
                        .update_class_dimensions(class_id, attr_count);
                    self.trigger_autosave();
                }
            }
            Message::InformationClassSearchChanged(q) => {
                self.info_class_search = q;
            }

            // Informationsmodel Canvas & Studio (Task 011)
            Message::AddClassToDiagram(class_id) => {
                let attr_count = self
                    .project
                    .information_model()
                    .get_class(class_id)
                    .map(|c| c.attributes().len())
                    .unwrap_or(0);
                let node_id = self
                    .project
                    .information_graph_mut()
                    .add_node(class_id, attr_count);
                self.selected_info_class_id = Some(class_id);
                self.selected_info_graph_node_id = Some(node_id);
                self.trigger_autosave();
            }
            Message::RemoveClassFromDiagram(node_id) => {
                self.project.information_graph_mut().remove_node(node_id);
                if self.selected_info_graph_node_id == Some(node_id) {
                    self.selected_info_graph_node_id = None;
                }
                self.trigger_autosave();
            }
            Message::UpdateClassNodePosition(node_id, x, y) => {
                let ig = self.project.information_graph_mut();
                ig.update_node_position(node_id, x, y);

                use crate::ui::diagram_canvas::{CanvasEdge, CanvasNode};
                let d_nodes: Vec<crate::features::concept_model::DiagramNode> =
                    ig.nodes().iter().map(|n| n.to_diagram_node()).collect();
                let d_edges: Vec<crate::features::concept_model::DiagramEdge> =
                    ig.edges().iter().map(|e| e.to_diagram_edge()).collect();
                let routes = crate::ui::edge_router::EdgeRouter::route_edges(&d_nodes, &d_edges);
                for r in routes {
                    ig.update_edge_ports(r.from, r.to, Some(r.from_side), Some(r.to_side));
                }
                self.broadcast_node_moved_throttled(node_id, x, y);
                self.trigger_autosave();
            }
            Message::SelectInfoGraphNode(node_id_opt) => {
                self.selected_info_graph_node_id = node_id_opt;
                self.selected_info_edge = None;
                if let Some(nid) = node_id_opt {
                    if let Some(node) = self.project.information_graph().find_node(nid) {
                        self.selected_info_class_id = Some(node.class_id());
                    }
                } else {
                    self.selected_info_class_id = None;
                }
            }
            Message::InfoEdgeSelected(edge) => {
                self.selected_info_edge = edge;
                if edge.is_some() {
                    self.selected_info_graph_node_id = None;
                    self.selected_info_class_id = None;
                }
            }
            Message::InfoEdgeCreated(from, to) => {
                if from == to {
                    return Task::none();
                }
                let graph = self.project.information_graph();
                if graph.find_node(from).is_some() && graph.find_node(to).is_some() {
                    if graph.find_edge(from, to).is_none() {
                        self.project.information_graph_mut().add_relation(
                            from,
                            to,
                            RelationKind::Association,
                            None,
                        );
                        self.trigger_autosave();
                    }
                    self.selected_info_graph_node_id = None;
                    self.selected_info_class_id = None;
                    self.selected_info_edge = Some((from, to));
                    return operation::focus("info_edge_label_input");
                }
            }
            Message::InfoUpdateEdgeKind(from, to, kind) => {
                if self
                    .project
                    .information_graph_mut()
                    .update_edge_kind(from, to, kind)
                {
                    self.trigger_autosave();
                }
            }
            Message::InfoUpdateEdgeLabel(from, to, label) => {
                let lbl = if label.trim().is_empty() {
                    None
                } else {
                    Some(label)
                };
                if self
                    .project
                    .information_graph_mut()
                    .update_edge_label(from, to, lbl)
                {
                    self.trigger_autosave();
                }
            }
            Message::InfoUpdateEdgeSourceMultiplicity(from, to, mult) => {
                if self
                    .project
                    .information_graph_mut()
                    .update_edge_source_multiplicity(from, to, mult)
                {
                    self.trigger_autosave();
                }
            }
            Message::InfoUpdateEdgeTargetMultiplicity(from, to, mult) => {
                if self
                    .project
                    .information_graph_mut()
                    .update_edge_target_multiplicity(from, to, mult)
                {
                    self.trigger_autosave();
                }
            }
            Message::InfoToggleEdgeDirected(from, to, directed) => {
                if self
                    .project
                    .information_graph_mut()
                    .update_edge_directed(from, to, directed)
                {
                    self.trigger_autosave();
                }
            }
            Message::InfoReverseEdge(from, to) => {
                if self
                    .project
                    .information_graph_mut()
                    .reverse_relation(from, to)
                {
                    self.selected_info_edge = Some((to, from));
                    self.trigger_autosave();
                }
            }
            Message::AddClassRelation(from, to, kind, label) => {
                self.project
                    .information_graph_mut()
                    .add_relation(from, to, kind, label);
                self.trigger_autosave();
            }
            Message::DeleteClassRelation(from, to) => {
                self.project
                    .information_graph_mut()
                    .remove_relation(from, to);
                if self.selected_info_edge == Some((from, to))
                    || self.selected_info_edge == Some((to, from))
                {
                    self.selected_info_edge = None;
                }
                self.trigger_autosave();
            }
            Message::InfoCanvasViewportChanged(vp) => {
                self.info_canvas_viewport = vp;
            }
            Message::ToggleInfoSnapToGrid => {
                self.info_snap_to_grid = !self.info_snap_to_grid;
            }
            Message::InfoCanvasZoomIn => {
                self.info_canvas_viewport
                    .zoom_at(Point::new(500.0, 400.0), 1.15);
            }
            Message::InfoCanvasZoomOut => {
                self.info_canvas_viewport
                    .zoom_at(Point::new(500.0, 400.0), 1.0 / 1.15);
            }
            Message::InfoCanvasResetView => {
                self.info_canvas_viewport.reset();
            }
            Message::OpenInfoRelationDialog => {
                let default_from = self.selected_info_graph_node_id.and_then(|id| {
                    self.project.information_graph().find_node(id).map(|n| {
                        let name = self
                            .project
                            .information_model()
                            .get_class(n.class_id())
                            .map(|c| c.name().to_string())
                            .unwrap_or_else(|| "Klasse".to_string());
                        NodeOption { id, label: name }
                    })
                });

                self.info_relation_dialog = Some(RelationDialogState {
                    from_node: default_from,
                    to_node: None,
                    kind: RelationKind::Generalization,
                    label: String::new(),
                    source_multiplicity: None,
                    target_multiplicity: None,
                    error: None,
                });
            }
            Message::CloseInfoRelationDialog => {
                self.info_relation_dialog = None;
            }
            Message::InfoRelationFromChanged(opt) => {
                if let Some(dlg) = &mut self.info_relation_dialog {
                    dlg.from_node = Some(opt);
                    dlg.error = None;
                }
            }
            Message::InfoRelationToChanged(opt) => {
                if let Some(dlg) = &mut self.info_relation_dialog {
                    dlg.to_node = Some(opt);
                    dlg.error = None;
                }
            }
            Message::InfoRelationKindChanged(kind) => {
                if let Some(dlg) = &mut self.info_relation_dialog {
                    dlg.kind = kind;
                }
            }
            Message::InfoRelationLabelChanged(label) => {
                if let Some(dlg) = &mut self.info_relation_dialog {
                    dlg.label = label;
                }
            }
            Message::InfoRelationSourceMultiplicityChanged(mult) => {
                if let Some(dlg) = &mut self.info_relation_dialog {
                    dlg.source_multiplicity = mult;
                }
            }
            Message::InfoRelationTargetMultiplicityChanged(mult) => {
                if let Some(dlg) = &mut self.info_relation_dialog {
                    dlg.target_multiplicity = mult;
                }
            }
            Message::InfoCreateRelation => {
                if let Some(dlg) = &self.info_relation_dialog {
                    match (&dlg.from_node, &dlg.to_node) {
                        (Some(from), Some(to)) => {
                            if from.id == to.id {
                                if let Some(d) = &mut self.info_relation_dialog {
                                    d.error = Some(
                                        "En klasse kan ikke have en relation til sig selv"
                                            .to_string(),
                                    );
                                }
                            } else {
                                let label = if dlg.label.trim().is_empty() {
                                    None
                                } else {
                                    Some(dlg.label.trim().to_string())
                                };
                                let (src_mult, tgt_mult) =
                                    if dlg.kind == RelationKind::Generalization {
                                        (None, None)
                                    } else {
                                        (dlg.source_multiplicity, dlg.target_multiplicity)
                                    };
                                self.project
                                    .information_graph_mut()
                                    .add_relation_with_multiplicities(
                                        from.id, to.id, dlg.kind, label, src_mult, tgt_mult,
                                    );
                                self.info_relation_dialog = None;
                                self.trigger_autosave();
                            }
                        }
                        _ => {
                            if let Some(d) = &mut self.info_relation_dialog {
                                d.error = Some(
                                    "Vælg venligst både en kilde- og destinationsklasse"
                                        .to_string(),
                                );
                            }
                        }
                    }
                }
            }
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let keyboard_sub = event::listen_with(|event, status, _window| {
            if let Event::Window(iced::window::Event::Unfocused) = event {
                return Some(Message::CanvasSpacePressed(false));
            }
            if status == event::Status::Captured {
                return None;
            }
            match event {
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
                        Key::Named(Named::Delete) | Key::Named(Named::Backspace) => {
                            Some(Message::GraphDeleteSelected)
                        }
                        Key::Named(Named::Space) => Some(Message::CanvasSpacePressed(true)),
                        Key::Character(c)
                            if (c == "s" || c == "S")
                                && (modifiers.control() || modifiers.command()) =>
                        {
                            Some(Message::SaveProject)
                        }
                        Key::Character(c)
                            if (c == "b" || c == "B")
                                && (modifiers.control() || modifiers.command()) =>
                        {
                            Some(Message::ToggleLeftSidebar)
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
            }
        });

        if let Some(channel) = &self.collab_channel {
            let sub_id = channel.sub_id();
            let collab_sub = Subscription::run_with(sub_id, |&sub_id| {
                iced::futures::stream::unfold(sub_id, |sub_id| async move {
                    let event =
                        crate::features::collab::network::next_registered_collab_event(sub_id)
                            .await?;
                    Some((Message::CollabNetworkEventReceived(event), sub_id))
                })
            });
            Subscription::batch([keyboard_sub, collab_sub])
        } else {
            keyboard_sub
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // 1. Desktop Header Bar
        let sidebar_toggle_btn = tooltip(
            button(
                text(if self.show_left_sidebar { "◨" } else { "⬚" })
                    .size(16)
                    .color(if self.show_left_sidebar {
                        ThemeColors::PRIMARY
                    } else {
                        ThemeColors::SLATE_500
                    }),
            )
            .style(secondary_button_style)
            .on_press(Message::ToggleLeftSidebar)
            .padding([4, 8]),
            text("Skjul/vis venstre palet (Ctrl+B)").size(11),
            tooltip::Position::Bottom,
        );

        let menu_button = |label: &'static str, menu_type: MenuType| {
            let is_active = self.active_menu == Some(menu_type);
            button(
                row![
                    text(label).size(13).color(if is_active {
                        ThemeColors::PRIMARY
                    } else {
                        ThemeColors::SLATE_800
                    }),
                    Space::new().width(3),
                    text("▾").size(10).color(if is_active {
                        ThemeColors::PRIMARY
                    } else {
                        ThemeColors::SLATE_400
                    }),
                ]
                .align_y(Alignment::Center),
            )
            .style(if is_active {
                primary_button_style
            } else {
                secondary_button_style
            })
            .on_press(Message::ToggleMenu(menu_type))
            .padding([4, 10])
        };

        let brand_section = row![
            sidebar_toggle_btn,
            Space::new().width(10),
            text("Edge").size(18).color(ThemeColors::PRIMARY),
            Space::new().width(4),
            container(text("FDA v2.1").size(10).color(ThemeColors::PRIMARY))
                .style(|_theme: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(ThemeColors::PRIMARY_LIGHT)),
                    border: iced::Border {
                        color: ThemeColors::PRIMARY,
                        width: 1.0,
                        radius: 10.0.into(),
                    },
                    ..Default::default()
                })
                .padding([2, 6]),
            Space::new().width(12),
            menu_button("Filer", MenuType::File),
            Space::new().width(4),
            menu_button("Hjælp", MenuType::Help),
            Space::new().width(4),
            menu_button("Samarbejde", MenuType::Collab),
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
                tab_item(Tab::ConceptList, "1. Begrebsliste"),
                tab_item(Tab::ConceptModel, "2. Begrebsmodel (Graf)"),
                tab_item(Tab::InformationModel, "3. Informationsmodel"),
            ]
            .spacing(2)
            .align_y(Alignment::Center),
        )
        .style(pill_container_style)
        .padding(3);

        let collab_header_badge: Element<Message> = if self.collab_state.is_active() {
            let (bg_color, dot_color) = match self.collab_connection_status {
                crate::features::collab::ConnectionStatus::Connected => (
                    iced::Color::from_rgb(0.92, 0.98, 0.94),
                    iced::Color::from_rgb(0.12, 0.65, 0.35),
                ),
                crate::features::collab::ConnectionStatus::Reconnecting => (
                    iced::Color::from_rgb(1.0, 0.97, 0.88),
                    iced::Color::from_rgb(0.85, 0.55, 0.10),
                ),
                _ => (
                    iced::Color::from_rgb(0.95, 0.95, 0.96),
                    iced::Color::from_rgb(0.55, 0.60, 0.68),
                ),
            };

            container(
                row![
                    text("●").size(10).color(dot_color),
                    Space::new().width(4),
                    text(self.collab_status_summary())
                        .size(11)
                        .color(ThemeColors::SLATE_800),
                ]
                .align_y(Alignment::Center),
            )
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(bg_color)),
                border: iced::Border {
                    color: dot_color,
                    width: 1.0,
                    radius: 12.0.into(),
                },
                ..Default::default()
            })
            .padding([3, 10])
            .into()
        } else {
            Space::new().width(0).height(0).into()
        };

        let header_right = row![
            collab_header_badge,
            Space::new().width(8),
            text(self.project.metadata().name())
                .size(12)
                .color(ThemeColors::SLATE_600),
        ]
        .align_y(Alignment::Center);

        let header_bar = container(
            row![
                container(brand_section)
                    .width(Length::FillPortion(1))
                    .align_x(Alignment::Start),
                tab_pill_bar,
                container(header_right)
                    .width(Length::FillPortion(1))
                    .align_x(Alignment::End),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .style(card_container_style)
        .padding([8, 16])
        .width(Length::Fill);

        // 2. Modale dialoger (Stack Overlay)
        let maybe_metadata_modal: Option<Element<Message>> =
            self.metadata_modal.as_ref().map(|meta_state| {
                let title_row = row![
                    text("📋 Modelomslag & Metadata")
                        .size(17)
                        .color(ThemeColors::SLATE_900),
                    Space::new().width(Length::Fill),
                    button(text("✕").size(13))
                        .style(secondary_button_style)
                        .on_press(Message::CloseMetadataModal)
                        .padding([3, 7]),
                ]
                .align_y(Alignment::Center);

                let subtitle =
                    text("Rediger overordnede metadata for modelprojektet jf. FDA Modelreglerne:")
                        .size(12)
                        .color(ThemeColors::TEXT_MUTED);

                // 1. Modelnavn
                let name_field = column![
                    text("Modelnavn *").size(12).color(ThemeColors::SLATE_700),
                    text_input("Nyt FDA Modelprojekt", &meta_state.name)
                        .style(modern_input_style)
                        .on_input(|val| Message::UpdateMetadataField(MetadataField::Name, val))
                        .padding(8)
                        .width(Length::Fill),
                ]
                .spacing(4);

                // 2. Beskrivelse
                let desc_field = column![
                    text("Beskrivelse").size(12).color(ThemeColors::SLATE_700),
                    text_input(
                        "Formål og omfang jf. FDA Modelreglerne...",
                        &meta_state.description
                    )
                    .style(modern_input_style)
                    .on_input(|val| Message::UpdateMetadataField(MetadataField::Description, val))
                    .padding(8)
                    .width(Length::Fill),
                ]
                .spacing(4);

                // 3. Status picklist & Version
                let status_pick = pick_list(
                    &ModelStatus::ALL[..],
                    Some(meta_state.status),
                    Message::UpdateMetadataStatus,
                )
                .padding(7)
                .width(Length::Fill);

                let status_field = column![
                    text("Modelstatus").size(12).color(ThemeColors::SLATE_700),
                    status_pick,
                ]
                .spacing(4)
                .width(Length::FillPortion(1));

                let version_field = column![
                    text("Version").size(12).color(ThemeColors::SLATE_700),
                    text_input("0.1.0", &meta_state.version)
                        .style(modern_input_style)
                        .on_input(|val| Message::UpdateMetadataField(MetadataField::Version, val))
                        .padding(8)
                        .width(Length::Fill),
                ]
                .spacing(4)
                .width(Length::FillPortion(1));

                let status_version_row = row![status_field, version_field].spacing(12);

                // 4. Emneområde (§26) & Ansvarlig organisation
                let domain_field = column![
                    text("Emneområde (§26)")
                        .size(12)
                        .color(ThemeColors::SLATE_700),
                    text_input("f.eks. Vej og Trafik", &meta_state.domain_area)
                        .style(modern_input_style)
                        .on_input(|val| Message::UpdateMetadataField(
                            MetadataField::DomainArea,
                            val
                        ))
                        .padding(8)
                        .width(Length::Fill),
                ]
                .spacing(4)
                .width(Length::FillPortion(1));

                let org_field = column![
                    text("Ansvarlig organisation")
                        .size(12)
                        .color(ThemeColors::SLATE_700),
                    text_input(
                        "f.eks. Styrelsen for Dataforsyning eller Vejdirektoratet",
                        &meta_state.responsible_org,
                    )
                    .style(modern_input_style)
                    .on_input(|val| Message::UpdateMetadataField(
                        MetadataField::ResponsibleOrg,
                        val
                    ))
                    .padding(8)
                    .width(Length::Fill),
                ]
                .spacing(4)
                .width(Length::FillPortion(1));

                let domain_org_row = row![domain_field, org_field].spacing(12);

                // 5. Model-URI
                let uri_field = column![
                    text("Model-URI").size(12).color(ThemeColors::SLATE_700),
                    text_input("https://data.gov.dk/model/core/...", &meta_state.uri)
                        .style(modern_input_style)
                        .on_input(|val| Message::UpdateMetadataField(MetadataField::Uri, val))
                        .padding(8)
                        .width(Length::Fill),
                ]
                .spacing(4);

                // 6. Action knapper
                let actions = row![
                    Space::new().width(Length::Fill),
                    button(text("Annuller").size(12))
                        .style(secondary_button_style)
                        .on_press(Message::CloseMetadataModal)
                        .padding([6, 14]),
                    button(text("💾 Gem Omslag").size(12))
                        .style(primary_button_style)
                        .on_press(Message::SaveMetadataModal)
                        .padding([6, 16]),
                ]
                .spacing(8)
                .align_y(Alignment::Center);

                let dialog_col = column![
                    title_row,
                    subtitle,
                    name_field,
                    desc_field,
                    status_version_row,
                    domain_org_row,
                    uri_field,
                    actions,
                ]
                .spacing(12);

                let modal_card = container(dialog_col)
                    .style(modal_card_style)
                    .padding(24)
                    .width(Length::Fixed(560.0));

                container(modal_card)
                    .style(modal_backdrop_style)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into()
            });

        let maybe_start_session_modal: Option<Element<Message>> =
            self.start_session_modal.as_ref().map(|modal_state| {
                let title_row = row![
                    text("🌐 Start Live Session (Vært)")
                        .size(17)
                        .color(ThemeColors::SLATE_900),
                    Space::new().width(Length::Fill),
                    button(text("✕").size(13))
                        .style(secondary_button_style)
                        .on_press(Message::CloseCollabModal)
                        .padding([3, 7]),
                ]
                .align_y(Alignment::Center);

                let subtitle = text("Start en end-to-end krypteret (E2EE) samarbejdssession. Vælg relay-server og del sessionskoden med dine deltagere:")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED);

                let preset_pick = pick_list(
                    &RelayServerPreset::ALL[..],
                    Some(modal_state.preset),
                    Message::CollabPresetSelected,
                )
                .padding(7)
                .width(Length::Fill);

                let preset_field = column![
                    text("Relay Server Preset").size(12).color(ThemeColors::SLATE_700),
                    preset_pick,
                ]
                .spacing(4);

                let custom_url_field = if modal_state.preset == RelayServerPreset::Custom
                    || modal_state.preset == RelayServerPreset::InternalOrg
                {
                    column![
                        text("Server WebSocket URL").size(12).color(ThemeColors::SLATE_700),
                        text_input("wss://relay.eksempel.dk/ws", &modal_state.custom_url)
                            .style(modern_input_style)
                            .on_input(Message::CollabCustomUrlChanged)
                            .padding(8)
                            .width(Length::Fill),
                    ]
                    .spacing(4)
                } else {
                    column![
                        text("Aktiv Server URL").size(12).color(ThemeColors::SLATE_500),
                        container(
                            text(modal_state.current_url())
                                .size(12)
                                .color(ThemeColors::SLATE_700)
                        )
                        .style(|_| container::Style {
                            background: Some(iced::Background::Color(ThemeColors::SURFACE_BG)),
                            border: iced::Border {
                                color: ThemeColors::SURFACE_BORDER,
                                width: 1.0,
                                radius: 6.0.into(),
                            },
                            ..Default::default()
                        })
                        .padding([8, 10])
                        .width(Length::Fill),
                    ]
                    .spacing(4)
                };

                let remember_checkbox = row![
                    checkbox(modal_state.remember_choice)
                        .on_toggle(Message::CollabToggleRememberPreset)
                        .size(16),
                    Space::new().width(6),
                    text("Husk servervalg til fremtidige sessioner")
                        .size(12)
                        .color(ThemeColors::SLATE_700),
                ]
                .align_y(Alignment::Center);

                let token_str = modal_state.ticket.to_token();
                let token_box = column![
                    text("Sessionskode (Sessionsbillet med E2EE-nøgle)")
                        .size(12)
                        .color(ThemeColors::SLATE_700),
                    container(
                        text(token_str)
                            .size(11)
                            .color(ThemeColors::PRIMARY)
                    )
                    .style(|_| container::Style {
                        background: Some(iced::Background::Color(ThemeColors::PRIMARY_LIGHT)),
                        border: iced::Border {
                            color: ThemeColors::PRIMARY,
                            width: 1.0,
                            radius: 6.0.into(),
                        },
                        ..Default::default()
                    })
                    .padding(8)
                    .width(Length::Fill),
                ]
                .spacing(4);

                let copy_label = if modal_state.copied {
                    "✓ Kopieret!"
                } else {
                    "📋 Kopiér sessionskode"
                };

                let copy_btn = button(text(copy_label).size(12))
                    .style(if modal_state.copied {
                        primary_button_style
                    } else {
                        secondary_button_style
                    })
                    .on_press(Message::CollabCopyTicket)
                    .padding([6, 14]);

                let actions = row![
                    copy_btn,
                    Space::new().width(Length::Fill),
                    button(text("Annuller").size(12))
                        .style(secondary_button_style)
                        .on_press(Message::CloseCollabModal)
                        .padding([6, 14]),
                    button(text("🚀 Start Session").size(12))
                        .style(primary_button_style)
                        .on_press(Message::CollabStartSession)
                        .padding([6, 16]),
                ]
                .spacing(8)
                .align_y(Alignment::Center);

                let dialog_col = column![
                    title_row,
                    subtitle,
                    preset_field,
                    custom_url_field,
                    remember_checkbox,
                    token_box,
                    actions,
                ]
                .spacing(12);

                let modal_card = container(dialog_col)
                    .style(modal_card_style)
                    .padding(24)
                    .width(Length::Fixed(560.0));

                container(modal_card)
                    .style(modal_backdrop_style)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into()
            });

        let maybe_join_session_modal: Option<Element<Message>> =
            self.join_session_modal.as_ref().map(|modal_state| {
                let title_row = row![
                    text("🌐 Deltag i Live Session (Gæst)")
                        .size(17)
                        .color(ThemeColors::SLATE_900),
                    Space::new().width(Length::Fill),
                    button(text("✕").size(13))
                        .style(secondary_button_style)
                        .on_press(Message::CloseCollabModal)
                        .padding([3, 7]),
                ]
                .align_y(Alignment::Center);

                let subtitle = text("Indsæt den sessionskode du har modtaget fra sessionens vært:")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED);

                let validation_feedback: Element<Message> =
                    if let Some(ticket) = &modal_state.parsed_ticket {
                        container(
                            row![
                                text("✓")
                                    .size(14)
                                    .color(iced::Color::from_rgb(0.12, 0.65, 0.35)),
                                Space::new().width(6),
                                text(format!(
                                    "Gyldig kode (Server: {} | Rum: {})",
                                    ticket.relay_url,
                                    ticket.room_id.as_str()
                                ))
                                .size(12)
                                .color(iced::Color::from_rgb(0.12, 0.65, 0.35)),
                            ]
                            .align_y(Alignment::Center),
                        )
                        .padding([4, 8])
                        .into()
                    } else if let Some(err) = &modal_state.error_message {
                        container(
                            row![
                                text("⚠️").size(12),
                                Space::new().width(6),
                                text(err)
                                    .size(12)
                                    .color(iced::Color::from_rgb(0.85, 0.20, 0.20)),
                            ]
                            .align_y(Alignment::Center),
                        )
                        .padding([4, 8])
                        .into()
                    } else {
                        container(
                            text("Format: edge:v1:<base64-payload>")
                                .size(11)
                                .color(ThemeColors::TEXT_MUTED),
                        )
                        .padding([4, 8])
                        .into()
                    };

                let token_field = column![
                    text("Sessionskode *")
                        .size(12)
                        .color(ThemeColors::SLATE_700),
                    text_input(
                        "Indsæt sessionskode her (f.eks. edge:v1:...)",
                        &modal_state.token_input
                    )
                    .style(modern_input_style)
                    .on_input(Message::CollabJoinTokenChanged)
                    .padding(8)
                    .width(Length::Fill),
                    validation_feedback,
                ]
                .spacing(4);

                let mut join_btn = button(text("🔗 Forbind").size(12))
                    .style(primary_button_style)
                    .padding([6, 16]);

                if modal_state.is_valid() {
                    join_btn = join_btn.on_press(Message::CollabJoinSession);
                }

                let actions = row![
                    Space::new().width(Length::Fill),
                    button(text("Annuller").size(12))
                        .style(secondary_button_style)
                        .on_press(Message::CloseCollabModal)
                        .padding([6, 14]),
                    join_btn,
                ]
                .spacing(8)
                .align_y(Alignment::Center);

                let dialog_col = column![title_row, subtitle, token_field, actions,].spacing(14);

                let modal_card = container(dialog_col)
                    .style(modal_card_style)
                    .padding(24)
                    .width(Length::Fixed(540.0));

                container(modal_card)
                    .style(modal_backdrop_style)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into()
            });

        let maybe_guest_ended_modal: Option<Element<Message>> =
            self.guest_ended_notice.as_ref().map(|notice| {
                let title_row = row![
                    text("⚠️ Live Session Afsluttet")
                        .size(17)
                        .color(ThemeColors::SLATE_900),
                    Space::new().width(Length::Fill),
                    button(text("✕").size(13))
                        .style(secondary_button_style)
                        .on_press(Message::CollabGuestDismissEndedModal)
                        .padding([3, 7]),
                ]
                .align_y(Alignment::Center);

                let notice_text = text(&notice.message)
                    .size(13)
                    .color(ThemeColors::SLATE_700);

                let explanation = text("Værten har afsluttet sessionen. Du har modellen i hukommelsen (RAM) og kan gemme en kopi som en ny lokal modelprojektfil før sessionen lukkes.")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED);

                let actions = row![
                    Space::new().width(Length::Fill),
                    button(text("Luk uden at gemme").size(12))
                        .style(secondary_button_style)
                        .on_press(Message::CollabGuestDismissEndedModal)
                        .padding([6, 14]),
                    button(text("💾 Gem som kopi...").size(12))
                        .style(primary_button_style)
                        .on_press(Message::SaveProjectAsDialog)
                        .padding([6, 16]),
                ]
                .spacing(8)
                .align_y(Alignment::Center);

                let dialog_col = column![
                    title_row,
                    notice_text,
                    explanation,
                    actions,
                ]
                .spacing(14);

                let modal_card = container(dialog_col)
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

        let maybe_menu_overlay: Option<Element<Message>> = self.active_menu.map(|menu_type| {
            let menu_item = |icon: &'static str, label: &'static str, msg: Message| {
                button(
                    row![
                        text(icon).size(14),
                        Space::new().width(10),
                        text(label).size(13).color(ThemeColors::SLATE_800),
                    ]
                    .align_y(Alignment::Center),
                )
                .style(|_theme, status| {
                    let background = match status {
                        button::Status::Hovered => {
                            Some(iced::Background::Color(ThemeColors::PRIMARY_LIGHT))
                        }
                        button::Status::Pressed => {
                            Some(iced::Background::Color(ThemeColors::SURFACE_BORDER))
                        }
                        _ => None,
                    };
                    button::Style {
                        background,
                        text_color: ThemeColors::SLATE_800,
                        border: iced::Border {
                            radius: 6.0.into(),
                            ..Default::default()
                        },
                        shadow: iced::Shadow::default(),
                        ..Default::default()
                    }
                })
                .on_press(msg)
                .padding([7, 10])
                .width(Length::Fill)
            };

            let make_separator = || {
                container(Space::new().width(Length::Fill).height(Length::Fixed(1.0))).style(|_| {
                    container::Style {
                        background: Some(iced::Background::Color(ThemeColors::SURFACE_BORDER)),
                        ..Default::default()
                    }
                })
            };

            let (left_offset, menu_body) = match menu_type {
                MenuType::File => (
                    136.0,
                    column![
                        text("PROJEKT & FILER")
                            .size(10)
                            .color(ThemeColors::TEXT_MUTED),
                        Space::new().height(2),
                        menu_item("➕", "Nyt projekt", Message::NewProject),
                        menu_item("📁", "Åbn projekt...", Message::OpenProjectDialog),
                        menu_item("💾", "Gem", Message::SaveProject),
                        menu_item("💾", "Gem som...", Message::SaveProjectAsDialog),
                        Space::new().height(4),
                        make_separator(),
                        Space::new().height(4),
                        text("MODELINDSTILLINGER")
                            .size(10)
                            .color(ThemeColors::TEXT_MUTED),
                        Space::new().height(2),
                        menu_item(
                            "📋",
                            "Modelomslag & Metadata...",
                            Message::OpenMetadataModal
                        ),
                    ]
                    .spacing(2)
                    .width(Length::Fixed(240.0)),
                ),
                MenuType::Help => (
                    216.0,
                    column![
                        text("DOKUMENTATION & HJÆLP")
                            .size(10)
                            .color(ThemeColors::TEXT_MUTED),
                        Space::new().height(2),
                        menu_item("📖", "FDA Modelregler v2.1 ↗", Message::OpenModelRules),
                    ]
                    .spacing(2)
                    .width(Length::Fixed(220.0)),
                ),
                MenuType::Collab => (
                    280.0,
                    column![
                        text("LIVE SAMARBEJDE (E2EE)")
                            .size(10)
                            .color(ThemeColors::TEXT_MUTED),
                        Space::new().height(2),
                        if self.collab_state.is_active() {
                            column![menu_item("🔴", "Afbryd session", Message::CollabDisconnect)]
                        } else {
                            column![
                                menu_item(
                                    "👑",
                                    "Start session (Vært)...",
                                    Message::OpenStartSessionModal
                                ),
                                menu_item(
                                    "👥",
                                    "Deltag i session (Gæst)...",
                                    Message::OpenJoinSessionModal
                                ),
                            ]
                            .spacing(2)
                        },
                    ]
                    .spacing(2)
                    .width(Length::Fixed(240.0)),
                ),
            };

            let menu_card = container(menu_body)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(ThemeColors::SURFACE_CARD)),
                    border: iced::Border {
                        color: ThemeColors::SURFACE_BORDER,
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    shadow: iced::Shadow {
                        color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.12),
                        offset: iced::Vector::new(0.0, 6.0),
                        blur_radius: 16.0,
                    },
                    ..Default::default()
                })
                .padding(10);

            let backdrop = mouse_area(
                container(Space::new().width(Length::Fill).height(Length::Fill))
                    .style(|_| container::Style::default()),
            )
            .on_press(Message::CloseMenu);

            let dropdown_position = container(row![
                Space::new().width(Length::Fixed(left_offset)),
                column![Space::new().height(Length::Fixed(56.0)), menu_card,],
            ])
            .width(Length::Fill)
            .height(Length::Fill);

            stack![backdrop, dropdown_position].into()
        });

        // 3. Fane Indhold
        let content: Element<Message> = match self.active_tab {
            Tab::ConceptList => {
                if let Some(editor) = &self.editor_state {
                    editor.view()
                } else {
                    let filtered = self.filtered_concepts();
                    concept_table::view(filtered, self.project.concepts().len(), &self.search_query)
                }
            }

            Tab::ConceptModel => concept_model_view::view(
                self.project.concepts(),
                self.project.concept_graph(),
                self.selected_graph_node_id,
                self.selected_edge,
                &self.concept_model_search,
                self.canvas_viewport,
                self.snap_to_grid,
                self.is_space_pressed,
                self.is_inline_graph_editing,
                self.editor_state.as_ref(),
                self.relation_dialog.as_ref(),
                self.show_left_sidebar,
            ),

            Tab::InformationModel => information_model_view::view(
                self.project.information_model(),
                self.project.information_graph(),
                self.project.concepts(),
                self.selected_info_class_id,
                self.selected_info_graph_node_id,
                self.selected_info_edge,
                &self.info_class_search,
                self.info_canvas_viewport,
                self.info_snap_to_grid,
                self.is_space_pressed,
                self.info_relation_dialog.as_ref(),
                self.show_left_sidebar,
            ),
        };

        // 4. Status Bar
        let save_status_text = self.footer_status_text();

        let rules_button = button(
            row![
                text("FDA Modelregler v2.1")
                    .size(12)
                    .color(ThemeColors::TEXT_MUTED),
                text("↗").size(10).color(ThemeColors::SLATE_400),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        )
        .on_press(Message::OpenModelRules)
        .style(|_theme, status| {
            let text_color = match status {
                button::Status::Hovered => ThemeColors::PRIMARY,
                _ => ThemeColors::SLATE_500,
            };
            button::Style {
                background: None,
                text_color,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
                ..Default::default()
            }
        })
        .padding([0, 2]);

        let save_widget: Element<Message> = match &self.save_status {
            SaveStatus::Saved { path, .. } => {
                let full_path = std::fs::canonicalize(path).unwrap_or_else(|_| PathBuf::from(path));
                tooltip(
                    text(format!("• {}", save_status_text))
                        .size(12)
                        .color(ThemeColors::TEXT_MUTED),
                    container(
                        text(full_path.display().to_string())
                            .size(11)
                            .color(ThemeColors::TEXT_DARK),
                    )
                    .padding([4, 8])
                    .style(|_| container::Style {
                        background: Some(iced::Background::Color(ThemeColors::SURFACE_CARD)),
                        border: iced::Border {
                            color: ThemeColors::SURFACE_BORDER,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }),
                    tooltip::Position::Top,
                )
                .into()
            }
            _ => text(format!("• {}", save_status_text))
                .size(12)
                .color(ThemeColors::TEXT_MUTED)
                .into(),
        };

        let collab_status_widget: Element<Message> = if self.collab_state.is_active() {
            let dot_color = match self.collab_connection_status {
                crate::features::collab::ConnectionStatus::Connected => {
                    iced::Color::from_rgb(0.12, 0.65, 0.35)
                }
                crate::features::collab::ConnectionStatus::Reconnecting => {
                    iced::Color::from_rgb(0.85, 0.55, 0.10)
                }
                _ => iced::Color::from_rgb(0.55, 0.60, 0.68),
            };
            row![
                text("•").size(12).color(dot_color),
                Space::new().width(4),
                text(self.collab_status_summary())
                    .size(12)
                    .color(ThemeColors::SLATE_700),
            ]
            .align_y(Alignment::Center)
            .into()
        } else {
            text("• Offline")
                .size(12)
                .color(ThemeColors::SLATE_500)
                .into()
        };

        let status_bar = row![
            rules_button,
            text("• Klar").size(12).color(ThemeColors::SLATE_500),
            Space::new().width(12),
            text(format!("• {} begreber", self.project.concepts().len()))
                .size(12)
                .color(ThemeColors::SLATE_600),
            Space::new().width(12),
            save_widget,
            Space::new().width(12),
            collab_status_widget,
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

        if let Some(modal) = maybe_metadata_modal {
            stack![base_layout, modal].into()
        } else if let Some(modal) = maybe_start_session_modal {
            stack![base_layout, modal].into()
        } else if let Some(modal) = maybe_join_session_modal {
            stack![base_layout, modal].into()
        } else if let Some(modal) = maybe_guest_ended_modal {
            stack![base_layout, modal].into()
        } else if let Some(modal) = maybe_file_dialog_modal {
            stack![base_layout, modal].into()
        } else if let Some(modal) = maybe_relation_modal {
            stack![base_layout, modal].into()
        } else if let Some(modal) = maybe_quick_create_modal {
            stack![base_layout, modal].into()
        } else if let Some(menu) = maybe_menu_overlay {
            stack![base_layout, menu].into()
        } else {
            base_layout
        }
    }
}
