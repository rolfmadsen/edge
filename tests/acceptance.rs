use edge::features::concept_model::{ConceptGraph, RelationKind};
use edge::features::concepts::{BelongsToDomain, Concept, ConceptValidator};
use edge::features::information_model::{
    Attribute, InformationClass, InformationModel, Multiplicity, PrimitiveType,
};
use edge::features::model::{ModelMetadata, ModelProject, ModelStatus};
use edge::ui::app::{App, ConceptOption, Message, Tab};

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
    valid_concept.set_identifier(Some(
        "https://data.gov.dk/model/core/vehicle/Koeretoej".to_string(),
    ));

    let validation = ConceptValidator::validate(&valid_concept);
    assert!(
        validation.is_ok(),
        "Gyldigt begreb skal bestå FDA validering"
    );

    // Ugyldigt begreb (mangler påkrævet definition)
    let invalid_concept = Concept::new("Ugyldigt", "", BelongsToDomain::Yes);
    let invalid_res = ConceptValidator::validate(&invalid_concept);
    assert!(
        invalid_res.is_err(),
        "Begreb uden definition skal fejle validering"
    );
}

#[test]
fn test_progression_from_concept_to_graph_and_information_model() {
    // 1. Opret begreber
    let c1 = Concept::new("Køretøj", "Transportmiddel...", BelongsToDomain::Yes);
    let c2 = Concept::new(
        "Personbil",
        "Køretøj indrettet til befordring af højst 9 personer...",
        BelongsToDomain::Yes,
    );

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
    assert_eq!(
        info_class.attributes()[0].multiplicity(),
        Multiplicity::new(1, Some(1))
    );
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
    let id1 = project
        .add_concept(c1.clone())
        .expect("Gyldigt begreb skal tilføjes");
    assert_eq!(project.concepts().len(), 1);
    assert_eq!(
        project.get_concept(id1).unwrap().preferred_term(),
        "Køretøj"
    );

    // 2. Reject Invalid Concept
    let invalid = Concept::new("", "Ugyldig uden term", BelongsToDomain::Yes);
    assert!(project.add_concept(invalid).is_err());

    // 3. Update
    let mut updated = project.get_concept(id1).unwrap().clone();
    updated.set_definition("Opdateret præcis definition af køretøj.");
    project
        .update_concept(updated)
        .expect("Opdatering skal lykkes");
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
    let _ = app.update(Message::UpdateConceptField(
        ConceptFormField::PreferredTerm,
        "Personbil".to_string(),
    ));
    let _ = app.update(Message::UpdateConceptField(
        ConceptFormField::Definition,
        "Køretøj indrettet til befordring af højst 9 personer.".to_string(),
    ));
    let _ = app.update(Message::UpdateConceptField(
        ConceptFormField::BelongsToDomain,
        "Ja".to_string(),
    ));
    let _ = app.update(Message::UpdateConceptField(
        ConceptFormField::Source,
        "Færdselsloven".to_string(),
    ));
    let _ = app.update(Message::UpdateConceptField(
        ConceptFormField::LegalSource,
        "LBK nr 1324".to_string(),
    ));

    // 3. Gem begreb
    let _ = app.update(Message::SaveConcept);
    assert!(!app.is_editing_concept());
    assert_eq!(app.project().concepts().len(), 1);

    // Verificer at view() renderer uden fejl for tabel med begreb
    let _ = app.view();

    let id = {
        let saved = &app.project().concepts()[0];
        assert_eq!(saved.preferred_term(), "Personbil");
        assert_eq!(
            saved.definition(),
            "Køretøj indrettet til befordring af højst 9 personer."
        );
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
    let _ = app.update(Message::UpdateConceptField(
        ConceptFormField::PreferredTerm,
        "Personbil (M1)".to_string(),
    ));
    let _ = app.update(Message::SaveConcept);
    assert_eq!(
        app.project().concepts()[0].preferred_term(),
        "Personbil (M1)"
    );

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
    let _ = app.update(Message::OpenInlineFileDialog(
        edge::ui::app::FileDialogMode::Open,
    ));
    assert!(app.is_file_dialog_open());
    let _ = app.update(Message::EscapePressed);
    assert!(!app.is_file_dialog_open(), "Escape skal lukke fildialog");
}

#[test]
fn test_new_project_does_not_overwrite_disk_file() {
    use edge::features::model::storage::ProjectStorage;
    use edge::ui::app::ConceptFormField;

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!(
        "test_edge_no_overwrite_{}.edge.json",
        uuid::Uuid::new_v4()
    ));

    let mut app = App::new_with_path(Some(file_path.clone()));

    // Opret et begreb så filen findes på disk med data
    let _ = app.update(Message::SelectTab(Tab::ConceptList));
    let _ = app.update(Message::StartNewConcept);
    let _ = app.update(Message::UpdateConceptField(
        ConceptFormField::PreferredTerm,
        "TestTerm".to_string(),
    ));
    let _ = app.update(Message::UpdateConceptField(
        ConceptFormField::Definition,
        "TestDefinition".to_string(),
    ));
    let _ = app.update(Message::SaveConcept);

    let on_disk_before = ProjectStorage::load_from_file(&file_path).expect("Skal kunne læses");
    assert_eq!(on_disk_before.concepts().len(), 1);

    // Tryk "Nyt Projekt"
    let _ = app.update(Message::NewProject);
    assert!(app.project().concepts().is_empty());
    assert_eq!(
        app.current_file_path(),
        None,
        "Nyt projekt skal nulstille aktiv filsti"
    );

    // Verificer at filen på disken STADIG indeholder det oprindelige begreb!
    let on_disk_after =
        ProjectStorage::load_from_file(&file_path).expect("Skal stadig kunne læses");
    assert_eq!(
        on_disk_after.concepts().len(),
        1,
        "Nyt projekt må IKKE slette eksisterende fil på disk"
    );

    let _ = std::fs::remove_file(file_path);
}

#[test]
fn test_project_storage_roundtrip_and_atomic_save() {
    use edge::features::model::storage::ProjectStorage;

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!(
        "test_edge_project_{}.edge.json",
        uuid::Uuid::new_v4()
    ));

    let mut project = ModelProject::default();
    let mut c1 = Concept::new(
        "Vej",
        "Færdselsareal for køretøjer og fodgængere.",
        BelongsToDomain::Yes,
    );
    c1.set_legal_source(Some("Vejloven § 3".to_string()));
    project.add_concept(c1).unwrap();

    // 1. Gem til fil
    let save_res = ProjectStorage::save_to_file(&project, &file_path);
    assert!(save_res.is_ok(), "Skal kunne gemme projektfil atomisk");
    assert!(file_path.exists(), "Projektfil skal eksistere på disken");

    // 2. Indlæs fra fil
    let loaded =
        ProjectStorage::load_from_file(&file_path).expect("Skal kunne indlæse gemt projektfil");
    assert_eq!(loaded.metadata().name(), project.metadata().name());
    assert_eq!(loaded.concepts().len(), 1);
    assert_eq!(loaded.concepts()[0].preferred_term(), "Vej");
    assert_eq!(loaded.concepts()[0].legal_source(), Some("Vejloven § 3"));

    // Oprydning
    let _ = std::fs::remove_file(file_path);
}

#[test]
fn test_app_autosave_lifecycle() {
    use edge::features::model::storage::ProjectStorage;
    use edge::ui::app::ConceptFormField;

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!(
        "test_edge_autosave_{}.edge.json",
        uuid::Uuid::new_v4()
    ));

    let mut app = App::new_with_path(Some(file_path.clone()));
    assert_eq!(app.current_file_path(), Some(&file_path));

    // 1. Opret begreb -> autosave skal opdatere filen på disken
    let _ = app.update(Message::SelectTab(Tab::ConceptList));
    let _ = app.update(Message::StartNewConcept);
    let _ = app.update(Message::UpdateConceptField(
        ConceptFormField::PreferredTerm,
        "Cykelsti".to_string(),
    ));
    let _ = app.update(Message::UpdateConceptField(
        ConceptFormField::Definition,
        "Færdselsareal forbeholdt cykler.".to_string(),
    ));
    let _ = app.update(Message::SaveConcept);

    assert!(
        file_path.exists(),
        "Autosave skal have oprettet filen på disken"
    );
    let on_disk =
        ProjectStorage::load_from_file(&file_path).expect("Skal kunne læse autosaved fil");
    assert_eq!(on_disk.concepts().len(), 1);
    assert_eq!(on_disk.concepts()[0].preferred_term(), "Cykelsti");

    // 2. Slet begreb -> autosave skal genskrive filen på disken
    let id = on_disk.concepts()[0].id();
    let _ = app.update(Message::DeleteConcept(id));

    let on_disk_after_del =
        ProjectStorage::load_from_file(&file_path).expect("Skal kunne læse efter sletning");
    assert!(
        on_disk_after_del.concepts().is_empty(),
        "Autosaved fil skal have 0 begreber efter sletning"
    );

    // Oprydning
    let _ = std::fs::remove_file(file_path);
}

