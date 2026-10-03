//! Platform-specifik initialisering og runtime-konfiguration.

/// Bestemmer den anbefalede standard Iced rendering backend baseret på operativsystem og eksisterende miljøvariabel.
///
/// Ifølge ADR 013 benyttes standard WGPU hardware-acceleration på alle platforme (inkl. Windows).
/// Funktionen returnerer `None`, så Iced frit kan vælge systemets hardware-renderer.
/// Eventuel eksplicit `ICED_BACKEND` (fx `tiny-skia` sat af brugeren for fejlsøgning) respekteres og overskrives ikke.
pub fn resolve_default_iced_backend(
    _is_windows: bool,
    _existing_backend: Option<&str>,
) -> Option<&'static str> {
    None
}

/// Initialiserer miljøvariabler og platformspecifikke standarder før Iced GUI-løkken starter.
pub fn initialize_platform_defaults() {
    let existing = std::env::var("ICED_BACKEND").ok();
    crate::features::diagnostics::log_info(
        "platform",
        &format!(
            "Initialiserer platform. OS: {}, Arch: {}, Eksisterende ICED_BACKEND: {:?}",
            std::env::consts::OS,
            std::env::consts::ARCH,
            existing
        ),
    );

    #[cfg(target_os = "windows")]
    {
        let is_windows = true;
        if let Some(backend) = resolve_default_iced_backend(is_windows, existing.as_deref()) {
            std::env::set_var("ICED_BACKEND", backend);
            crate::features::diagnostics::log_info(
                "platform",
                &format!("Overstyrede ICED_BACKEND til {}", backend),
            );
        }
    }
}
