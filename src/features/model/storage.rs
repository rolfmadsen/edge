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
        let mut project: ModelProject = serde_json::from_str(&content)?;

        for concept in project.concepts() {
            ConceptValidator::validate(concept)?;
        }

        project.migrate_information_graph_edges_to_model();
        project.sync_information_graph();

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

    /// Returnerer den overordnede modelmappe, hvis den angivne sti peger på en fil inde i .kant/ kataloget.
    pub fn effective_model_path(path: &Path) -> PathBuf {
        if path.is_file() {
            if path.file_name().and_then(|n| n.to_str()) == Some("metadata.json") {
                if let Some(parent) = path.parent() {
                    if parent.file_name().and_then(|n| n.to_str()) == Some(".kant") {
                        if let Some(root) = parent.parent() {
                            return root.to_path_buf();
                        }
                    }
                    return parent.to_path_buf();
                }
            } else if let Some(parent) = path.parent() {
                if parent.file_name().and_then(|n| n.to_str()) == Some(".kant") {
                    if let Some(root) = parent.parent() {
                        return root.to_path_buf();
                    }
                    return parent.to_path_buf();
                } else if let Some(grandparent) = parent.parent() {
                    if grandparent.file_name().and_then(|n| n.to_str()) == Some(".kant") {
                        if let Some(root) = grandparent.parent() {
                            return root.to_path_buf();
                        }
                        return grandparent.to_path_buf();
                    }
                }
            }
        }
        path.to_path_buf()
    }

    /// Automatisk format-detekterende indlæsning (støtter .kant.json fil, .kant mappe eller rodmappe).
    pub fn load(path: &Path) -> Result<ModelProject, StorageError> {
        let eff = Self::effective_model_path(path);
        if eff.is_dir() {
            if eff.file_name().and_then(|n| n.to_str()) == Some(".kant")
                || eff.join(".kant").is_dir()
            {
                Self::load_from_directory(&eff)
            } else {
                let default_file = eff.join(Self::default_project_path());
                if default_file.is_file() {
                    Self::load_from_file(&default_file)
                } else {
                    Self::load_from_directory(&eff)
                }
            }
        } else {
            Self::load_from_file(&eff)
        }
    }

    /// Automatisk format-detekterende skrivning.
    pub fn save(project: &ModelProject, path: &Path) -> Result<(), StorageError> {
        let eff = Self::effective_model_path(path);
        if eff.is_dir() || eff.file_name().and_then(|n| n.to_str()) == Some(".kant") {
            Self::save_to_directory(project, &eff)
        } else {
            Self::save_to_file(project, &eff)
        }
    }
}
