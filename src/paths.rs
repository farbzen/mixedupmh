use std::{path::PathBuf, sync::OnceLock};

static BASE_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn init(dir: PathBuf) {
    BASE_DIR.set(dir).expect("paths::init called twice");
}

pub fn base_dir() -> PathBuf {
    BASE_DIR.get().expect("paths::init not called").clone()
}

/// Tauri's `resource_dir()` points at `target/debug`/`target/release` under
/// `cargo tauri dev`/`cargo build` (resources are only copied there by a real
/// `tauri build`), so fall back to the compile-time project root for dev.
pub fn resolve_base_dir(
    app: &tauri::AppHandle,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    use tauri::Manager;

    if let Ok(dir) = app.path().resource_dir()
        && dir.join("static").is_dir()
    {
        return Ok(dir);
    }

    Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}
