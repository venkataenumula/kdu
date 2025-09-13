
use std::path::PathBuf;
use std::fs;
use anyhow::{Context, Result};

pub fn fx(p: &str) -> PathBuf {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // tests live at workspace root or in crate; both should have a "tests" dir
    // Navigate to workspace root if this file is included from a crate
    // but keep it robust by probing both options.
    // We'll first try <crate>/tests; if not found, try <workspace>/tests.
    let try1 = base.join("tests").join(p);
    if try1.exists() { return try1; }

    // If not, climb up until we find a "Cargo.toml" indicative of workspace
    let mut up = base.clone();
    for _ in 0..5 {
        up = up.parent().unwrap_or(up.as_path()).to_path_buf();
        if up.join("Cargo.toml").exists() && up.join("tests").exists() {
            return up.join("tests").join(p);
        }
    }
    // fallback: return the original join (may fail at runtime)
    try1
}

pub fn read_string(p: &str) -> Result<String> {
    let path = fx(p);
    let s = fs::read_to_string(&path)
        .with_context(|| format!("read_to_string: {}", path.display()))?;
    Ok(s)
}
