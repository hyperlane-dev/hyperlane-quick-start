use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

const DEPENDENCY_BLOCKS: [&str; 4] = [
    "dependencies",
    "dev-dependencies",
    "build-dependencies",
    "workspace.dependencies",
];

struct Entry {
    key: String,
    length: usize,
    blank_after: bool,
}

fn manifests(root: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = Vec::new();
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        let Ok(entries) = fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries.flatten() {
            let path: PathBuf = entry.path();
            let name: String = entry.file_name().to_string_lossy().to_string();
            if name == "target" || name == ".git" || name == "node_modules" {
                continue;
            }
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if name == "Cargo.toml" {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

fn package_name(text: &str) -> Option<String> {
    let mut in_package: bool = false;
    for line in text.lines() {
        let trimmed: &str = line.trim();
        if trimmed.starts_with('[') {
            in_package = trimmed == "[package]";
            continue;
        }
        if in_package {
            if let Some(value) = trimmed.strip_prefix("name") {
                if let Some(name) = value.split('=').nth(1) {
                    return Some(name.trim().trim_matches('"').to_string());
                }
            }
        }
    }
    None
}

fn workspace_members(root: &Path, text: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut members: Vec<String> = Vec::new();
    let mut in_members: bool = false;
    for line in text.lines() {
        let trimmed: &str = line.trim();
        if trimmed.starts_with('[') {
            in_members = trimmed == "[workspace]";
            continue;
        }
        if in_members {
            if let Some(value) = trimmed.strip_prefix("members") {
                let raw: &str = value.split('=').nth(1).unwrap_or_default();
                for part in raw.split(',') {
                    let member: &str = part.trim().trim_matches('"');
                    if !member.is_empty() {
                        members.push(member.to_string());
                    }
                }
            }
        }
    }
    for member in members {
        let manifest: PathBuf = root.join(&member).join("Cargo.toml");
        if let Ok(text) = fs::read_to_string(&manifest)
            && let Some(name) = package_name(&text)
        {
            names.push(name);
        }
    }
    names
}

fn block_entries(text: &str, block: &str) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    let mut inside: bool = false;
    let mut pending: Option<(String, usize, isize)> = None;
    for line in text.lines() {
        let trimmed: &str = line.trim();
        if trimmed.starts_with('[') {
            let name: &str = trimmed.trim_start_matches('[').trim_end_matches(']');
            if inside {
                break;
            }
            inside = name == block;
            continue;
        }
        if !inside {
            continue;
        }
        if trimmed.is_empty() {
            match pending.take() {
                Some((key, length, _)) => entries.push(Entry {
                    key,
                    length,
                    blank_after: true,
                }),
                None => {
                    if let Some(entry) = entries.last_mut() {
                        entry.blank_after = true;
                    }
                }
            }
            continue;
        }
        let opens: isize = trimmed.chars().filter(|c: &char| *c == '{').count() as isize;
        let closes: isize = trimmed.chars().filter(|c: &char| *c == '}').count() as isize;
        if let Some((_, length, depth)) = pending.as_mut() {
            *length += trimmed.chars().count();
            *depth += opens;
            *depth -= closes;
            if *depth <= 0 {
                let (key, length, _) = pending.take().expect("the entry is there");
                entries.push(Entry {
                    key,
                    length,
                    blank_after: false,
                });
            }
            continue;
        }
        let key: String = trimmed
            .split('=')
            .next()
            .unwrap_or_default()
            .trim()
            .to_string();
        let depth: isize = opens - closes;
        if depth <= 0 {
            entries.push(Entry {
                key,
                length: trimmed.chars().count(),
                blank_after: false,
            });
        } else {
            pending = Some((key, trimmed.chars().count(), depth));
        }
    }
    entries
}

fn check_manifest(path: &Path, local: &HashSet<String>) -> Vec<String> {
    let mut problems: Vec<String> = Vec::new();
    let Ok(text) = fs::read_to_string(path) else {
        return problems;
    };
    for block in DEPENDENCY_BLOCKS {
        let entries: Vec<Entry> = block_entries(&text, block);
        if entries.is_empty() {
            continue;
        }
        let mut local_entries: Vec<&Entry> = entries
            .iter()
            .filter(|entry: &&Entry| local.contains(&entry.key))
            .collect();
        let mut third: Vec<&Entry> = entries
            .iter()
            .filter(|entry: &&Entry| !local.contains(&entry.key))
            .collect();
        let boundary: usize = if local_entries.is_empty() || third.is_empty() {
            0
        } else {
            1
        };
        let blanks: usize = entries
            .iter()
            .take(entries.len().saturating_sub(1))
            .filter(|entry: &&Entry| entry.blank_after)
            .count();
        if blanks != boundary {
            problems.push(format!(
                "{} [{block}] order: {blanks} blank lines inside the block, expected {boundary}",
                path.display()
            ));
        }
        local_entries.sort_by(|first: &&Entry, second: &&Entry| {
            (first.length, first.key.as_str()).cmp(&(second.length, second.key.as_str()))
        });
        third.sort_by(|first: &&Entry, second: &&Entry| {
            (first.length, first.key.as_str()).cmp(&(second.length, second.key.as_str()))
        });
        let expected: Vec<&str> = local_entries
            .iter()
            .chain(third.iter())
            .map(|entry: &&Entry| entry.key.as_str())
            .collect();
        let actual: Vec<&str> = entries
            .iter()
            .map(|entry: &Entry| entry.key.as_str())
            .collect();
        if expected != actual {
            problems.push(format!(
                "{} [{block}] order: expected {expected:?}, found {actual:?}",
                path.display()
            ));
        }
    }
    problems
}

#[test]
fn dependency_blocks_keep_their_order() {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("frontend");
    if !root.join("Cargo.toml").is_file() {
        return;
    }
    let mut local: HashSet<String> = HashSet::new();
    for manifest in manifests(&root) {
        let Ok(text) = fs::read_to_string(&manifest) else {
            continue;
        };
        if let Some(name) = package_name(&text) {
            local.insert(name);
        }
        for member in workspace_members(&root, &text) {
            local.insert(member);
        }
    }
    let mut problems: Vec<String> = Vec::new();
    for manifest in manifests(&root) {
        problems.extend(check_manifest(&manifest, &local));
    }
    assert!(
        problems.is_empty(),
        "the dependency blocks of the frontend workspace are out of order:\n{}",
        problems.join("\n")
    );
}
