use edge::features::concepts::{BelongsToDomain, Concept, ConceptValidator};
use edge::features::concept_model::{ConceptGraph, RelationKind};
use edge::features::information_model::{Attribute, InformationClass, Multiplicity, PrimitiveType};
use edge::features::model::{ModelMetadata, ModelProject, ModelStatus};
use edge::ui::app::{App, Message, Tab};

#[test]
fn test_fda_project_initialization_and_metadata() {
    let metadata = ModelMetadata::new(
        "Køretøjsmodellen",
        "Kernemodel for køretøjer og registrering i Danmark",
        "https://data.gov.dk/model/core/vehicle",
        "Motorstyrelsen",
        "Transport og Trafik",
        "1.0.0",
        ModelStatus::Draft,
    );

    let project = ModelProject::new(metadata);
    assert_eq!(project.metadata().name(), "Køretøjsmodellen");
    assert_eq!(project.metadata().version(), "1.0.0");
    assert_eq!(project.metadata().status(), ModelStatus::Draft);
}

#[test]
fn test_fda_concept_validation_rules() {
    // Gyldigt begreb efter Bilag D og E
    let mut valid_concept = Concept::new(
        "Køretøj",
        "Et mobilt teknisk anlæg, der anvendes til transport af personer eller gods.",
        BelongsToDomain::Yes,
    );
    valid_concept.set_accepted_term(Some("Transportmiddel".to_string()));
    valid_concept.set_source(Some("Færdselsloven § 2, stk. 1".to_string()));
    valid_concept.set_identifier(Some("https://data.gov.dk/model/core/vehicle/Koeretoej".to_string()));

    let validation = ConceptValidator::validate(&valid_concept);
    assert!(validation.is_ok(), "Gyldigt begreb skal bestå FDA validering");

    // Ugyldigt begreb (mangler påkrævet definition)
    let invalid_concept = Concept::new("Ugyldigt", "", BelongsToDomain::Yes);
    let invalid_res = ConceptValidator::validate(&invalid_concept);
    assert!(invalid_res.is_err(), "Begreb uden definition skal fejle validering");
}

#[test]
fn test_progression_from_concept_to_graph_and_information_model() {
    // 1. Opret begreber
    let c1 = Concept::new("Køretøj", "Transportmiddel...", BelongsToDomain::Yes);
    let c2 = Concept::new("Personbil", "Køretøj indrettet til befordring af højst 9 personer...", BelongsToDomain::Yes);

    // 2. Begrebsmodel (Graf med generalisering)
    let mut graph = ConceptGraph::new();
    let n1 = graph.add_node(&c1);
    let n2 = graph.add_node(&c2);
    graph.add_relation(n2, n1, RelationKind::Generalization);

    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);

    // 3. Informationsmodel (Klasse med attributter og multipliciteter)
    let mut info_class = InformationClass::from_concept(&c2);
    info_class.add_attribute(Attribute::new(
        "registreringsnummer",
        PrimitiveType::CharacterString,
        Multiplicity::exactly_one(),
    ));

    assert_eq!(info_class.name(), "Personbil");
    assert_eq!(info_class.attributes().len(), 1);
    assert_eq!(info_class.attributes()[0].multiplicity(), Multiplicity::new(1, Some(1)));
}

#[test]
fn test_ui_app_state_and_tab_switching() {
    let mut app = App::new_with_path(None);
    assert_eq!(app.active_tab(), Tab::Metadata);

    let _ = app.update(Message::SelectTab(Tab::ConceptList));
    assert_eq!(app.active_tab(), Tab::ConceptList);

    let _ = app.update(Message::SelectTab(Tab::ConceptModel));
    assert_eq!(app.active_tab(), Tab::ConceptModel);

    let _ = app.update(Message::SelectTab(Tab::InformationModel));
    assert_eq!(app.active_tab(), Tab::InformationModel);
}

