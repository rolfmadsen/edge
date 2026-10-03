use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogResult {
    Selected(PathBuf),
    Cancelled,
    Unavailable,
}

/// Åbner native filvælger til at vælge en eksisterende FDA modelprojektfil (*.kant.json / *.edge.json / *.json)
pub fn pick_file_to_open() -> DialogResult {
    let dialog = rfd::FileDialog::new()
        .set_title("Åbn FDA Modelprojekt")
        .add_filter(
            "FDA Modelprojekter (*.kant.json, *.edge.json, *.json)",
            &["kant.json", "edge.json", "json"],
        )
        .add_filter("Alle filer", &["*"]);

    match dialog.pick_file() {
        Some(path) => DialogResult::Selected(path),
        None => DialogResult::Cancelled,
    }
}

/// Åbner native filvælger til at vælge en Excel-fil (*.xlsx, *.xls) til import
pub fn pick_excel_file_to_import() -> DialogResult {
    let dialog = rfd::FileDialog::new()
        .set_title("Vælg Excel-regneark med begreber (*.xlsx, *.xls)")
        .add_filter("Excel-arbejdsbøger (*.xlsx, *.xls)", &["xlsx", "xls"])
        .add_filter("Alle filer", &["*"]);

    match dialog.pick_file() {
        Some(path) => DialogResult::Selected(path),
        None => DialogResult::Cancelled,
    }
}

/// Åbner native filvælger til at gemme modelprojekt som ny fil
pub fn pick_file_to_save(default_name: Option<&str>) -> DialogResult {
    let mut dialog = rfd::FileDialog::new()
        .set_title("Gem FDA Modelprojekt som...")
        .add_filter(
            "FDA Modelprojekter (*.kant.json, *.edge.json, *.json)",
            &["kant.json", "edge.json", "json"],
        )
        .add_filter("Alle filer", &["*"]);

    if let Some(name) = default_name {
        dialog = dialog.set_file_name(name);
    } else {
        dialog = dialog.set_file_name("model.kant.json");
    }

    match dialog.save_file() {
        Some(mut path) => {
            if path.extension().is_none() {
                path.set_extension("kant.json");
            }
            DialogResult::Selected(path)
        }
        None => DialogResult::Cancelled,
    }
}

/// Åbner native filvælger til eksport af SVG, CSV eller modelrapporter
pub fn pick_file_to_export(default_name: &str, filter_name: &str, extension: &str) -> DialogResult {
    let dialog = rfd::FileDialog::new()
        .set_title(format!("Eksporter {}", filter_name))
        .add_filter(filter_name, &[extension])
        .add_filter("Alle filer", &["*"])
        .set_file_name(default_name);

    match dialog.save_file() {
        Some(mut path) => {
            if path.extension().is_none() {
                path.set_extension(extension);
            }
            DialogResult::Selected(path)
        }
        None => DialogResult::Cancelled,
    }
}

/// Åbner native mappevælger til at vælge en destinationsmappe til f.eks. Git-kloning
pub fn pick_folder() -> DialogResult {
    let dialog = rfd::FileDialog::new().set_title("Vælg mappe til model");
    match dialog.pick_folder() {
        Some(path) => DialogResult::Selected(path),
        None => DialogResult::Cancelled,
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