#[test]
fn test_concept_graph_lifecycle_and_persistence() {
    use edge::features::concept_model::RelationKind;
    use edge::features::model::storage::ProjectStorage;

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!(
        "test_edge_graph_{}.edge.json",
        uuid::Uuid::new_v4()
    ));

    let mut project = ModelProject::default();

    let mut c1 = Concept::new("Køretøj", "Mobilt teknisk anlæg...", BelongsToDomain::Yes);
    c1.set_legal_source(Some("Færdselsloven § 2".to_string()));
    let id1 = project.add_concept(c1).unwrap();

    let c2 = Concept::new(
        "Personbil",
        "Køretøj til højst 9 personer...",
        BelongsToDomain::Yes,
    );
    let id2 = project.add_concept(c2).unwrap();

    let c3 = Concept::new("Person", "CPR-registreret person...", BelongsToDomain::No);
    let id3 = project.add_concept(c3).unwrap();

    // 1. Synkroniser projektbegreber med concept_graph
    project.sync_concept_graph();
    let graph = project.concept_graph();
    assert_eq!(graph.node_count(), 3, "Skal have 3 noder i grafen");

    let node1 = graph
        .find_node_by_concept(id1)
        .expect("Skal finde node for Køretøj");
    let node2 = graph
        .find_node_by_concept(id2)
        .expect("Skal finde node for Personbil");
    let node3 = graph
        .find_node_by_concept(id3)
        .expect("Skal finde node for Person");

    assert_eq!(node1.label(), "Køretøj");
    assert!(node1.is_local(), "Køretøj er lokalt begreb (FDA sand)");
    assert!(!node3.is_local(), "Person er indlånt begreb (FDA blå)");

    let n1_id = node1.id();
    let n2_id = node2.id();
    let n3_id = node3.id();

    // 2. Opret UML relationer: Generalisering (Personbil -> Køretøj) og Association (Person -> Personbil)
    project
        .concept_graph_mut()
        .add_relation(n2_id, n1_id, RelationKind::Generalization);
    project.concept_graph_mut().add_relation_with_label(
        n3_id,
        n2_id,
        RelationKind::Association,
        Some("ejer".to_string()),
    );
    assert_eq!(project.concept_graph().edge_count(), 2);

    // 3. Flyt node position (bruger trækker node på lærredet)
    project
        .concept_graph_mut()
        .update_node_position(n1_id, 320.0, 140.0);
    let moved_node = project.concept_graph().find_node(n1_id).unwrap();
    assert_eq!(moved_node.x(), 320.0);
    assert_eq!(moved_node.y(), 140.0);

    // 4. Persistens roundtrip: Gem til .edge.json og indlæs igen
    ProjectStorage::save_to_file(&project, &file_path).expect("Skal kunne gemme projekt med graf");
    let loaded =
        ProjectStorage::load_from_file(&file_path).expect("Skal kunne indlæse projekt med graf");

    assert_eq!(loaded.concepts().len(), 3);
    assert_eq!(loaded.concept_graph().node_count(), 3);
    assert_eq!(loaded.concept_graph().edge_count(), 2);

    let loaded_n1 = loaded
        .concept_graph()
        .find_node(n1_id)
        .expect("Node1 skal findes efter indlæsning");
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
        card_container_style, modal_backdrop_style, modal_card_style, modern_input_style,
        pill_container_style, primary_button_style, secondary_button_style, ThemeColors,
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
    let input_focused = modern_input_style(
        &theme,
        iced::widget::text_input::Status::Focused { is_hovered: false },
    );
    assert_ne!(input_active.border.color, input_focused.border.color);
}

#[test]
fn test_app_modal_overlay_rendering() {
    let mut app = App::new_with_path(None);

    // 1. Åbn fildialog og verificer at modal view renderes uden panic
    let _ = app.update(Message::OpenInlineFileDialog(
        edge::ui::app::FileDialogMode::SaveAs,
    ));
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

#[test]
fn test_canvas_direct_concept_creation_and_node_editing() {
    use edge::features::concept_model::NodeId;
    use edge::features::model::storage::ProjectStorage;
    use edge::ui::app::ConceptFormField;

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!(
        "test_edge_canvas_crud_{}.edge.json",
        uuid::Uuid::new_v4()
    ));

    let mut app = App::new_with_path(Some(file_path.clone()));

    // 1. Skift til Begrebsmodel (Graf) fanen
    let _ = app.update(Message::SelectTab(Tab::ConceptModel));
    assert_eq!(app.active_tab(), Tab::ConceptModel);

    // 2. Dobbeltklik på tomt canvas (x: 450.0, y: 250.0) fanges og åbner lynoprettelse
    let _ = app.update(Message::CanvasDoubleClicked(450.0, 250.0));
    assert!(
        app.is_quick_create_open(),
        "Dobbeltklik på tomt canvas skal åbne lynoprettelses-dialog"
    );

    // Verificer at view() renderer modal overlay for lynoprettelse uden fejl
    let _ = app.view();

    // 3. Validering via ConceptValidator: Ugyldigt begreb (tom definition og term) må IKKE oprettes
    let _ = app.update(Message::QuickCreateSubmit);
    assert_eq!(
        app.project().concepts().len(),
        0,
        "Ugyldigt begreb uden term og definition må ikke oprettes"
    );
    assert!(
        app.is_quick_create_open(),
        "Lynoprettelse skal forblive åben ved valideringsfejl"
    );

    // Udfyld kun term (mangler definition jf. FDA krav)
    let _ = app.update(Message::QuickCreateTermChanged("Godsvogn".to_string()));
    let _ = app.update(Message::QuickCreateSubmit);
    assert_eq!(
        app.project().concepts().len(),
        0,
        "Begreb uden definition skal afvises af ConceptValidator"
    );
    assert!(
        app.is_quick_create_open(),
        "Dialog forbliver åben da definition mangler"
    );

    // Udfyld gyldig FDA definition
    let _ = app.update(Message::QuickCreateDefinitionChanged(
        "Jernbanekøretøj indrettet til transport af gods.".to_string(),
    ));
    let _ = app.update(Message::QuickCreateSubmit);

    // 4. Oprettelse lykkes: begreb tilføjes, grafnode placeres på (450, 250) og markeres straks
    assert!(
        !app.is_quick_create_open(),
        "Lynoprettelse dialog skal lukke efter succesfuld oprettelse"
    );
    assert_eq!(app.project().concepts().len(), 1);

    let concept = &app.project().concepts()[0];
    assert_eq!(concept.preferred_term(), "Godsvogn");
    assert_eq!(
        concept.definition(),
        "Jernbanekøretøj indrettet til transport af gods."
    );

    let node = app
        .project()
        .concept_graph()
        .find_node_by_concept(concept.id())
        .expect("Grafen skal indeholde en node for det nye begreb");
    assert_eq!(node.label(), "Godsvogn");
    assert_eq!(
        node.x(),
        450.0,
        "Noden skal placeres præcist på klikkets x-koordinat"
    );
    assert_eq!(
        node.y(),
        250.0,
        "Noden skal placeres præcist på klikkets y-koordinat"
    );
    let node_id: NodeId = node.id();
    assert_eq!(
        app.selected_graph_node_id(),
        Some(node_id),
        "Den nyoprettede grafnode skal automatisk markeres"
    );

    // Verificer autosave: projektfilen på disken indeholder det nye begreb og noden
    let on_disk = ProjectStorage::load_from_file(&file_path)
        .expect("Projektfil skal være autosaved efter lynoprettelse på canvas");
    assert_eq!(on_disk.concepts().len(), 1);
    assert_eq!(on_disk.concept_graph().node_count(), 1);

    // 5. Dobbeltklik på eksisterende grafnode åbner hurtigredigering uden at forlade canvas-fanen
    let _ = app.update(Message::GraphNodeDoubleClicked(node_id));
    assert_eq!(
        app.active_tab(),
        Tab::ConceptModel,
        "Dobbeltklik på node må IKKE navigere væk fra ConceptModel-fanen"
    );
    assert!(
        app.is_node_editing(),
        "Hurtigredigering af nodens begreb skal være aktiv"
    );

    let _ = app.view();

    // Rediger nodens begreb direkte og gem
    let _ = app.update(Message::UpdateConceptField(
        ConceptFormField::PreferredTerm,
        "Godsvogn (Jernbane)".to_string(),
    ));
    let _ = app.update(Message::SaveConcept);

    // 6. Verificer at fanen forbliver ConceptModel, og data er opdateret i både begreb og grafnode
    assert_eq!(
        app.active_tab(),
        Tab::ConceptModel,
        "Skal forblive på ConceptModel fanen efter gem af hurtigredigering"
    );
    assert!(
        !app.is_node_editing(),
        "Hurtigredigering skal være lukket efter gem"
    );
    assert_eq!(
        app.project().concepts()[0].preferred_term(),
        "Godsvogn (Jernbane)"
    );

    let updated_node = app
        .project()
        .concept_graph()
        .find_node(node_id)
        .expect("Noden skal findes");
    assert_eq!(
        updated_node.label(),
        "Godsvogn (Jernbane)",
        "Grafnodens label skal være opdateret til den nye term"
    );

    // Verificer autosave efter hurtigredigering
    let on_disk_after_edit = ProjectStorage::load_from_file(&file_path)
        .expect("Projektfil skal være autosaved efter hurtigredigering");
    assert_eq!(
        on_disk_after_edit.concepts()[0].preferred_term(),
        "Godsvogn (Jernbane)"
    );

    let _ = std::fs::remove_file(file_path);
}