#[test]
fn test_fda_project_concept_crud() {
    let mut project = ModelProject::default();
    assert!(project.concepts().is_empty());

    let mut c1 = Concept::new(
        "Køretøj",
        "Et mobilt teknisk anlæg til transport af personer eller gods.",
        BelongsToDomain::Yes,
    );
    c1.set_source(Some("Færdselsloven § 2, stk. 1".to_string()));
    c1.set_legal_source(Some("LBK nr 1324 af 21/11/2023".to_string()));

    // 1. Create (Add)
    let id1 = project.add_concept(c1.clone()).expect("Gyldigt begreb skal tilføjes");
    assert_eq!(project.concepts().len(), 1);
    assert_eq!(project.get_concept(id1).unwrap().preferred_term(), "Køretøj");

    // 2. Reject Invalid Concept
    let invalid = Concept::new("", "Ugyldig uden term", BelongsToDomain::Yes);
    assert!(project.add_concept(invalid).is_err());

    // 3. Update
    let mut updated = project.get_concept(id1).unwrap().clone();
    updated.set_definition("Opdateret præcis definition af køretøj.");
    project.update_concept(updated).expect("Opdatering skal lykkes");
    assert_eq!(
        project.get_concept(id1).unwrap().definition(),
        "Opdateret præcis definition af køretøj."
    );

    // 4. Delete
    let removed = project.remove_concept(id1);
    assert!(removed.is_some());
    assert!(project.concepts().is_empty());
}

#[test]
fn test_concept_list_ui_crud_cycle() {
    use edge::ui::app::ConceptFormField;

    let mut app = App::new_with_path(None);
    let _ = app.update(Message::SelectTab(Tab::ConceptList));

    // 1. Start nyt begreb
    let _ = app.update(Message::StartNewConcept);
    assert!(app.is_editing_concept());

    // 2. Udfyld felter
    let _ = app.update(Message::UpdateConceptField(ConceptFormField::PreferredTerm, "Personbil".to_string()));
    let _ = app.update(Message::UpdateConceptField(
        ConceptFormField::Definition,
        "Køretøj indrettet til befordring af højst 9 personer.".to_string(),
    ));
    let _ = app.update(Message::UpdateConceptField(ConceptFormField::BelongsToDomain, "Ja".to_string()));
    let _ = app.update(Message::UpdateConceptField(ConceptFormField::Source, "Færdselsloven".to_string()));
    let _ = app.update(Message::UpdateConceptField(ConceptFormField::LegalSource, "LBK nr 1324".to_string()));

    // 3. Gem begreb
    let _ = app.update(Message::SaveConcept);
    assert!(!app.is_editing_concept());
    assert_eq!(app.project().concepts().len(), 1);

    // Verificer at view() renderer uden fejl for tabel med begreb
    let _ = app.view();

    let id = {
        let saved = &app.project().concepts()[0];
        assert_eq!(saved.preferred_term(), "Personbil");
        assert_eq!(saved.definition(), "Køretøj indrettet til befordring af højst 9 personer.");
        assert_eq!(saved.belongs_to_domain(), &BelongsToDomain::Yes);
        assert_eq!(saved.source(), Some("Færdselsloven"));
        assert_eq!(saved.legal_source(), Some("LBK nr 1324"));
        saved.id()
    };

    // 4. Søgning / filtrering
    let _ = app.update(Message::SearchQueryChanged("Person".to_string()));
    assert_eq!(app.filtered_concepts().len(), 1);
    let _ = app.view();

    let _ = app.update(Message::SearchQueryChanged("Ukendt".to_string()));
    assert_eq!(app.filtered_concepts().len(), 0);
    let _ = app.view();

    let _ = app.update(Message::SearchQueryChanged("".to_string()));
    assert_eq!(app.filtered_concepts().len(), 1);

    // 5. Rediger begreb
    let _ = app.update(Message::EditConcept(id));
    assert!(app.is_editing_concept());
    let _ = app.view();
    let _ = app.update(Message::UpdateConceptField(ConceptFormField::PreferredTerm, "Personbil (M1)".to_string()));
    let _ = app.update(Message::SaveConcept);
    assert_eq!(app.project().concepts()[0].preferred_term(), "Personbil (M1)");

    // 6. Slet begreb
    let _ = app.update(Message::DeleteConcept(id));
    assert!(app.project().concepts().is_empty());
}

