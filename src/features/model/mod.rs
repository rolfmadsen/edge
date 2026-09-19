use crate::features::concept_model::ConceptGraph;
use crate::features::concepts::{Concept, ConceptValidator, ValidationError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod storage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelStatus {
    Draft,
    Candidate,
    Approved,
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

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn responsible_org(&self) -> &str {
        &self.responsible_org
    }

    pub fn domain_area(&self) -> &str {
        &self.domain_area
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn status(&self) -> ModelStatus {
        self.status
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
}

impl ModelProject {
    pub fn new(metadata: ModelMetadata) -> Self {
        Self {
            metadata,
            concepts: Vec::new(),
            concept_graph: ConceptGraph::new(),
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

    pub fn sync_concept_graph(&mut self) {
        self.concept_graph.sync_with_concepts(&self.concepts);
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