#[test]
fn test_canvas_ergonomics_zoom_pan_grid() {
    use edge::features::concept_model::{DEFAULT_NODE_HEIGHT, DEFAULT_NODE_WIDTH, GRID_SIZE};
    use edge::features::concepts::{BelongsToDomain, Concept};
    use edge::ui::graph_canvas::CanvasViewport;
    use iced::Point;

    // 1. Verificer nodedimensioner og gitter-alignment
    assert_eq!(GRID_SIZE, 20.0, "Gitteret skal være 20px raster");
    assert_eq!(DEFAULT_NODE_WIDTH, 180.0, "Bredde skal være 180px (9x20)");
    assert_eq!(DEFAULT_NODE_HEIGHT, 80.0, "Højde skal være 80px (4x20)");
    assert_eq!(
        DEFAULT_NODE_WIDTH % GRID_SIZE,
        0.0,
        "Bredde skal være multiplum af gitter"
    );
    assert_eq!(
        DEFAULT_NODE_HEIGHT % GRID_SIZE,
        0.0,
        "Højde skal være multiplum af gitter"
    );

    let mut app = App::new_with_path(None);
    let _ = app.update(Message::SelectTab(Tab::ConceptModel));

    // Opret et begreb i projektet og synkroniser til graf
    let c = Concept::new("Vogn", "Rullende materiel", BelongsToDomain::Yes);
    let _ = app.project_mut().add_concept(c);
    app.project_mut().sync_concept_graph();

    let node = &app.project().concept_graph().nodes()[0];
    let node_id = node.id();
    assert_eq!(node.width(), 180.0, "Nodebredde skal være 180px");
    assert_eq!(node.height(), 80.0, "Nodehøjde skal være 80px");

    // 2. Test CanvasViewport transformation og zoom-grænser
    let mut viewport = CanvasViewport::default();
    assert_eq!(viewport.zoom(), 1.0, "Default zoom skal være 1.0 (100%)");
    assert_eq!(viewport.pan().x, 0.0);
    assert_eq!(viewport.pan().y, 0.0);

    // Test world-to-screen og screen-to-world
    let pt = Point::new(100.0, 100.0);
    assert_eq!(viewport.to_world(pt), pt);
    assert_eq!(viewport.to_screen(pt), pt);

    // Test zoom clamping (min 0.25, max 1.50)
    viewport.set_zoom(0.10);
    assert_eq!(viewport.zoom(), 0.25, "Zoom skal klemmes til min 0.25");
    viewport.set_zoom(3.0);
    assert_eq!(viewport.zoom(), 1.50, "Zoom skal klemmes til max 1.50");

    // Test zoom_at forankring (punkt under cursor skal forblive uændret i verdenskoordinater)
    viewport.set_zoom(1.0);
    let cursor = Point::new(200.0, 200.0);
    let world_before = viewport.to_world(cursor);
    viewport.zoom_at(cursor, 1.2);
    let world_after = viewport.to_world(cursor);
    assert!((world_before.x - world_after.x).abs() < 0.001);
    assert!((world_before.y - world_after.y).abs() < 0.001);

    // 3. Test Magnetisk Snap-to-Grid i App
    assert!(
        app.is_snap_to_grid_enabled(),
        "Snap to grid skal være slået til som default"
    );

    // Flyt node til arbitrære koordinater (137.4, 91.2) - skal snappe til (140.0, 100.0)
    let _ = app.update(Message::GraphNodeMoved(node_id, 137.4, 91.2));
    let moved_node = app.project().concept_graph().find_node(node_id).unwrap();
    assert_eq!(
        moved_node.x(),
        140.0,
        "Node x skal snappe til nærmeste multiplum af 20"
    );
    assert_eq!(
        moved_node.y(),
        100.0,
        "Node y skal snappe til nærmeste multiplum af 20"
    );

    // Verificer at alle 4 hjørner rammer gitterpunkter
    assert_eq!(
        (moved_node.x() + moved_node.width()) % GRID_SIZE,
        0.0,
        "Top-højre hjørne"
    );
    assert_eq!(
        (moved_node.y() + moved_node.height()) % GRID_SIZE,
        0.0,
        "Bund-venstre hjørne"
    );
    assert_eq!(
        (moved_node.x() + moved_node.width()) % GRID_SIZE,
        0.0,
        "Bund-højre x"
    );
    assert_eq!(
        (moved_node.y() + moved_node.height()) % GRID_SIZE,
        0.0,
        "Bund-højre y"
    );

    // Slå snapping fra og test at position ikke snappes
    let _ = app.update(Message::ToggleSnapToGrid);
    assert!(!app.is_snap_to_grid_enabled());
    let _ = app.update(Message::GraphNodeMoved(node_id, 137.4, 91.2));
    let unsnapped = app.project().concept_graph().find_node(node_id).unwrap();
    assert_eq!(unsnapped.x(), 137.4);
    assert_eq!(unsnapped.y(), 91.2);

    // 4. Test Zoom-kontroller i App
    assert_eq!(app.canvas_zoom(), 1.0);
    let _ = app.update(Message::CanvasZoomIn);
    assert!(app.canvas_zoom() > 1.0);
    let _ = app.update(Message::CanvasResetView);
    assert_eq!(app.canvas_zoom(), 1.0);
}