#[test]
fn test_keyboard_navigation_and_shortcuts() {
    let mut app = App::new_with_path(None);

    // FocusNext og FocusPrevious returnerer gyldige tasks
    let _ = app.update(Message::FocusNext);
    let _ = app.update(Message::FocusPrevious);

    // Escape lukker editor
    let _ = app.update(Message::StartNewConcept);
    assert!(app.is_editing_concept());
    let _ = app.update(Message::EscapePressed);
    assert!(!app.is_editing_concept(), "Escape skal annullere editor");

    // Escape lukker også fildialog
    let _ = app.update(Message::OpenInlineFileDialog(edge::ui::app::FileDialogMode::Open));
    assert!(app.is_file_dialog_open());
    let _ = app.update(Message::EscapePressed);
    assert!(!app.is_file_dialog_open(), "Escape skal lukke fildialog");
}

#[test]
fn test_new_project_does_not_overwrite_disk_file() {
    use edge::ui::app::ConceptFormField;
    use edge::features::model::storage::ProjectStorage;

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("test_edge_no_overwrite_{}.edge.json", uuid::Uuid::new_v4()));

    let mut app = App::new_with_path(Some(file_path.clone()));

    // Opret et begreb så filen findes på disk med data
    let _ = app.update(Message::SelectTab(Tab::ConceptList));
    let _ = app.update(Message::StartNewConcept);
    let _ = app.update(Message::UpdateConceptField(ConceptFormField::PreferredTerm, "TestTerm".to_string()));
    let _ = app.update(Message::UpdateConceptField(ConceptFormField::Definition, "TestDefinition".to_string()));
    let _ = app.update(Message::SaveConcept);

    let on_disk_before = ProjectStorage::load_from_file(&file_path).expect("Skal kunne læses");
    assert_eq!(on_disk_before.concepts().len(), 1);

    // Tryk "Nyt Projekt"
    let _ = app.update(Message::NewProject);
    assert!(app.project().concepts().is_empty());
    assert_eq!(app.current_file_path(), None, "Nyt projekt skal nulstille aktiv filsti");

    // Verificer at filen på disken STADIG indeholder det oprindelige begreb!
    let on_disk_after = ProjectStorage::load_from_file(&file_path).expect("Skal stadig kunne læses");
    assert_eq!(on_disk_after.concepts().len(), 1, "Nyt projekt må IKKE slette eksisterende fil på disk");

    let _ = std::fs::remove_file(file_path);
}

#[test]
fn test_project_storage_roundtrip_and_atomic_save() {
    use edge::features::model::storage::ProjectStorage;

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("test_edge_project_{}.edge.json", uuid::Uuid::new_v4()));

    let mut project = ModelProject::default();
    let mut c1 = Concept::new("Vej", "Færdselsareal for køretøjer og fodgængere.", BelongsToDomain::Yes);
    c1.set_legal_source(Some("Vejloven § 3".to_string()));
    project.add_concept(c1).unwrap();

    // 1. Gem til fil
    let save_res = ProjectStorage::save_to_file(&project, &file_path);
    assert!(save_res.is_ok(), "Skal kunne gemme projektfil atomisk");
    assert!(file_path.exists(), "Projektfil skal eksistere på disken");

    // 2. Indlæs fra fil
    let loaded = ProjectStorage::load_from_file(&file_path).expect("Skal kunne indlæse gemt projektfil");
    assert_eq!(loaded.metadata().name(), project.metadata().name());
    assert_eq!(loaded.concepts().len(), 1);
    assert_eq!(loaded.concepts()[0].preferred_term(), "Vej");
    assert_eq!(loaded.concepts()[0].legal_source(), Some("Vejloven § 3"));

    // Oprydning
    let _ = std::fs::remove_file(file_path);
}

