//! Minimal WizTree integration for Windows.
//!
//! v0.1: find WizTree64.exe, run it on C:\, export CSV, verify the file exists.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Default install locations for WizTree64.exe.
const SEARCH_DIRS: &[&str] = &[
    r"C:\Program Files\WizTree",
    r"C:\Program Files (x86)\WizTree",
];

/// Look for `WizTree64.exe` in common install directories.
pub fn find_wiztree_executable() -> Option<PathBuf> {
    // Check %LOCALAPPDATA%\WizTree first (per-user installs).
    if let Some(localappdata) = std::env::var_os("LOCALAPPDATA") {
        let p = Path::new(&localappdata).join("WizTree").join("WizTree64.exe");
        if p.exists() {
            return Some(p);
        }
    }
    // Check default Program Files locations.
    for dir in SEARCH_DIRS {
        let p = Path::new(dir).join("WizTree64.exe");
        if p.exists() {
            return Some(p);
        }
    }
    None
}

/// Run WizTree on C:\ and export a CSV to `output_csv`.
///
/// Blocks until WizTree exits. Returns an error if:
/// - the process fails to spawn,
/// - the process exits with a non-zero status,
/// - the output CSV is missing or empty.
pub fn export_c_drive_csv(wiztree_path: &Path, output_csv: &Path) -> anyhow::Result<()> {
    let status = Command::new(wiztree_path)
        .arg(r"C:\")
        .arg(format!("/export={}", output_csv.display()))
        .status()?;

    if !status.success() {
        anyhow::bail!(
            "WizTree exited with code {:?}",
            status.code()
        );
    }

    // Verify CSV was actually written.
    let meta = std::fs::metadata(output_csv)
        .map_err(|e| anyhow::anyhow!("CSV file not found at {}: {e}", output_csv.display()))?;

    if meta.len() == 0 {
        anyhow::bail!("CSV file is empty: {}", output_csv.display());
    }

    Ok(())
}