#[test]
fn test_orthogonal_edge_routing_and_ports() {
    use edge::features::concept_model::{DiagramEdge, DiagramNode, RelationKind};
    use edge::features::concepts::{BelongsToDomain, Concept};
    use edge::ui::edge_router::{EdgeRouter, PortSide};

    // Opret test-begreber og noder
    let c_super = Concept::new("Superklasse", "Overordnet begreb", BelongsToDomain::Yes);
    let c_sub1 = Concept::new("Subklasse1", "Underordnet begreb 1", BelongsToDomain::Yes);
    let c_sub2 = Concept::new("Subklasse2", "Underordnet begreb 2", BelongsToDomain::Yes);
    let c_assoc = Concept::new("Associeret", "Tilknyttet begreb", BelongsToDomain::Yes);

    // Node dimensioner er standard 180x80
    // Superklasse placeret øverst: x=200, y=40 (bottom er y=120)
    let node_super = DiagramNode::new(&c_super, 200.0, 40.0);
    // Subklasse 1 placeret nederst til venstre: x=100, y=240 (top er y=240)
    let node_sub1 = DiagramNode::new(&c_sub1, 100.0, 240.0);
    // Subklasse 2 placeret nederst til højre: x=300, y=240 (top er y=240)
    let node_sub2 = DiagramNode::new(&c_sub2, 300.0, 240.0);
    // Associeret placeret til højre for superklasse: x=500, y=40
    let node_assoc = DiagramNode::new(&c_assoc, 500.0, 40.0);

    let nodes = vec![
        node_super.clone(),
        node_sub1.clone(),
        node_sub2.clone(),
        node_assoc.clone(),
    ];

    // Edge 1: Generalisering sub1 -> super
    let mut edge_gen1 = DiagramEdge::new(
        node_sub1.id(),
        node_super.id(),
        RelationKind::Generalization,
    );
    edge_gen1.set_label(Some("er en".to_string())); // skal undertrykkes jf FDA

    // Edge 2: Generalisering sub2 -> super
    let edge_gen2 = DiagramEdge::new(
        node_sub2.id(),
        node_super.id(),
        RelationKind::Generalization,
    );

    // Edge 3: Association super -> assoc
    let mut edge_asc =
        DiagramEdge::new(node_super.id(), node_assoc.id(), RelationKind::Association);
    edge_asc.set_label(Some("relaterer".to_string()));

    let edges = vec![edge_gen1.clone(), edge_gen2.clone(), edge_asc.clone()];

    let routes = EdgeRouter::route_edges(&nodes, &edges);
    assert_eq!(routes.len(), 3, "Skal route alle 3 edges");

    // 1. Verificér ortogonalitet (kun 90 graders vinkler: alle segmenter er enten rent horisontale eller vertikale)
    for route in &routes {
        assert!(
            route.points.len() >= 2,
            "En rute skal have mindst 2 punkter"
        );
        for window in route.points.windows(2) {
            let p1 = window[0];
            let p2 = window[1];
            let is_horizontal = (p1.y - p2.y).abs() < 0.001;
            let is_vertical = (p1.x - p2.x).abs() < 0.001;
            assert!(
                is_horizontal || is_vertical,
                "Alle linjesegmenter skal være strengt ortogonale (90°). Segment fra {:?} til {:?}",
                p1,
                p2
            );
        }
    }

    // 2. Verificér FDA label-semantik: ingen label på generalisering, label bevares på association
    let r_gen1 = routes
        .iter()
        .find(|r| r.from == edge_gen1.from() && r.to == edge_gen1.to())
        .unwrap();
    assert_eq!(
        r_gen1.label, None,
        "Generalisering må IKKE vise label jf FDA vejledning linje 1474 & 1526"
    );

    let r_asc = routes
        .iter()
        .find(|r| r.from == edge_asc.from() && r.to == edge_asc.to())
        .unwrap();
    assert_eq!(
        r_asc.label.as_deref(),
        Some("relaterer"),
        "Association skal bevare sin label"
    );

    // 3. Verificér pilehoved: forankret præcist på målnodens kant
    let arrow1 = r_gen1
        .arrow_head
        .as_ref()
        .expect("Generalisering skal have pilehoved");
    assert_eq!(
        arrow1.direction,
        PortSide::Bottom,
        "Pil til superklasse oppefra skal ramme bundporten"
    );
    assert_eq!(
        arrow1.tip.y,
        node_super.y() + node_super.height(),
        "Pilehovedets spids skal røre målnodens bundkant præcist (y = 120.0)"
    );
    assert!(
        arrow1.left.y > arrow1.tip.y && arrow1.right.y > arrow1.tip.y,
        "Pilehovedets trekantsbase skal ligge udenfor noden (større y), ikke skjules inde i noden"
    );

    // 4. Verificér multi-relation af samme type deler anker på target
    let r_gen2 = routes
        .iter()
        .find(|r| r.from == edge_gen2.from() && r.to == edge_gen2.to())
        .unwrap();
    let arrow2 = r_gen2
        .arrow_head
        .as_ref()
        .expect("Generalisering 2 skal have pilehoved");
    assert_eq!(
        arrow1.tip, arrow2.tip,
        "To generaliseringer til samme superklasse på samme side skal dele ankerpunkt (FDA Fig 7.1)"
    );

    // 5. Test Nærhedshåndtering (Proximity Port Shift):
    // Når to noder er så tæt på hinanden at afstanden er mindre end D_min (36px),
    // må pilen IKKE routes direkte mellem modstående flader så pilen klemmes.
    let close_sub = DiagramNode::new(&c_sub1, 200.0, 130.0); // y=130, super bottom=120 -> afstand kun 10px!
    let close_nodes = vec![node_super.clone(), close_sub.clone()];
    let close_edge = DiagramEdge::new(
        close_sub.id(),
        node_super.id(),
        RelationKind::Generalization,
    );
    let close_routes = EdgeRouter::route_edges(&close_nodes, &[close_edge]);
    let close_route = &close_routes[0];
    let close_arrow = close_route
        .arrow_head
        .as_ref()
        .expect("Skal have pilehoved");
    // Da afstanden vertikalt kun er 10px, skal porten skifte til side-port for at undgå flad/inverteret pil
    assert_ne!(
        close_arrow.direction,
        PortSide::Bottom,
        "Ved kritisk nærhed (afstand < 36px) skal porten skifte til side-porte for at bevare pilerum"
    );

    // 6. Test Krydsningsbroer (Bridge hops):
    // Opret to edges der krydser hinanden ortogonalt i et X-kryds
    let n_horiz_left = DiagramNode::new(&c_sub1, 0.0, 300.0);
    let n_horiz_right = DiagramNode::new(&c_sub2, 400.0, 300.0);
    let n_vert_top = DiagramNode::new(&c_super, 200.0, 100.0);
    let n_vert_bottom = DiagramNode::new(&c_assoc, 200.0, 500.0);

    let cross_nodes = vec![
        n_horiz_left.clone(),
        n_horiz_right.clone(),
        n_vert_top.clone(),
        n_vert_bottom.clone(),
    ];
    let edge_h = DiagramEdge::new(
        n_horiz_left.id(),
        n_horiz_right.id(),
        RelationKind::Association,
    );
    let edge_v = DiagramEdge::new(
        n_vert_top.id(),
        n_vert_bottom.id(),
        RelationKind::Association,
    );

    let cross_routes = EdgeRouter::route_edges(&cross_nodes, &[edge_h, edge_v]);
    let has_bridge = cross_routes.iter().any(|r| !r.bridges.is_empty());
    assert!(
        has_bridge,
        "Når to ortogonale linjer krydser, skal der detekteres mindst én krydsningsbro"
    );
}

#[test]
fn test_task_010_information_model_classes_attributes_and_concept_traceability() {
    // 1. Opret kildebegreber i begrebsmodellen
    let c_person = Concept::new(
        "Person",
        "En fysisk person i det danske samfund.",
        BelongsToDomain::Yes,
    );
    let c_cpr = Concept::new(
        "CprNummer",
        "Et 10-cifret unikt personnummer udstedt af CPR-registret.",
        BelongsToDomain::Yes,
    );

    // 2. Opret en Informationsklasse knyttet til begrebet Person
    let mut person_class = InformationClass::from_concept(&c_person);
    assert_eq!(person_class.name(), "Person");
    assert_eq!(
        person_class.description(),
        Some("En fysisk person i det danske samfund.")
    );
    assert!(
        person_class.concept_ids().contains(&c_person.id()),
        "Klassen skal spore tilbage til person-begrebet"
    );

    // 3. Tilføj attribut med direkte begrebssporing (cprNummer -> c_cpr)
    let attr_cpr = Attribute::new(
        "cprNummer",
        PrimitiveType::CharacterString,
        Multiplicity::exactly_one(),
    )
    .with_concepts(vec![c_cpr.id()]);

    assert_eq!(attr_cpr.name(), "cprNummer");
    assert_eq!(attr_cpr.data_type(), PrimitiveType::CharacterString);
    assert_eq!(attr_cpr.multiplicity(), Multiplicity::exactly_one());
    assert!(
        attr_cpr.concept_ids().contains(&c_cpr.id()),
        "Attributten skal spore til CprNummer-begrebet"
    );
    person_class.add_attribute(attr_cpr);

    // 4. Tilføj en selvstændig attribut uden begreb (f.eks. registreringsTidspunkt)
    let attr_tid = Attribute::new(
        "registreringsTidspunkt",
        PrimitiveType::DateTime,
        Multiplicity::zero_or_one(),
    );
    assert!(
        attr_tid.concept_ids().is_empty(),
        "Selvstændig attribut har ingen begrebsrelation"
    );
    person_class.add_attribute(attr_tid);

    // 5. Opret en selvstændig teknisk klasse uden begrebsrelation
    let mut audit_class = InformationClass::new("AuditLog");
    audit_class.set_description(Some("Teknisk hændelseslog".to_string()));
    assert!(
        audit_class.concept_ids().is_empty(),
        "Selvstændig klasse skal kunne oprettes uden begrebsrelation"
    );

    // 6. Test InformationModel container
    let mut info_model = InformationModel::new();
    let person_class_id = info_model.add_class(person_class);
    let audit_class_id = info_model.add_class(audit_class);

    assert_eq!(
        info_model.classes().len(),
        2,
        "Informationsmodellen skal indeholde to klasser"
    );

    let retrieved = info_model
        .get_class(person_class_id)
        .expect("Klassen skal kunne hentes via id");
    assert_eq!(retrieved.name(), "Person");
    assert_eq!(retrieved.attributes().len(), 2);

    // Test opslag via begreb
    let classes_for_p = info_model.classes_for_concept(c_person.id());
    assert_eq!(classes_for_p.len(), 1);
    assert_eq!(classes_for_p[0].id(), person_class_id);

    let attrs_for_cpr = info_model.attributes_for_concept(c_cpr.id());
    assert_eq!(attrs_for_cpr.len(), 1);
    assert_eq!(attrs_for_cpr[0].1.name(), "cprNummer");

    // 7. Test integration i ModelProject og Disk-Persistens (Round-trip serialisering)
    let mut project = ModelProject::default();
    project.information_model_mut().add_class(
        info_model
            .get_class(person_class_id)
            .cloned()
            .expect("Skal have klasse"),
    );
    project.information_model_mut().add_class(
        info_model
            .get_class(audit_class_id)
            .cloned()
            .expect("Skal have klasse"),
    );

    let json = serde_json::to_string_pretty(&project).expect("Serialisering skal lykkes");
    let loaded: ModelProject =
        serde_json::from_str(&json).expect("Deserialisering skal genskabe ModelProject");

    assert_eq!(
        loaded.information_model().classes().len(),
        2,
        "De serialiserede klasser skal bevares intakt"
    );

    // 8. Test Bagudkompatibilitet: Deserialisering af projekt JSON uden 'information_model' felt
    let legacy_json = r#"{
        "metadata": {
            "name": "Legacy Model",
            "description": "Uden informationsmodel",
            "uri": "https://data.gov.dk/model/core/legacy",
            "responsible_org": "Myndighed",
            "domain_area": "Test",
            "version": "1.0.0",
            "status": "Draft",
            "legal_source": null
        },
        "concepts": [],
        "concept_graph": {
            "nodes": [],
            "edges": []
        }
    }"#;

    let legacy_project: ModelProject = serde_json::from_str(legacy_json).expect(
        "Bagudkompatibilitet skal sikre at legacy JSON uden information_model kan indlæses",
    );
    assert_eq!(
        legacy_project.information_model().classes().len(),
        0,
        "Legacy projekt skal initialisere en tom InformationModel via #[serde(default)]"
    );
}