#[test]
fn test_app_autosave_lifecycle() {
    use edge::ui::app::ConceptFormField;
    use edge::features::model::storage::ProjectStorage;

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("test_edge_autosave_{}.edge.json", uuid::Uuid::new_v4()));

    let mut app = App::new_with_path(Some(file_path.clone()));
    assert_eq!(app.current_file_path(), Some(&file_path));

    // 1. Opret begreb -> autosave skal opdatere filen på disken
    let _ = app.update(Message::SelectTab(Tab::ConceptList));
    let _ = app.update(Message::StartNewConcept);
    let _ = app.update(Message::UpdateConceptField(ConceptFormField::PreferredTerm, "Cykelsti".to_string()));
    let _ = app.update(Message::UpdateConceptField(ConceptFormField::Definition, "Færdselsareal forbeholdt cykler.".to_string()));
    let _ = app.update(Message::SaveConcept);

    assert!(file_path.exists(), "Autosave skal have oprettet filen på disken");
    let on_disk = ProjectStorage::load_from_file(&file_path).expect("Skal kunne læse autosaved fil");
    assert_eq!(on_disk.concepts().len(), 1);
    assert_eq!(on_disk.concepts()[0].preferred_term(), "Cykelsti");

    // 2. Slet begreb -> autosave skal genskrive filen på disken
    let id = on_disk.concepts()[0].id();
    let _ = app.update(Message::DeleteConcept(id));

    let on_disk_after_del = ProjectStorage::load_from_file(&file_path).expect("Skal kunne læse efter sletning");
    assert!(on_disk_after_del.concepts().is_empty(), "Autosaved fil skal have 0 begreber efter sletning");

    // Oprydning
    let _ = std::fs::remove_file(file_path);
}

#[test]
fn test_concept_graph_lifecycle_and_persistence() {
    use edge::features::concept_model::RelationKind;
    use edge::features::model::storage::ProjectStorage;

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("test_edge_graph_{}.edge.json", uuid::Uuid::new_v4()));

    let mut project = ModelProject::default();

    let mut c1 = Concept::new("Køretøj", "Mobilt teknisk anlæg...", BelongsToDomain::Yes);
    c1.set_legal_source(Some("Færdselsloven § 2".to_string()));
    let id1 = project.add_concept(c1).unwrap();

    let c2 = Concept::new("Personbil", "Køretøj til højst 9 personer...", BelongsToDomain::Yes);
    let id2 = project.add_concept(c2).unwrap();

    let c3 = Concept::new("Person", "CPR-registreret person...", BelongsToDomain::No);
    let id3 = project.add_concept(c3).unwrap();

    // 1. Synkroniser projektbegreber med concept_graph
    project.sync_concept_graph();
    let graph = project.concept_graph();
    assert_eq!(graph.node_count(), 3, "Skal have 3 noder i grafen");

    let node1 = graph.find_node_by_concept(id1).expect("Skal finde node for Køretøj");
    let node2 = graph.find_node_by_concept(id2).expect("Skal finde node for Personbil");
    let node3 = graph.find_node_by_concept(id3).expect("Skal finde node for Person");

    assert_eq!(node1.label(), "Køretøj");
    assert!(node1.is_local(), "Køretøj er lokalt begreb (FDA sand)");
    assert!(!node3.is_local(), "Person er indlånt begreb (FDA blå)");

    let n1_id = node1.id();
    let n2_id = node2.id();
    let n3_id = node3.id();

    // 2. Opret UML relationer: Generalisering (Personbil -> Køretøj) og Association (Person -> Personbil)
    project.concept_graph_mut().add_relation(n2_id, n1_id, RelationKind::Generalization);
    project.concept_graph_mut().add_relation_with_label(
        n3_id,
        n2_id,
        RelationKind::Association,
        Some("ejer".to_string()),
    );
    assert_eq!(project.concept_graph().edge_count(), 2);

    // 3. Flyt node position (bruger trækker node på lærredet)
    project.concept_graph_mut().update_node_position(n1_id, 320.0, 140.0);
    let moved_node = project.concept_graph().find_node(n1_id).unwrap();
    assert_eq!(moved_node.x(), 320.0);
    assert_eq!(moved_node.y(), 140.0);

    // 4. Persistens roundtrip: Gem til .edge.json og indlæs igen
    ProjectStorage::save_to_file(&project, &file_path).expect("Skal kunne gemme projekt med graf");
    let loaded = ProjectStorage::load_from_file(&file_path).expect("Skal kunne indlæse projekt med graf");

    assert_eq!(loaded.concepts().len(), 3);
    assert_eq!(loaded.concept_graph().node_count(), 3);
    assert_eq!(loaded.concept_graph().edge_count(), 2);

    let loaded_n1 = loaded.concept_graph().find_node(n1_id).expect("Node1 skal findes efter indlæsning");
    assert_eq!(loaded_n1.x(), 320.0);
    assert_eq!(loaded_n1.y(), 140.0);

    // 5. Fail-Closed kaskadesletning: Sletning af Køretøj skal fjerne noden OG generaliserings-relationen
    let mut project_mut = loaded;
    project_mut.remove_concept(id1);
    assert_eq!(project_mut.concept_graph().node_count(), 2);
    assert_eq!(
        project_mut.concept_graph().edge_count(),
        1,
        "Generaliseringen til Køretøj skal automatisk være slettet (ingen dangling edges)"
    );

    // 6. Test App TEA graf-beskeder
    let mut app = App::new_with_path(Some(file_path.clone()));
    let _ = app.update(Message::SelectTab(Tab::ConceptModel));
    let _ = app.update(Message::GraphNodeSelected(Some(n2_id)));
    assert_eq!(app.selected_graph_node_id(), Some(n2_id));

    let _ = app.update(Message::GraphNodeMoved(n2_id, 500.0, 300.0));
    let app_n2 = app.project().concept_graph().find_node(n2_id).unwrap();
    assert_eq!(app_n2.x(), 500.0);
    assert_eq!(app_n2.y(), 300.0);

    let _ = std::fs::remove_file(file_path);
}

