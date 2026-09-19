use crate::features::concepts::{ConceptValidator, ValidationError};
use crate::features::model::ModelProject;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("I/O fejl: {0}")]
    Io(#[from] io::Error),

    #[error("Serialiseringsfejl: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Valideringsfejl ved indlæsning: {0}")]
    Validation(#[from] ValidationError),
}

pub struct ProjectStorage;

impl ProjectStorage {
    pub fn default_project_path() -> PathBuf {
        PathBuf::from("model.edge.json")
    }

    /// Gemmer et FDA modelprojekt deterministisk og atomisk til disk.
    pub fn save_to_file(project: &ModelProject, path: &Path) -> Result<(), StorageError> {
        // Valider samtlige begreber før skrivning (Fail-Closed invariant)
        for concept in project.concepts() {
            ConceptValidator::validate(concept)?;
        }

        let json = serde_json::to_string_pretty(project)?;

        // Opret overordnede mapper hvis nødvendigt
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        // Atomisk skrivning: skriv til midlertidig fil og omdøb
        let tmp_file_name = format!(
            ".{}.{}.tmp",
            path.file_name().and_then(|n| n.to_str()).unwrap_or("model"),
            uuid::Uuid::new_v4()
        );
        let tmp_path = path.with_file_name(tmp_file_name);

        fs::write(&tmp_path, json.as_bytes())?;
        fs::rename(&tmp_path, path)?;

        Ok(())
    }

    /// Indlæser et FDA modelprojekt fra disk og validerer samtlige begreber.
    pub fn load_from_file(path: &Path) -> Result<ModelProject, StorageError> {
        let content = fs::read_to_string(path)?;
        let project: ModelProject = serde_json::from_str(&content)?;

        for concept in project.concepts() {
            ConceptValidator::validate(concept)?;
        }

        Ok(project)
    }
}
