use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogResult {
    Selected(PathBuf),
    Cancelled,
    Unavailable,
}

/// Åbner native filvælger til at vælge en eksisterende FDA modelprojektfil (*.kant.json / *.edge.json / *.json)
pub fn pick_file_to_open() -> DialogResult {
    let mut cmd = std::process::Command::new("zenity");
    cmd.args([
        "--file-selection",
        "--title=Åbn FDA Modelprojekt",
        "--file-filter=FDA Modelprojekter (*.kant.json, *.edge.json, *.json) | *.kant.json *.edge.json *.json",
        "--file-filter=Alle filer | *",
    ]);

    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path_str.is_empty() {
                    return DialogResult::Selected(PathBuf::from(path_str));
                }
            }
            DialogResult::Cancelled
        }
        Err(_) => DialogResult::Unavailable,
    }
}

/// Åbner native filvælger til at gemme modelprojekt som ny fil
pub fn pick_file_to_save(default_name: Option<&str>) -> DialogResult {
    let mut cmd = std::process::Command::new("zenity");
    let filename_arg = format!("--filename={}", default_name.unwrap_or("model.kant.json"));
    cmd.args([
        "--file-selection",
        "--save",
        "--confirm-overwrite",
        "--title=Gem FDA Modelprojekt som...",
        &filename_arg,
        "--file-filter=FDA Modelprojekter (*.kant.json, *.edge.json, *.json) | *.kant.json *.edge.json *.json",
        "--file-filter=Alle filer | *",
    ]);

    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path_str.is_empty() {
                    let mut path = PathBuf::from(path_str);
                    if path.extension().is_none() {
                        path.set_extension("kant.json");
                    }
                    return DialogResult::Selected(path);
                }
            }
            DialogResult::Cancelled
        }
        Err(_) => DialogResult::Unavailable,
    }
}

/// Scanner en mappe for eksisterende *.kant.json, *.edge.json eller model*.json filer
pub fn scan_local_project_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let filename = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if filename.ends_with(".kant.json")
                    || filename.ends_with(".kant")
                    || filename.ends_with(".edge.json")
                    || filename.ends_with(".edge")
                {
                    files.push(path);
                }
            }
        }
    }
    files.sort();
    files
}
