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
    let mut app = App::new();
    assert_eq!(app.active_tab(), Tab::Metadata);

    app.update(Message::SelectTab(Tab::ConceptList));
    assert_eq!(app.active_tab(), Tab::ConceptList);

    app.update(Message::SelectTab(Tab::ConceptModel));
    assert_eq!(app.active_tab(), Tab::ConceptModel);

    app.update(Message::SelectTab(Tab::InformationModel));
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

    let mut app = App::new();
    app.update(Message::SelectTab(Tab::ConceptList));

    // 1. Start nyt begreb
    app.update(Message::StartNewConcept);
    assert!(app.is_editing_concept());

    // 2. Udfyld felter
    app.update(Message::UpdateConceptField(ConceptFormField::PreferredTerm, "Personbil".to_string()));
    app.update(Message::UpdateConceptField(
        ConceptFormField::Definition,
        "Køretøj indrettet til befordring af højst 9 personer.".to_string(),
    ));
    app.update(Message::UpdateConceptField(ConceptFormField::BelongsToDomain, "Ja".to_string()));
    app.update(Message::UpdateConceptField(ConceptFormField::Source, "Færdselsloven".to_string()));
    app.update(Message::UpdateConceptField(ConceptFormField::LegalSource, "LBK nr 1324".to_string()));

    // 3. Gem begreb
    app.update(Message::SaveConcept);
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
    app.update(Message::SearchQueryChanged("Person".to_string()));
    assert_eq!(app.filtered_concepts().len(), 1);
    let _ = app.view();

    app.update(Message::SearchQueryChanged("Ukendt".to_string()));
    assert_eq!(app.filtered_concepts().len(), 0);
    let _ = app.view();

    app.update(Message::SearchQueryChanged("".to_string()));
    assert_eq!(app.filtered_concepts().len(), 1);

    // 5. Rediger begreb
    app.update(Message::EditConcept(id));
    assert!(app.is_editing_concept());
    let _ = app.view();
    app.update(Message::UpdateConceptField(ConceptFormField::PreferredTerm, "Personbil (M1)".to_string()));
    app.update(Message::SaveConcept);
    assert_eq!(app.project().concepts()[0].preferred_term(), "Personbil (M1)");

    // 6. Slet begreb
    app.update(Message::DeleteConcept(id));
    assert!(app.project().concepts().is_empty());
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
    app.update(Message::SelectTab(Tab::ConceptList));
    app.update(Message::StartNewConcept);
    app.update(Message::UpdateConceptField(ConceptFormField::PreferredTerm, "Cykelsti".to_string()));
    app.update(Message::UpdateConceptField(ConceptFormField::Definition, "Færdselsareal forbeholdt cykler.".to_string()));
    app.update(Message::SaveConcept);

    assert!(file_path.exists(), "Autosave skal have oprettet filen på disken");
    let on_disk = ProjectStorage::load_from_file(&file_path).expect("Skal kunne læse autosaved fil");
    assert_eq!(on_disk.concepts().len(), 1);
    assert_eq!(on_disk.concepts()[0].preferred_term(), "Cykelsti");

    // 2. Slet begreb -> autosave skal genskrive filen på disken
    let id = on_disk.concepts()[0].id();
    app.update(Message::DeleteConcept(id));

    let on_disk_after_del = ProjectStorage::load_from_file(&file_path).expect("Skal kunne læse efter sletning");
    assert!(on_disk_after_del.concepts().is_empty(), "Autosaved fil skal have 0 begreber efter sletning");

    // Oprydning
    let _ = std::fs::remove_file(file_path);
}