#[test]
fn test_information_model_ui_crud_and_concept_linking() {
    let mut app = App::new_with_path(None);

    // 1. Skift til Informationsmodel-fanen
    let _ = app.update(Message::SelectTab(Tab::InformationModel));
    assert_eq!(app.active_tab(), Tab::InformationModel);

    // 2. Opret et begreb i projektet til sporing
    let c = Concept::new("Borger", "Fysisk person.", BelongsToDomain::Yes);
    let c_id = app.project_mut().add_concept(c).unwrap();

    // 3. Opret en ny klasse via UI
    let _ = app.update(Message::CreateInformationClass);
    assert_eq!(
        app.project().information_model().classes().len(),
        1,
        "Skal have oprettet 1 klasse"
    );

    let class_id = app.project().information_model().classes()[0].id();

    // 4. Opdater klassens navn og beskrivelse
    let _ = app.update(Message::UpdateInformationClassName(
        class_id,
        "BorgerKlasse".to_string(),
    ));
    let _ = app.update(Message::UpdateInformationClassDescription(
        class_id,
        "En informationsklasse for borgere".to_string(),
    ));

    // 5. Knyt begreb til klassen
    let _ = app.update(Message::AddConceptToInformationClass(
        class_id,
        ConceptOption {
            id: c_id,
            term: "Borger".to_string(),
        },
    ));

    let class = app
        .project()
        .information_model()
        .get_class(class_id)
        .unwrap();
    assert_eq!(class.name(), "BorgerKlasse");
    assert_eq!(
        class.description(),
        Some("En informationsklasse for borgere")
    );
    assert!(class.concept_ids().contains(&c_id));

    // 6. Tilføj attribut
    let _ = app.update(Message::AddAttributeToClass(class_id));
    let class = app
        .project()
        .information_model()
        .get_class(class_id)
        .unwrap();
    assert_eq!(class.attributes().len(), 1);
    let attr_id = class.attributes()[0].id();

    // 7. Opdater attribut oplysninger
    let _ = app.update(Message::UpdateAttributeName(
        class_id,
        attr_id,
        "cprNummer".to_string(),
    ));
    let _ = app.update(Message::UpdateAttributeType(
        class_id,
        attr_id,
        PrimitiveType::CharacterString,
    ));
    let _ = app.update(Message::UpdateAttributeMultiplicity(
        class_id,
        attr_id,
        Multiplicity::exactly_one(),
    ));

    // 8. Knyt begreb til attribut
    let _ = app.update(Message::AddConceptToAttribute(
        class_id,
        attr_id,
        ConceptOption {
            id: c_id,
            term: "Borger".to_string(),
        },
    ));

    let class = app
        .project()
        .information_model()
        .get_class(class_id)
        .unwrap();
    let attr = &class.attributes()[0];
    assert_eq!(attr.name(), "cprNummer");
    assert_eq!(attr.data_type(), PrimitiveType::CharacterString);
    assert_eq!(attr.multiplicity(), Multiplicity::exactly_one());
    assert!(attr.concept_ids().contains(&c_id));

    // 9. Verificer at view renderer uden panik
    {
        let _view = app.view();
    }

    // 10. Slet attribut
    let _ = app.update(Message::DeleteAttribute(class_id, attr_id));
    let class = app
        .project()
        .information_model()
        .get_class(class_id)
        .unwrap();
    assert_eq!(class.attributes().len(), 0);

    // 11. Slet klasse
    let _ = app.update(Message::DeleteInformationClass(class_id));
    assert_eq!(app.project().information_model().classes().len(), 0);
}

#[test]
fn test_information_model_uml_canvas_and_studio_layout() {
    let mut app = App::new_with_path(None);

    // 1. Skift til Informationsmodel fanen
    let _ = app.update(Message::SelectTab(Tab::InformationModel));
    assert_eq!(app.active_tab(), Tab::InformationModel);

    // 2. Opret to klasser i projektet
    let _ = app.update(Message::CreateInformationClass);
    let class_a_id = app.project().information_model().classes()[0].id();
    let _ = app.update(Message::UpdateInformationClassName(
        class_a_id,
        "Køretøj".to_string(),
    ));

    let _ = app.update(Message::CreateInformationClass);
    let class_b_id = app.project().information_model().classes()[1].id();
    let _ = app.update(Message::UpdateInformationClassName(
        class_b_id,
        "Personbil".to_string(),
    ));

    // 3. Tilføj klasser til diagrammet (ClassGraph)
    let _ = app.update(Message::AddClassToDiagram(class_a_id));
    let _ = app.update(Message::AddClassToDiagram(class_b_id));

    let (node_a_id, node_b_id, initial_height_a) = {
        let graph = app.project().information_graph();
        assert_eq!(graph.node_count(), 2, "Begge klasser skal være på canvas");
        let node_a = graph
            .find_node_by_class(class_a_id)
            .expect("Node A skal findes");
        let node_b = graph
            .find_node_by_class(class_b_id)
            .expect("Node B skal findes");
        let height = node_a.height();
        assert!(height >= 80.0, "UML node skal have en minimumshøjde");
        (node_a.id(), node_b.id(), height)
    };

    // 4. Tilføj attributter til Klasse A og verificer at nodens højde vokser dynamisk
    let _ = app.update(Message::AddAttributeToClass(class_a_id));
    let attr_id = app
        .project()
        .information_model()
        .get_class(class_a_id)
        .unwrap()
        .attributes()[0]
        .id();
    let _ = app.update(Message::UpdateAttributeName(
        class_a_id,
        attr_id,
        "registreringsNummer".to_string(),
    ));

    let updated_height_a = app
        .project()
        .information_graph()
        .find_node_by_class(class_a_id)
        .unwrap()
        .height();
    assert!(
        updated_height_a > initial_height_a,
        "UML node højde skal vokse dynamisk når attributter tilføjes"
    );

    // 5. Opret en generaliseringsrelation mellem Personbil -> Køretøj
    let _ = app.update(Message::AddClassRelation(
        node_b_id,
        node_a_id,
        RelationKind::Generalization,
        None,
    ));

    assert_eq!(
        app.project().information_graph().edge_count(),
        1,
        "Skal have oprettet 1 relation i informationsgrafen"
    );

    // 6. Flyt node på canvas og verificer position
    let _ = app.update(Message::UpdateClassNodePosition(node_b_id, 320.0, 240.0));
    let node_b_moved = app
        .project()
        .information_graph()
        .find_node(node_b_id)
        .unwrap();
    assert_eq!(node_b_moved.x(), 320.0);
    assert_eq!(node_b_moved.y(), 240.0);

    // 7. Verificer at Canvas Studio viewet renderer fejlfrit (Venstre palet, Canvas, Højre inspector)
    {
        let _view = app.view();
    }

    // 8. Slet Klasse A og verificer kaskadesletning i graf og relationer (Fail-Closed)
    let _ = app.update(Message::DeleteInformationClass(class_a_id));
    let graph = app.project().information_graph();
    assert_eq!(graph.node_count(), 1, "Node A skal være kaskadeslettet");
    assert_eq!(
        graph.edge_count(),
        0,
        "Relationer til Node A skal være kaskadeslettet uden hængende kanter"
    );
}