#[test]
fn test_ui_theme_tokens_and_widget_styles() {
    use edge::ui::theme::{
        ThemeColors, card_container_style, pill_container_style,
        primary_button_style, secondary_button_style, modern_input_style,
        modal_backdrop_style, modal_card_style,
    };
    use iced::Theme;

    let theme = Theme::Light;

    // 1. Verificer at de nye tokens er tilgængelige
    assert_ne!(ThemeColors::SURFACE_CARD, ThemeColors::SURFACE_BG);
    assert_ne!(ThemeColors::SLATE_50, ThemeColors::SLATE_900);
    assert_ne!(ThemeColors::PRIMARY_HOVER, ThemeColors::PRIMARY_ACTIVE);

    // 2. Verificer container styles
    let card = card_container_style(&theme);
    assert!(card.background.is_some());
    assert_eq!(card.border.radius, 8.0.into());

    let pill = pill_container_style(&theme);
    assert!(pill.background.is_some());
    assert_eq!(pill.border.radius, 8.0.into());

    let modal_bd = modal_backdrop_style(&theme);
    assert!(modal_bd.background.is_some());

    let modal_card = modal_card_style(&theme);
    assert!(modal_card.background.is_some());
    assert_eq!(modal_card.border.radius, 12.0.into());

    // 3. Verificer knap styles (Active & Hovered)
    let btn_prim_active = primary_button_style(&theme, iced::widget::button::Status::Active);
    assert!(btn_prim_active.background.is_some());
    let btn_prim_hover = primary_button_style(&theme, iced::widget::button::Status::Hovered);
    assert_ne!(btn_prim_active.background, btn_prim_hover.background);

    let btn_sec_active = secondary_button_style(&theme, iced::widget::button::Status::Active);
    assert!(btn_sec_active.background.is_some());

    // 4. Verificer input style
    let input_active = modern_input_style(&theme, iced::widget::text_input::Status::Active);
    let input_focused = modern_input_style(&theme, iced::widget::text_input::Status::Focused { is_hovered: false });
    assert_ne!(input_active.border.color, input_focused.border.color);
}

#[test]
fn test_app_modal_overlay_rendering() {
    let mut app = App::new_with_path(None);

    // 1. Åbn fildialog og verificer at modal view renderes uden panic
    let _ = app.update(Message::OpenInlineFileDialog(edge::ui::app::FileDialogMode::SaveAs));
    assert!(app.is_file_dialog_open());
    let _ = app.view();

    // 2. Escape lukker modal fildialog
    let _ = app.update(Message::EscapePressed);
    assert!(!app.is_file_dialog_open());
    let _ = app.view();

    // 3. Åbn graf relationsdialog og verificer at modal view renderes uden panic
    let _ = app.update(Message::SelectTab(Tab::ConceptModel));
    let _ = app.update(Message::GraphOpenRelationDialog);
    assert!(app.is_relation_dialog_open());
    let _ = app.view();

    // 4. Escape lukker modal relationsdialog
    let _ = app.update(Message::EscapePressed);
    assert!(!app.is_relation_dialog_open());
    let _ = app.view();
}


