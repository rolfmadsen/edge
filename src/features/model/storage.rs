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
        PathBuf::from("model.kant.json")
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

    /// Gemmer et FDA modelprojekt i det dekomponerede `.kant/` format.
    pub fn save_to_directory(project: &ModelProject, root_path: &Path) -> Result<(), StorageError> {
        crate::features::model::decomposed::save_decomposed(project, root_path)
    }

    /// Indlæser et FDA modelprojekt fra et dekomponeret `.kant/` katalog.
    pub fn load_from_directory(root_path: &Path) -> Result<ModelProject, StorageError> {
        crate::features::model::decomposed::load_decomposed(root_path)
    }

    /// Automatisk format-detekterende indlæsning (støtter .kant.json fil, .kant mappe eller rodmappe).
    pub fn load(path: &Path) -> Result<ModelProject, StorageError> {
        if path.is_dir() {
            if path.file_name().and_then(|n| n.to_str()) == Some(".kant")
                || path.join(".kant").is_dir()
            {
                Self::load_from_directory(path)
            } else {
                let default_file = path.join(Self::default_project_path());
                if default_file.is_file() {
                    Self::load_from_file(&default_file)
                } else {
                    Self::load_from_directory(path)
                }
            }
        } else {
            Self::load_from_file(path)
        }
    }

    /// Automatisk format-detekterende skrivning.
    pub fn save(project: &ModelProject, path: &Path) -> Result<(), StorageError> {
        if path.is_dir() || path.file_name().and_then(|n| n.to_str()) == Some(".kant") {
            Self::save_to_directory(project, path)
        } else {
            Self::save_to_file(project, path)
        }
    }
}