#[test]
fn test_task_012_unified_diagram_canvas_and_concept_studio_layout() {
    use edge::features::concept_model::{DiagramEdge, DiagramNode, RelationKind};
    use edge::features::concepts::{BelongsToDomain, Concept};
    use edge::features::information_model::{ClassDiagramEdge, ClassDiagramNode};
    use edge::ui::diagram_canvas::{CanvasEdge, CanvasNode};
    use uuid::Uuid;

    // 1. Verificer abstraktionerne for CanvasNode og CanvasEdge
    let c = Concept::new("Kunde", "En aftalepart", BelongsToDomain::Yes);
    let concept_node = DiagramNode::new(&c, 100.0, 150.0);
    assert_eq!(CanvasNode::id(&concept_node), concept_node.id());
    assert_eq!(CanvasNode::position(&concept_node), (100.0, 150.0));
    assert_eq!(CanvasNode::size(&concept_node), (180.0, 80.0));
    assert!(CanvasNode::contains(&concept_node, 110.0, 160.0));
    assert!(!CanvasNode::contains(&concept_node, 10.0, 10.0));

    let edge = DiagramEdge::new(
        concept_node.id(),
        Uuid::new_v4(),
        RelationKind::Generalization,
    );
    assert_eq!(CanvasEdge::kind(&edge), RelationKind::Generalization);

    let class_node = ClassDiagramNode::new(Uuid::new_v4(), 200.0, 250.0, 2);
    assert_eq!(CanvasNode::position(&class_node), (200.0, 250.0));
    assert!(CanvasNode::contains(&class_node, 220.0, 270.0));

    let class_edge = ClassDiagramEdge::new(
        class_node.id(),
        Uuid::new_v4(),
        RelationKind::Association,
        Some("kunde".to_string()),
    );
    assert_eq!(CanvasEdge::label(&class_edge), Some("kunde"));

    // 2. Initialiser App og test Canvas Studio workflow for Begrebsmodellen (Fane 3)
    let mut app = App::new_with_path(None);
    let _ = app.update(Message::SelectTab(Tab::ConceptModel));

    let c1 = Concept::new("Kunde", "En aftalepart", BelongsToDomain::Yes);
    let c2 = Concept::new("Faktura", "Et betalingskrav", BelongsToDomain::Yes);
    let c1_id = app.project_mut().add_concept(c1).unwrap();
    let c2_id = app.project_mut().add_concept(c2).unwrap();
    assert!(app.project().get_concept(c2_id).is_some());

    // Verificer is_concept_on_diagram metoden
    let node_c1_id = app
        .project()
        .concept_graph()
        .find_node_by_concept(c1_id)
        .map(|n| n.id());

    // Hvis noder er på diagrammet, test fjernelse af node uden at slette begrebet
    if let Some(n1) = node_c1_id {
        let _ = app.update(Message::RemoveConceptFromDiagram(n1));
        assert!(
            !app.project().concept_graph().is_concept_on_diagram(c1_id),
            "Noden skal være fjernet fra diagrammet"
        );
        assert!(
            app.project().get_concept(c1_id).is_some(),
            "Kernebegrebet må IKKE slettes fra projektets repository når det fjernes fra diagram"
        );
    }

    // Test tilføjelse til diagram via Message::AddConceptToDiagram
    let _ = app.update(Message::AddConceptToDiagram(c1_id));
    assert!(
        app.project().concept_graph().is_concept_on_diagram(c1_id),
        "Begrebet skal nu optræde på diagrammet"
    );

    // 3. Test søgning i Begrebsmodel Studio paletten
    let _ = app.update(Message::ConceptModelSearchChanged("fak".to_string()));

    // 4. Verificer at Canvas Studio viewet for Begrebsmodellen renderer fejlfrit (3-delt opbygning)
    {
        let _view = app.view();
    }

    // 5. Verificer at Informationsmodellen også renderer fejlfrit med DiagramCanvas
    let _ = app.update(Message::SelectTab(Tab::InformationModel));
    {
        let _view = app.view();
    }
}

#[test]
fn test_task_012_grid_resize_and_information_model_relations_inspector() {
    use edge::features::concept_model::RelationKind;
    use edge::features::information_model::InformationClass;
    use edge::ui::diagram_canvas::CanvasViewport;
    use iced::{Point, Rectangle, Size};

    // 1. Verificer at grid beregning dækker vilkårlige vinduesstørrelser uden fordoblet transformation
    let vp = CanvasViewport::new(iced::Vector::new(15.0, 25.0), 1.0);
    let bounds = Rectangle::new(Point::ORIGIN, Size::new(2560.0, 1440.0));
    let step = 20.0 * vp.zoom();
    let ox = vp.pan().x.rem_euclid(step);
    let oy = vp.pan().y.rem_euclid(step);
    let mut last_x = ox;
    while last_x + step <= bounds.width {
        last_x += step;
    }
    let mut last_y = oy;
    while last_y + step <= bounds.height {
        last_y += step;
    }
    assert!(
        last_x >= 2540.0,
        "Grid dots skal dække hele bredden ved resize"
    );
    assert!(
        last_y >= 1420.0,
        "Grid dots skal dække hele højden ved resize"
    );

    // 2. Verificer at Informationsmodellen viser tilknyttede relationer for en valgt klasse
    let mut app = App::new_with_path(None);
    let _ = app.update(Message::SelectTab(Tab::InformationModel));

    let class_a = InformationClass::new("Person");
    let class_b = InformationClass::new("OrgPerson");
    let class_a_id = class_a.id();
    let class_b_id = class_b.id();

    let _ = app.project_mut().information_model_mut().add_class(class_a);
    let _ = app.project_mut().information_model_mut().add_class(class_b);

    let node_a = app
        .project_mut()
        .information_graph_mut()
        .add_node(class_a_id, 0);
    let node_b = app
        .project_mut()
        .information_graph_mut()
        .add_node(class_b_id, 0);

    // Opret en generaliseringsrelation mellem OrgPerson -> Person
    let _ = app.update(Message::AddClassRelation(
        node_b,
        node_a,
        RelationKind::Generalization,
        None,
    ));

    // Vælg OrgPerson
    let _ = app.update(Message::SelectInformationClass(Some(class_b_id)));

    // Verificer at app.view() renderer fejlfrit med tilknyttede relationer i inspectoren
    {
        let _view = app.view();
    }

    // Slet relationen via DeleteClassRelation og verificer at den fjernes
    let _ = app.update(Message::DeleteClassRelation(node_b, node_a));
    assert_eq!(app.project().information_graph().edge_count(), 0);
}

#[test]
fn test_class_node_height_grows_in_grid_size_increments_and_aligns_with_grid() {
    use edge::features::concept_model::GRID_SIZE;
    use edge::features::information_model::InformationClass;

    let mut app = App::new_with_path(None);
    let _ = app.update(Message::SelectTab(Tab::InformationModel));

    let class = InformationClass::new("Person");
    let class_id = class.id();
    let _ = app.project_mut().information_model_mut().add_class(class);
    let node_id = app
        .project_mut()
        .information_graph_mut()
        .add_node(class_id, 0);

    // Initial node højde (0 attributter)
    let node = app
        .project()
        .information_graph()
        .find_node(node_id)
        .unwrap();
    let initial_height = node.height();
    assert_eq!(
        (initial_height % GRID_SIZE).abs(),
        0.0,
        "Initial højde skal være et multiplum af GRID_SIZE ({})",
        initial_height
    );

    // Node placeres på en grid-række (f.eks. y = 180.0)
    app.project_mut()
        .information_graph_mut()
        .update_node_position(node_id, 40.0, 180.0);
    let node = app
        .project()
        .information_graph()
        .find_node(node_id)
        .unwrap();
    let bottom_y = node.y() + node.height();
    assert_eq!(
        (bottom_y % GRID_SIZE).abs(),
        0.0,
        "Underkant skal flugte med en grid-række ({})",
        bottom_y
    );

    // Tilføj 1. attribut og verificer at højden vokser med grid-størrelsen
    let _ = app.update(Message::AddAttributeToClass(class_id));
    let node = app
        .project()
        .information_graph()
        .find_node(node_id)
        .unwrap();
    assert_eq!(
        (node.height() % GRID_SIZE).abs(),
        0.0,
        "Højde efter 1 attribut skal være et multiplum af GRID_SIZE ({})",
        node.height()
    );
    assert_eq!(
        ((node.y() + node.height()) % GRID_SIZE).abs(),
        0.0,
        "Underkant skal fortsat flugte med grid-rækker efter tilføjelse af attribut ({})",
        node.y() + node.height()
    );

    // Tilføj 2. attribut og verificer tilsvarende (Person-eksemplet fra brugeren)
    let _ = app.update(Message::AddAttributeToClass(class_id));
    let node = app
        .project()
        .information_graph()
        .find_node(node_id)
        .unwrap();
    assert_eq!(
        (node.height() % GRID_SIZE).abs(),
        0.0,
        "Højde efter 2 attributter skal være et multiplum af GRID_SIZE ({})",
        node.height()
    );
    assert_eq!(
        node.height(),
        120.0,
        "Højde for Person med 2 attributter skal være præcis 120.0 (6 * 20)"
    );
    assert_eq!(
        node.y() + node.height(),
        300.0,
        "Underkant af Person noden skal lande præcis på y=300 (15 * 20) i stedet for mellem to grid-linjer"
    );
}

