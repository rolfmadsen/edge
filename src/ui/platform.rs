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

/// Bestemmer den anbefalede standard WGPU backend (fx `dx12` på Windows) hvis ikke sat.
///
/// På Windows tvinges `dx12` som standard for at forhindre WGPU i at enumerere
/// eventuelt defekte eller langsomme Vulkan ICD-drivere på Intel Iris Xe grafik (Task 066).
pub fn resolve_default_wgpu_backend(
    is_windows: bool,
    existing: Option<&str>,
) -> Option<&'static str> {
    if is_windows && existing.is_none() {
        Some("dx12")
    } else {
        None
    }
}

/// Initialiserer miljøvariabler og platformspecifikke standarder før Iced GUI-løkken starter.
pub fn initialize_platform_defaults() {
    crate::features::diagnostics::init_process_timer();
    crate::features::diagnostics::record_startup_milestone("platform_init");

    let existing_iced = std::env::var("ICED_BACKEND").ok();
    let existing_wgpu = std::env::var("WGPU_BACKEND").ok();

    crate::features::diagnostics::log_info(
        "platform",
        &format!(
            "Initialiserer platform. OS: {}, Arch: {}, ICED_BACKEND: {:?}, WGPU_BACKEND: {:?}",
            std::env::consts::OS,
            std::env::consts::ARCH,
            existing_iced,
            existing_wgpu
        ),
    );

    #[cfg(target_os = "windows")]
    {
        let is_windows = true;
        if let Some(backend) = resolve_default_iced_backend(is_windows, existing_iced.as_deref()) {
            std::env::set_var("ICED_BACKEND", backend);
            crate::features::diagnostics::log_info(
                "platform",
                &format!("Overstyrede ICED_BACKEND til {}", backend),
            );
        }
        if let Some(backend) = resolve_default_wgpu_backend(is_windows, existing_wgpu.as_deref()) {
            std::env::set_var("WGPU_BACKEND", backend);
            crate::features::diagnostics::log_info(
                "platform",
                &format!("Aktiverede WGPU DX12 fast-path: WGPU_BACKEND={}", backend),
            );
        }
    }
}
