use std::path::PathBuf;
use std::sync::OnceLock;

// Global static for parent directory path
static PARENT_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Initialize the parent directory based on compile mode
pub(crate) fn init_parent_dir() {
    #[cfg(debug_assertions)]
    {
        // For debug builds (cargo run)
        PARENT_DIR.get_or_init(|| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "")));
        println!(
            "Running in debug mode, parent directory set to: {:?}",
            PARENT_DIR.get().unwrap()
        );
    }

    #[cfg(not(debug_assertions))]
    {
        use std::path::Path;
        // For release builds
        // Get the executable's directory and use its parent
        PARENT_DIR.get_or_init(|| {
            let exe_path = std::env::current_exe().expect("Failed to get executable path");
            exe_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf()
        });
        println!(
            "Running in release mode, parent directory set to: {:?}",
            PARENT_DIR.get().unwrap()
        );
    }
}

/// Get path relative to parent directory
pub fn get_path(relative_path: &str) -> PathBuf {
    let parent = PARENT_DIR
        .get()
        .expect("Parent directory is not initialized");
    parent.join(relative_path)
}