#[test]
fn test_task_007_interactive_edges_drag_to_connect_and_inspector_crud() {
    let mut app = App::new_with_path(None);
    let _ = app.update(Message::SelectTab(Tab::ConceptModel));

    // 1. Opret to begreber i projektet
    let c1 = Concept::new(
        "Køretøj",
        "Transportmiddel for personer eller gods",
        BelongsToDomain::Yes,
    );
    let c2 = Concept::new(
        "Motor",
        "Drivkraftkilde for et køretøj",
        BelongsToDomain::Yes,
    );
    let id1 = c1.id();
    let id2 = c2.id();

    let _ = app.project_mut().add_concept(c1);
    let _ = app.project_mut().add_concept(c2);

    // 2. Tilføj begge begreber til begrebsdiagrammet
    let _ = app.update(Message::AddConceptToDiagram(id1));
    let _ = app.update(Message::AddConceptToDiagram(id2));

    let node1 = app
        .project()
        .concept_graph()
        .find_node_by_concept(id1)
        .unwrap()
        .id();
    let node2 = app
        .project()
        .concept_graph()
        .find_node_by_concept(id2)
        .unwrap()
        .id();

    // 3. Markér Node 1 (Køretøj)
    let _ = app.update(Message::GraphNodeSelected(Some(node1)));
    assert_eq!(app.selected_graph_node_id(), Some(node1));

    // 4. Test Invariant: Self-loop afvises (Must NOT: self-loop)
    let _ = app.update(Message::GraphEdgeCreated(node1, node1));
    assert_eq!(
        app.project().concept_graph().edge_count(),
        0,
        "Self-loop må ikke oprettes"
    );

    // 5. Drag-to-Connect: Forbind Node 1 til Node 2 -> skal automatisk oprette en Association og markere relationen
    let _ = app.update(Message::GraphEdgeCreated(node1, node2));
    assert_eq!(
        app.project().concept_graph().edge_count(),
        1,
        "En relation skal være oprettet"
    );
    let edge = &app.project().concept_graph().edges()[0];
    assert_eq!(edge.from(), node1);
    assert_eq!(edge.to(), node2);
    assert_eq!(
        edge.kind(),
        RelationKind::Association,
        "Default relationstype ved drop skal være Association"
    );

    // Relationen skal straks være markeret som et selvstændigt objekt, og nodemarkering ryddet
    assert_eq!(
        app.selected_graph_node_id(),
        None,
        "Nodemarkering skal ryddes ved kantvalg"
    );
    assert_eq!(
        app.selected_edge(),
        Some((node1, node2)),
        "Den nye relation skal være valgt"
    );

    // 6. Inspektørpanel for valgt relation: Ændre label og type
    let _ = app.update(Message::GraphUpdateEdgeLabel(
        node1,
        node2,
        "omfatter".to_string(),
    ));
    assert_eq!(
        app.project().concept_graph().edges()[0].label(),
        Some("omfatter"),
        "Associationsnavn skal opdateres i grafen"
    );

    let _ = app.update(Message::GraphUpdateEdgeKind(
        node1,
        node2,
        RelationKind::Generalization,
    ));
    assert_eq!(
        app.project().concept_graph().edges()[0].kind(),
        RelationKind::Generalization,
        "Relationstype skal kunne ændres fra Association til Generalisering"
    );

    // 7. Verify view() renderer inspektøren uden modal
    {
        let _view = app.view();
        assert!(
            !app.is_relation_dialog_open(),
            "Flimsy modal må ikke være åben"
        );
    }

    // 8. Deselect og Select på canvas
    let _ = app.update(Message::GraphEdgeSelected(None));
    assert_eq!(app.selected_edge(), None);

    let _ = app.update(Message::GraphEdgeSelected(Some((node1, node2))));
    assert_eq!(app.selected_edge(), Some((node1, node2)));

    // 9. Tastatursletning via Delete-tast
    let _ = app.update(Message::GraphDeleteSelected);
    assert_eq!(
        app.project().concept_graph().edge_count(),
        0,
        "Valgt relation skal slettes ved GraphDeleteSelected"
    );
    assert_eq!(
        app.selected_edge(),
        None,
        "Markering skal ryddes efter sletning"
    );
}

#[test]
fn test_information_model_interactive_edges_drag_to_connect_and_inspector_crud() {
    let mut app = App::new_with_path(None);
    let _ = app.update(Message::SelectTab(Tab::InformationModel));
    assert_eq!(app.active_tab(), Tab::InformationModel);

    // 1. Opret to informationsklasser på diagrammet
    let _ = app.update(Message::CreateInformationClass);
    let class1_id = app
        .selected_info_class_id()
        .expect("Klasse 1 skal oprettes");
    let node1_id = app
        .selected_info_graph_node_id()
        .expect("Node 1 skal oprettes");

    let _ = app.update(Message::CreateInformationClass);
    let class2_id = app
        .selected_info_class_id()
        .expect("Klasse 2 skal oprettes");
    let node2_id = app
        .selected_info_graph_node_id()
        .expect("Node 2 skal oprettes");

    // Navngiv klasserne
    let _ = app.update(Message::UpdateInformationClassName(
        class1_id,
        "Kunde".to_string(),
    ));
    let _ = app.update(Message::UpdateInformationClassName(
        class2_id,
        "Ordre".to_string(),
    ));

    // Ingen edge er valgt endnu
    assert_eq!(app.selected_info_edge(), None);

    // 2. Simuler Drag-to-Connect: Forbind node1 -> node2 via InfoEdgeCreated
    let _ = app.update(Message::InfoEdgeCreated(node1_id, node2_id));

    // Skal have oprettet relationen i information_graph
    assert_eq!(
        app.project().information_graph().edge_count(),
        1,
        "Relation skal oprettes i informationsgrafen"
    );
    assert_eq!(
        app.selected_info_edge(),
        Some((node1_id, node2_id)),
        "Nyoprettet relation skal være valgt i informationsmodellen"
    );
    assert_eq!(
        app.selected_info_graph_node_id(),
        None,
        "Node skal fravælges når relation oprettes"
    );

    // 3. Opdater label og type via inspektøren
    let _ = app.update(Message::InfoUpdateEdgeLabel(
        node1_id,
        node2_id,
        "afgiver".to_string(),
    ));
    assert_eq!(
        app.project().information_graph().edges()[0].label(),
        Some("afgiver")
    );

    let _ = app.update(Message::InfoUpdateEdgeKind(
        node1_id,
        node2_id,
        RelationKind::Composition,
    ));
    assert_eq!(
        app.project().information_graph().edges()[0].kind(),
        RelationKind::Composition
    );

    // 4. Test kantudvælgelse (deselect / select)
    let _ = app.update(Message::InfoEdgeSelected(None));
    assert_eq!(app.selected_info_edge(), None);

    let _ = app.update(Message::InfoEdgeSelected(Some((node1_id, node2_id))));
    assert_eq!(app.selected_info_edge(), Some((node1_id, node2_id)));

    // 5. Test sletning via Delete-tast (GraphDeleteSelected)
    let _ = app.update(Message::GraphDeleteSelected);
    assert_eq!(
        app.project().information_graph().edge_count(),
        0,
        "Relation skal slettes fra informationsgrafen ved GraphDeleteSelected"
    );
    assert_eq!(app.selected_info_edge(), None);
}

#[test]
fn test_composition_edge_has_diamond_at_source_node() {
    use edge::features::concept_model::{DiagramEdge, DiagramNode, RelationKind};
    use edge::features::concepts::{BelongsToDomain, Concept};
    use edge::ui::edge_router::{EdgeRouter, PortSide};

    let c_whole = Concept::new("Bil", "Et motorkøretøj", BelongsToDomain::Yes);
    let c_part = Concept::new("Motor", "En fremdriftsmaskine", BelongsToDomain::Yes);

    let node_whole = DiagramNode::new(&c_whole, 100.0, 100.0);
    let node_part = DiagramNode::new(&c_part, 400.0, 100.0);

    let edge_comp = DiagramEdge::new(node_whole.id(), node_part.id(), RelationKind::Composition);

    let routes = EdgeRouter::route_edges(&[node_whole.clone(), node_part.clone()], &[edge_comp]);
    assert_eq!(routes.len(), 1);
    let route = &routes[0];

    // Skal have et source_diamond forankret på source-noden (node_whole)
    let diamond = route
        .source_diamond
        .as_ref()
        .expect("Komposition skal have en diamant ved source noden");
    assert_eq!(
        diamond.direction,
        PortSide::Right,
        "Source noden (venstre) skal have diamanten pegende ud fra højre port mod part-noden"
    );
    assert_eq!(
        diamond.tip.x,
        node_whole.x() + node_whole.width(),
        "Diamantens spids skal røre source nodens højre kant præcist"
    );
    assert!(
        diamond.back.x > diamond.tip.x,
        "Diamantens bagerste spids skal pege ud i lærredet mod part-noden"
    );
}

