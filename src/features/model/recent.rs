use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct RecentStore {
    #[serde(default)]
    pub last_opened: Option<PathBuf>,
    #[serde(default)]
    pub recent_paths: Vec<PathBuf>,
}

impl RecentStore {
    /// Returnerer stien til konfigurationsfilen for seneste modeller.
    pub fn config_path() -> PathBuf {
        let home = crate::ui::app::user_home_dir();
        #[cfg(target_os = "windows")]
        {
            if let Ok(app_data) = std::env::var("APPDATA") {
                if !app_data.is_empty() {
                    return PathBuf::from(app_data)
                        .join("kant")
                        .join("recent_models.json");
                }
            }
        }
        #[cfg(target_os = "macos")]
        {
            if let Ok(h) = std::env::var("HOME") {
                if !h.is_empty() {
                    return PathBuf::from(h)
                        .join("Library")
                        .join("Application Support")
                        .join("kant")
                        .join("recent_models.json");
                }
            }
        }
        #[cfg(target_os = "linux")]
        {
            if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
                if !xdg.is_empty() {
                    return PathBuf::from(xdg).join("kant").join("recent_models.json");
                }
            }
        }
        home.join(".config").join("kant").join("recent_models.json")
    }

    /// Indlæser seneste modeller fra disk. Returnerer en tom liste hvis filen ikke findes.
    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(bytes) = fs::read(&path) {
            if let Ok(store) = serde_json::from_slice::<RecentStore>(&bytes) {
                return store;
            }
        }
        Self::default()
    }

    /// Gemmer seneste modeller til disk.
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::other(e.to_string()))?;
        fs::write(path, json.as_bytes())?;
        Ok(())
    }

    /// Normaliserer en modelsti (fjerner .kant/metadata.json subpath) så kun selve modelmappen eller .kant.json filen gemmes.
    pub fn normalize_path(path: &Path) -> PathBuf {
        let mut p = path.to_path_buf();
        if p.file_name().and_then(|n| n.to_str()) == Some("metadata.json") {
            if let Some(parent) = p.parent() {
                p = parent.to_path_buf();
            }
        }
        if p.file_name().and_then(|n| n.to_str()) == Some(".kant") {
            if let Some(parent) = p.parent() {
                p = parent.to_path_buf();
            }
        }
        p
    }

    /// Registrerer en åbnet modelsti og flytter den øverst på listen.
    pub fn record_opened(&mut self, path: &Path) {
        let normalized = Self::normalize_path(path);
        self.last_opened = Some(normalized.clone());
        self.recent_paths.retain(|p| p != &normalized);
        self.recent_paths.insert(0, normalized);
        if self.recent_paths.len() > 10 {
            self.recent_paths.truncate(10);
        }
        let _ = self.save();
    }

    /// Finder den bedste kandidat til automatisk genåbning ved opstart.
    pub fn get_auto_open_candidate(&self) -> Option<PathBuf> {
        if let Some(ref p) = self.last_opened {
            if p.exists() {
                return Some(p.clone());
            }
        }
        for p in &self.recent_paths {
            if p.exists() {
                return Some(p.clone());
            }
        }
        None
    }
}
