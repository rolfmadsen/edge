use crate::features::concept_model::ConceptGraph;
use crate::features::concepts::{Concept, ConceptValidator, ValidationError};
use crate::features::information_model::{ClassGraph, InformationClass, InformationModel};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod storage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelStatus {
    Draft,
    Candidate,
    Approved,
}

impl ModelStatus {
    pub const ALL: [ModelStatus; 3] = [
        ModelStatus::Draft,
        ModelStatus::Candidate,
        ModelStatus::Approved,
    ];
}

impl std::fmt::Display for ModelStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelStatus::Draft => write!(f, "Udkast (Draft)"),
            ModelStatus::Candidate => write!(f, "Kandidat (Candidate)"),
            ModelStatus::Approved => write!(f, "Godkendt (Approved)"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelMetadata {
    name: String,
    description: String,
    uri: String,
    responsible_org: String,
    domain_area: String,
    version: String,
    status: ModelStatus,
    legal_source: Option<String>,
}

impl ModelMetadata {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        uri: impl Into<String>,
        responsible_org: impl Into<String>,
        domain_area: impl Into<String>,
        version: impl Into<String>,
        status: ModelStatus,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            uri: uri.into(),
            responsible_org: responsible_org.into(),
            domain_area: domain_area.into(),
            version: version.into(),
            status,
            legal_source: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn set_uri(&mut self, uri: impl Into<String>) {
        self.uri = uri.into();
    }

    pub fn responsible_org(&self) -> &str {
        &self.responsible_org
    }

    pub fn set_responsible_org(&mut self, responsible_org: impl Into<String>) {
        self.responsible_org = responsible_org.into();
    }

    pub fn domain_area(&self) -> &str {
        &self.domain_area
    }

    pub fn set_domain_area(&mut self, domain_area: impl Into<String>) {
        self.domain_area = domain_area.into();
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn set_version(&mut self, version: impl Into<String>) {
        self.version = version.into();
    }

    pub fn status(&self) -> ModelStatus {
        self.status
    }

    pub fn set_status(&mut self, status: ModelStatus) {
        self.status = status;
    }

    pub fn legal_source(&self) -> Option<&str> {
        self.legal_source.as_deref()
    }

    pub fn set_legal_source(&mut self, source: Option<String>) {
        self.legal_source = source;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelProject {
    metadata: ModelMetadata,
    concepts: Vec<Concept>,
    #[serde(default)]
    concept_graph: ConceptGraph,
    #[serde(default)]
    information_model: InformationModel,
    #[serde(default)]
    information_graph: ClassGraph,
}

impl ModelProject {
    pub fn new(metadata: ModelMetadata) -> Self {
        Self {
            metadata,
            concepts: Vec::new(),
            concept_graph: ConceptGraph::new(),
            information_model: InformationModel::new(),
            information_graph: ClassGraph::new(),
        }
    }

    pub fn metadata(&self) -> &ModelMetadata {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut ModelMetadata {
        &mut self.metadata
    }

    pub fn concepts(&self) -> &[Concept] {
        &self.concepts
    }

    pub fn concepts_mut(&mut self) -> &mut Vec<Concept> {
        &mut self.concepts
    }

    pub fn concept_graph(&self) -> &ConceptGraph {
        &self.concept_graph
    }

    pub fn concept_graph_mut(&mut self) -> &mut ConceptGraph {
        &mut self.concept_graph
    }

    pub fn information_model(&self) -> &InformationModel {
        &self.information_model
    }

    pub fn information_model_mut(&mut self) -> &mut InformationModel {
        &mut self.information_model
    }

    pub fn information_graph(&self) -> &ClassGraph {
        &self.information_graph
    }

    pub fn information_graph_mut(&mut self) -> &mut ClassGraph {
        &mut self.information_graph
    }

    pub fn remove_information_class(&mut self, id: Uuid) -> Option<InformationClass> {
        let removed = self.information_model.remove_class(id);
        if removed.is_some() {
            self.information_graph.remove_class_node(id);
        }
        removed
    }

    pub fn sync_concept_graph(&mut self) {
        self.concept_graph.sync_with_concepts(&self.concepts);
    }

    pub fn sync_information_graph(&mut self) {
        self.information_graph
            .sync_with_information_model(&self.information_model);
    }

    pub fn get_concept(&self, id: Uuid) -> Option<&Concept> {
        self.concepts.iter().find(|c| c.id() == id)
    }

    pub fn get_concept_mut(&mut self, id: Uuid) -> Option<&mut Concept> {
        self.concepts.iter_mut().find(|c| c.id() == id)
    }

    pub fn add_concept(&mut self, concept: Concept) -> Result<Uuid, ValidationError> {
        ConceptValidator::validate(&concept)?;
        let id = concept.id();
        self.concepts.push(concept);
        self.sync_concept_graph();
        Ok(id)
    }

    pub fn update_concept(&mut self, concept: Concept) -> Result<(), ValidationError> {
        ConceptValidator::validate(&concept)?;
        if let Some(existing) = self.concepts.iter_mut().find(|c| c.id() == concept.id()) {
            *existing = concept;
            self.sync_concept_graph();
            Ok(())
        } else {
            Err(ValidationError::MissingRequiredField(
                "Begreb ikke fundet i projekt",
            ))
        }
    }

    pub fn remove_concept(&mut self, id: Uuid) -> Option<Concept> {
        if let Some(idx) = self.concepts.iter().position(|c| c.id() == id) {
            let removed = self.concepts.remove(idx);
            self.concept_graph.remove_node_by_concept(id);
            Some(removed)
        } else {
            None
        }
    }
}

impl Default for ModelProject {
    fn default() -> Self {
        Self::new(ModelMetadata::new(
            "Nyt FDA Modelprojekt",
            "Beskrivelse af modelprojektet",
            "https://data.gov.dk/model/core/new-model",
            "Ansvarlig Myndighed",
            "Emneområde",
            "0.1.0",
            ModelStatus::Draft,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_metadata_setters_and_status() {
        let mut meta = ModelMetadata::new(
            "Orig",
            "Desc",
            "https://orig",
            "Org",
            "Domain",
            "1.0.0",
            ModelStatus::Draft,
        );

        meta.set_name("Updated Name");
        meta.set_description("Updated Desc");
        meta.set_uri("https://updated");
        meta.set_responsible_org("Updated Org");
        meta.set_domain_area("Updated Domain");
        meta.set_version("2.0.0");
        meta.set_status(ModelStatus::Approved);

        assert_eq!(meta.name(), "Updated Name");
        assert_eq!(meta.description(), "Updated Desc");
        assert_eq!(meta.uri(), "https://updated");
        assert_eq!(meta.responsible_org(), "Updated Org");
        assert_eq!(meta.domain_area(), "Updated Domain");
        assert_eq!(meta.version(), "2.0.0");
        assert_eq!(meta.status(), ModelStatus::Approved);

        assert_eq!(ModelStatus::ALL.len(), 3);
        assert_eq!(format!("{}", ModelStatus::Draft), "Udkast (Draft)");
        assert_eq!(
            format!("{}", ModelStatus::Candidate),
            "Kandidat (Candidate)"
        );
        assert_eq!(format!("{}", ModelStatus::Approved), "Godkendt (Approved)");
    }
}