#[test]
fn test_edges_do_not_cross_unnecessarily_when_sorted_vertically() {
    use edge::features::concept_model::{DiagramEdge, DiagramNode, RelationKind};
    use edge::features::concepts::{BelongsToDomain, Concept};
    use edge::ui::edge_router::EdgeRouter;

    let c_person = Concept::new("Person", "En person", BelongsToDomain::Yes);
    let c_cpr = Concept::new("CprPerson", "CPR person", BelongsToDomain::Yes);
    let c_bil = Concept::new("Personbil", "En bil", BelongsToDomain::No);
    let c_org = Concept::new("OrgPerson", "Organisation person", BelongsToDomain::Yes);

    let node_person = DiagramNode::new(&c_person, 100.0, 200.0);
    let node_cpr = DiagramNode::new(&c_cpr, 500.0, 80.0);
    let node_bil = DiagramNode::new(&c_bil, 500.0, 190.0);
    let node_org = DiagramNode::new(&c_org, 500.0, 300.0);

    let nodes = vec![
        node_person.clone(),
        node_cpr.clone(),
        node_bil.clone(),
        node_org.clone(),
    ];

    // Opret edges i "omvendt" rækkefølge (nederste node først, øverste node sidst)
    let edge_org = DiagramEdge::new(
        node_org.id(),
        node_person.id(),
        RelationKind::Generalization,
    );
    let edge_bil = DiagramEdge::new(node_bil.id(), node_person.id(), RelationKind::Association);
    let edge_cpr = DiagramEdge::new(node_cpr.id(), node_person.id(), RelationKind::Composition);

    let edges = vec![edge_org, edge_bil, edge_cpr];
    let routes = EdgeRouter::route_edges(&nodes, &edges);
    assert_eq!(routes.len(), 3);

    // Ingen af disse tre relationer fra parallelle noder til venstre-noden må krydse hinanden!
    let total_bridges: usize = routes.iter().map(|r| r.bridges.len()).sum();
    assert_eq!(
        total_bridges, 0,
        "Relationer må ikke krydse hinanden unødigt (forventede 0 krydsningsbroer, fik {})",
        total_bridges
    );
}

#[test]
fn test_stateful_edge_port_hysteresis_and_persistence() {
    use edge::features::concept_model::{DiagramEdge, DiagramNode, PortSide, RelationKind};
    use edge::features::concepts::{BelongsToDomain, Concept};
    use edge::ui::edge_router::EdgeRouter;

    let c_person = Concept::new("Person", "En person", BelongsToDomain::Yes);
    let c_test = Concept::new("TestKlasse", "En testklasse", BelongsToDomain::Yes);
    let c_org = Concept::new("OrgPerson", "Organisation person", BelongsToDomain::Yes);

    // Person i (100, 200), width=180, height=80 -> right=280, bottom=280
    let node_person = DiagramNode::new(&c_person, 100.0, 200.0);
    // TestKlasse i (400, 50) -> Nordøst for Person (x > 280, y < 200)
    let node_test = DiagramNode::new(&c_test, 400.0, 50.0);
    // OrgPerson i (400, 380) -> Sydøst for Person (x > 280, y > 280)
    let node_org = DiagramNode::new(&c_org, 400.0, 380.0);

    let nodes = vec![node_person.clone(), node_test.clone(), node_org.clone()];

    // 1. Initial oprettelse med låst/husket Right port
    let mut edge_test =
        DiagramEdge::new(node_person.id(), node_test.id(), RelationKind::Association);
    edge_test.set_ports(Some(PortSide::Right), Some(PortSide::Left));

    let mut edge_org = DiagramEdge::new(
        node_org.id(),
        node_person.id(),
        RelationKind::Generalization,
    );
    edge_org.set_ports(Some(PortSide::Left), Some(PortSide::Right));

    let routes = EdgeRouter::route_edges(&nodes, &[edge_test.clone(), edge_org.clone()]);
    assert_eq!(routes.len(), 2);

    // edge_test skal udgå fra Person's højre side og ramme TestKlasse's venstre side
    assert_eq!(
        routes[0].from_side,
        PortSide::Right,
        "Person skal bevare højre port i Nordøst-kvadranten"
    );
    assert_eq!(
        routes[0].to_side,
        PortSide::Left,
        "TestKlasse skal rammes på venstre side"
    );

    // edge_org (generalisering) skal ramme Person på højre side og udgå fra OrgPerson's venstre side
    assert_eq!(
        routes[1].from_side,
        PortSide::Left,
        "OrgPerson skal udgå fra venstre side mod Person"
    );
    assert_eq!(routes[1].to_side, PortSide::Right, "Person skal modtage generalisering på højre side fremfor at lave baglæns u-vending under bunden");

    // 2. Hysterese-udløser: Flyt TestKlasse ind over den vertikale linje (x < person.right)
    // Person right er 100 + 180 = 280. Sæt TestKlasse x = 150 (direkte over Person)
    let mut node_test_above = node_test.clone();
    node_test_above.set_position(150.0, 50.0);
    let routes_above = EdgeRouter::route_edges(
        &[node_person.clone(), node_test_above, node_org.clone()],
        &[edge_test.clone(), edge_org.clone()],
    );
    assert_eq!(
        routes_above[0].from_side,
        PortSide::Top,
        "Når noden trækkes ind over den vertikale grænse over Person, skal porten skifte til Top"
    );

    // 3. Persistens: Serialisering og deserialisering med serde
    let serialized =
        serde_json::to_string(&edge_test).expect("DiagramEdge skal kunne serialiseres");
    assert!(
        serialized.contains("Right"),
        "JSON skal indeholde 'Right' portside"
    );
    let deserialized: DiagramEdge =
        serde_json::from_str(&serialized).expect("DiagramEdge skal deserialiseres");
    assert_eq!(deserialized.source_port(), Some(PortSide::Right));
    assert_eq!(deserialized.target_port(), Some(PortSide::Left));

    // Bagudkompatibilitet: JSON uden porte skal deserialisere med None
    let legacy_json = format!(
        r#"{{"from":"{}","to":"{}","kind":"Association","label":null}}"#,
        node_person.id(),
        node_test.id()
    );
    let legacy_edge: DiagramEdge =
        serde_json::from_str(&legacy_json).expect("Legacy JSON skal deserialiseres uden fejl");
    assert_eq!(legacy_edge.source_port(), None);
    assert_eq!(legacy_edge.target_port(), None);
}

#[test]
fn test_directed_association_half_arrow_and_reversal() {
    use edge::features::concept_model::{DiagramEdge, DiagramNode, PortSide, RelationKind};
    use edge::features::concepts::{BelongsToDomain, Concept};
    use edge::features::information_model::ClassGraph;
    use edge::ui::edge_router::EdgeRouter;
    use iced::Point;

    let c_a = Concept::new("KlasseA", "A", BelongsToDomain::Yes);
    let c_b = Concept::new("KlasseB", "B", BelongsToDomain::Yes);

    let node_a = DiagramNode::new(&c_a, 100.0, 100.0);
    let node_b = DiagramNode::new(&c_b, 400.0, 100.0);

    // 1. Association er rettet som standard (directed == true)
    let edge_assoc = DiagramEdge::new(node_a.id(), node_b.id(), RelationKind::Association);
    assert!(edge_assoc.is_directed(), "Association skal være rettet som standard");

    let routes = EdgeRouter::route_edges(&[node_a.clone(), node_b.clone()], &[edge_assoc.clone()]);
    assert_eq!(routes.len(), 1);
    let route = &routes[0];

    // Skal have et half_arrow mod målnoden node_b (venstre port på node_b)
    let half_arrow = route.half_arrow.as_ref().expect("Rettet association skal have en halv pil");
    assert_eq!(half_arrow.tip, Point::new(node_b.x(), node_b.center().1));
    assert_eq!(half_arrow.direction, PortSide::Left);

    // 2. Kan slå pilen fra (undirected association)
    let mut edge_undirected = edge_assoc.clone();
    edge_undirected.set_directed(false);
    assert!(!edge_undirected.is_directed());
    let routes_undirected = EdgeRouter::route_edges(&[node_a.clone(), node_b.clone()], &[edge_undirected]);
    assert!(routes_undirected[0].half_arrow.is_none(), "Uorienteret association må ikke have en halv pil");

    // 3. Retningsvending (reverse_relation) i ClassGraph / ConceptGraph
    let mut graph = ClassGraph::new();
    let n1 = graph.add_node(uuid::Uuid::new_v4(), 0);
    let n2 = graph.add_node(uuid::Uuid::new_v4(), 0);
    graph.add_relation(n1, n2, RelationKind::Association, Some("forbinder".to_string()));
    graph.update_edge_ports(n1, n2, Some(PortSide::Right), Some(PortSide::Left));

    assert!(graph.find_edge(n1, n2).is_some());
    assert!(graph.find_edge(n2, n1).is_none());

    let reversed = graph.reverse_relation(n1, n2);
    assert!(reversed, "Skal kunne vende relation");
    assert!(graph.find_edge(n1, n2).is_none(), "Gammel retning skal være fjernet");
    let rev_edge = graph.find_edge(n2, n1).expect("Ny vendt relation skal findes");
    assert_eq!(rev_edge.from(), n2);
    assert_eq!(rev_edge.to(), n1);
    assert_eq!(rev_edge.source_port(), Some(PortSide::Left), "Porte skal være spejlvendt");
    assert_eq!(rev_edge.target_port(), Some(PortSide::Right), "Porte skal være spejlvendt");
    assert_eq!(rev_edge.label(), Some("forbinder"));
}

