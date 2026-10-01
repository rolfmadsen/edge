//! Platform-specifik initialisering og runtime-konfiguration.

/// Bestemmer den anbefalede standard Iced rendering backend baseret på operativsystem og eksisterende miljøvariabel.
///
/// På Windows vælges `tiny-skia` (software rendering) som robust standard for at undgå
/// GPU driver swapchain stalls og sorte skærme ved maksimering (særligt på Intel Iris Xe grafik).
/// Hvis brugeren allerede har specificeret `ICED_BACKEND` (fx `wgpu`), overskrives dette ikke.
pub fn resolve_default_iced_backend(
    is_windows: bool,
    existing_backend: Option<&str>,
) -> Option<&'static str> {
    if is_windows && existing_backend.is_none() {
        Some("tiny-skia")
    } else {
        None
    }
}

/// Initialiserer miljøvariabler og platformspecifikke standarder før Iced GUI-løkken starter.
pub fn initialize_platform_defaults() {
    #[cfg(target_os = "windows")]
    {
        let is_windows = true;
        let existing = std::env::var("ICED_BACKEND").ok();
        if let Some(backend) = resolve_default_iced_backend(is_windows, existing.as_deref()) {
            std::env::set_var("ICED_BACKEND", backend);
        }
    }
}
