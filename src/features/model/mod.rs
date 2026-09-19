use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelProject {
    metadata: ModelMetadata,
}

impl ModelProject {
    pub fn new(metadata: ModelMetadata) -> Self {
        Self { metadata }
    }

    pub fn metadata(&self) -> &ModelMetadata {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut ModelMetadata {
        &mut self.metadata
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
