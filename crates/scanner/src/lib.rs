//! Cross-platform disk scanner. Returns a tree of directories with size/file_count.
//!
//! v0.1: walkdir-based, single thread. v0.2 will swap in direct NTFS MFT read on Windows.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub file_count: u64,
    pub children: Vec<Node>,
    #[serde(default)]
    pub scaffold_id: Option<String>,
    #[serde(default)]
    pub top_extensions: Vec<ExtShare>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtShare {
    pub ext: String,
    pub bytes: u64,
    pub count: u64,
}

#[derive(Debug, Default)]
struct DirAcc {
    size: u64,
    file_count: u64,
    ext_bytes: HashMap<String, u64>,
    ext_count: HashMap<String, u64>,
}

pub struct ScanOptions {
    pub follow_symlinks: bool,
    pub max_depth: Option<usize>,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self { follow_symlinks: false, max_depth: None }
    }
}

pub fn scan<P: AsRef<Path>>(root: P) -> anyhow::Result<Node> {
    scan_with(root, ScanOptions::default())
}

pub fn scan_with<P: AsRef<Path>>(root: P, opts: ScanOptions) -> anyhow::Result<Node> {
    let root = root.as_ref().to_path_buf();
    let mut accs: HashMap<PathBuf, DirAcc> = HashMap::new();

    let mut walker = WalkDir::new(&root).follow_links(opts.follow_symlinks);
    if let Some(d) = opts.max_depth {
        walker = walker.max_depth(d);
    }

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        let ext = path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_else(|| "(none)".into());

        let mut cur = path.parent();
        while let Some(dir) = cur {
            let acc = accs.entry(dir.to_path_buf()).or_default();
            acc.size += size;
            acc.file_count += 1;
            *acc.ext_bytes.entry(ext.clone()).or_insert(0) += size;
            *acc.ext_count.entry(ext.clone()).or_insert(0) += 1;
            if dir == root || !dir.starts_with(&root) {
                break;
            }
            cur = dir.parent();
        }
    }

    Ok(build_tree(&root, &accs))
}

fn build_tree(dir: &Path, accs: &HashMap<PathBuf, DirAcc>) -> Node {
    let acc = accs.get(dir);
    let size = acc.map(|a| a.size).unwrap_or(0);
    let file_count = acc.map(|a| a.file_count).unwrap_or(0);
    let top_extensions = acc
        .map(|a| {
            let mut v: Vec<_> = a
                .ext_bytes
                .iter()
                .map(|(k, &b)| ExtShare {
                    ext: k.clone(),
                    bytes: b,
                    count: a.ext_count.get(k).copied().unwrap_or(0),
                })
                .collect();
            v.sort_by(|a, b| b.bytes.cmp(&a.bytes));
            v.truncate(8);
            v
        })
        .unwrap_or_default();

    let name = dir
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| dir.to_string_lossy().to_string());

    let mut children = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for entry in rd.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                children.push(build_tree(&entry.path(), accs));
            }
        }
    }
    children.sort_by(|a, b| b.size.cmp(&a.size));

    Node {
        name,
        path: dir.to_string_lossy().to_string(),
        is_dir: true,
        size,
        file_count,
        children,
        scaffold_id: None,
        top_extensions,
    }
}

/// Pull up to `n` sample paths from a directory, ordered shallowest-first.
pub fn sample_paths<P: AsRef<Path>>(root: P, n: usize) -> Vec<String> {
    let root = root.as_ref();
    WalkDir::new(root)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .take(n)
        .map(|e| e.path().to_string_lossy().to_string())
        .collect()
}

#[cfg(target_os = "windows")]
pub mod wiztree;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn scans_temp_dir() {
        let dir = tempdir_path();
        fs::create_dir_all(dir.join("a/b")).unwrap();
        fs::write(dir.join("a/file1.txt"), b"hello").unwrap();
        fs::write(dir.join("a/b/file2.txt"), b"world!").unwrap();

        let node = scan(&dir).unwrap();
        assert_eq!(node.size, 11);
        assert_eq!(node.file_count, 2);
        let _ = fs::remove_dir_all(&dir);
    }

    fn tempdir_path() -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "diskwise-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }
}
