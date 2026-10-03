//! Diagnostik og logning for platform, rendering og runtime evidens (ADR 013).

use std::collections::VecDeque;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

/// Omfattende system- og grafikdiagnostik snapshot.
#[derive(Debug, Clone)]
pub struct SystemDiagnostics {
    pub os: String,
    pub arch: String,
    pub app_version: String,
    pub iced_backend: String,
    pub log_path: Option<String>,
}

impl SystemDiagnostics {
    /// Formaterer en læsbar rapport velegnet til fejlfinding og udklipsholder.
    pub fn formatted_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=====================================================\n");
        report.push_str("  Kant System- og Grafikdiagnostik (ADR 013)\n");
        report.push_str("=====================================================\n");
        report.push_str(&format!("  Operativsystem: {}\n", self.os));
        report.push_str(&format!("  Arkitektur:     {}\n", self.arch));
        report.push_str(&format!("  Kant Version:   {}\n", self.app_version));
        report.push_str(&format!("  Iced Backend:   {}\n", self.iced_backend));
        if let Some(ref path) = self.log_path {
            report.push_str(&format!("  Logfil:         {}\n", path));
        } else {
            report.push_str("  Logfil:         (Ingen skriveadgang / in-memory)\n");
        }
        report.push_str("=====================================================\n");
        report
    }
}

static LOG_BUFFER: Mutex<Option<VecDeque<String>>> = Mutex::new(None);
const MAX_LOG_ENTRIES: usize = 1000;

/// Indsamler aktuelle platform- og grafikparametre.
pub fn get_system_diagnostics() -> SystemDiagnostics {
    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();
    let app_version = env!("CARGO_PKG_VERSION").to_string();
    let iced_backend = std::env::var("ICED_BACKEND")
        .unwrap_or_else(|_| "wgpu (hardware acceleration standard)".to_string());
    let log_path = get_log_file_path().map(|p| p.to_string_lossy().to_string());

    SystemDiagnostics {
        os,
        arch,
        app_version,
        iced_backend,
        log_path,
    }
}

/// Returnerer den platform-specifikke logfil-sti.
pub fn get_log_file_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            return Some(
                PathBuf::from(appdata)
                    .join("Kant")
                    .join("logs")
                    .join("kant.log"),
            );
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(data_home) = std::env::var("XDG_DATA_HOME") {
            return Some(
                PathBuf::from(data_home)
                    .join("kant")
                    .join("logs")
                    .join("kant.log"),
            );
        }
        if let Ok(home) = std::env::var("HOME") {
            return Some(
                PathBuf::from(home)
                    .join(".local")
                    .join("share")
                    .join("kant")
                    .join("logs")
                    .join("kant.log"),
            );
        }
    }

    None
}

/// Logger en informationsbesked til både in-memory buffer og lokal logfil.
pub fn log_info(target: &str, message: &str) {
    log_entry("INFO", target, message);
}

/// Logger en advarsel til både in-memory buffer og lokal logfil.
pub fn log_warn(target: &str, message: &str) {
    log_entry("WARN", target, message);
}

/// Logger en fejlbesked til både in-memory buffer og lokal logfil.
pub fn log_error(target: &str, message: &str) {
    log_entry("ERROR", target, message);
}

fn log_entry(level: &str, target: &str, message: &str) {
    let formatted = format!("[{}] [{}] {}", level, target, message);

    // 1. Gem i lokal in-memory ring-buffer
    if let Ok(mut lock) = LOG_BUFFER.lock() {
        let buffer = lock.get_or_insert_with(VecDeque::new);
        if buffer.len() >= MAX_LOG_ENTRIES {
            buffer.pop_front();
        }
        buffer.push_back(formatted.clone());
    }

    // 2. Skriv til disk logfil (hvis sti er tilgængelig og tilladt)
    if let Some(path) = get_log_file_path() {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) {
            let _ = writeln!(file, "{}", formatted);
        }
    }
}

/// Returnerer en kopi af alle nylige logbeskeder i bufferen.
pub fn get_recent_logs() -> Vec<String> {
    if let Ok(mut lock) = LOG_BUFFER.lock() {
        let buffer = lock.get_or_insert_with(VecDeque::new);
        buffer.iter().cloned().collect()
    } else {
        Vec::new()
    }
}
