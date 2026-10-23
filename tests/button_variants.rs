use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

const ROW_CLASS: &str = "c_hls_button_row()";

const PRIMARY_ACTIONS: [(&str, &str); 10] = [
    ("frontend/pages/chat/src/page/view/fn.rs", "LABEL_SEND"),
    ("frontend/pages/cicd/src/page/view/fn.rs", "LABEL_DETAIL"),
    (
        "frontend/pages/monitor/src/page/view/fn.rs",
        "LABEL_REFRESH",
    ),
    (
        "frontend/pages/notification/src/page/view/fn.rs",
        "LABEL_MARK_READ",
    ),
    (
        "frontend/pages/order/src/page/view/fn.rs",
        "LABEL_SEARCH_BUTTON",
    ),
    ("frontend/pages/redis/src/page/view/fn.rs", "LABEL_REFRESH"),
    ("frontend/pages/rss/src/page/view/fn.rs", "LABEL_REFRESH"),
    (
        "frontend/pages/tracking/src/page/view/fn.rs",
        "LABEL_DETAIL",
    ),
    (
        "frontend/pages/user/src/page/view/fn.rs",
        "LABEL_SEARCH_BUTTON",
    ),
    ("frontend/pages/user/src/page/view/fn.rs", "LABEL_OPEN"),
];

fn page_sources(root: &Path) -> Vec<PathBuf> {
    let pages: PathBuf = root.join("frontend").join("pages");
    let Ok(entries) = fs::read_dir(&pages) else {
        return Vec::new();
    };
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if !path.is_dir() {
            continue;
        }
        collect_rs(&path, &mut files);
    }
    files.sort();
    files
}

fn collect_rs(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            collect_rs(&path, files);
        } else if path.to_string_lossy().ends_with(".rs") {
            files.push(path);
        }
    }
}

fn rows_of(source: &str) -> Vec<Vec<(String, String)>> {
    let mut rows: Vec<Vec<(String, String)>> = Vec::new();
    let mut rest: &str = source;
    while let Some(start) = rest.find(ROW_CLASS) {
        let after: &str = &rest[start + ROW_CLASS.len()..];
        let end: usize = match after.find("class: c_") {
            Some(index) => index,
            None => after.len(),
        };
        let chunk: &str = &after[..end];
        let mut buttons: Vec<(String, String)> = Vec::new();
        for piece in chunk.split("euv_button {").skip(1) {
            let variant: String = piece
                .lines()
                .find(|line: &&str| line.trim_start().starts_with("variant:"))
                .unwrap_or("")
                .trim()
                .to_string();
            let label: String = piece
                .lines()
                .find(|line: &&str| {
                    let trimmed: &str = line.trim();
                    trimmed.starts_with("LABEL_") || trimmed.starts_with('"')
                })
                .unwrap_or("")
                .trim()
                .trim_end_matches(',')
                .to_string();
            buttons.push((variant, label));
        }
        if !buttons.is_empty() {
            rows.push(buttons);
        }
        rest = &rest[start + ROW_CLASS.len() + end..];
    }
    rows
}

fn buttons_of(source: &str) -> Vec<(String, String)> {
    let mut buttons: Vec<(String, String)> = Vec::new();
    for piece in source.split("euv_button {").skip(1) {
        let variant: String = piece
            .lines()
            .find(|line: &&str| line.trim_start().starts_with("variant:"))
            .unwrap_or("")
            .trim()
            .to_string();
        let label: String = piece
            .lines()
            .find(|line: &&str| {
                let trimmed: &str = line.trim();
                trimmed.starts_with("LABEL_") || trimmed.starts_with('"')
            })
            .unwrap_or("")
            .trim()
            .trim_end_matches(',')
            .to_string();
        buttons.push((variant, label));
    }
    buttons
}

fn is_primary(variant: &str) -> bool {
    let tail: &str = variant.rsplit(':').next().unwrap_or(variant).trim();
    tail == "Primary" || tail == "VARIANT_PRIMARY"
}

fn primary_labels_of(variant: &str) -> bool {
    is_primary(variant)
}

#[test]
fn a_row_of_buttons_holds_at_most_one_primary_button() {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    let mut offenders: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for file in page_sources(&root) {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };
        for row in rows_of(&source) {
            let mut primaries: Vec<String> = Vec::new();
            for (variant, label) in &row {
                if primary_labels_of(variant) {
                    primaries.push(label.clone());
                }
            }
            if primaries.len() > 1 {
                offenders
                    .entry(format!(
                        "{}",
                        file.strip_prefix(&root).unwrap_or(&file).display()
                    ))
                    .or_default()
                    .extend(primaries);
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "a row of buttons must hold at most one primary button: {offenders:?}"
    );
}

#[test]
fn the_action_of_a_row_wears_the_primary_variant() {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    let mut offenders: Vec<String> = Vec::new();
    for (file, label) in PRIMARY_ACTIONS {
        let path: PathBuf = root.join(file);
        let Ok(source) = fs::read_to_string(&path) else {
            offenders.push(format!("{file} is missing"));
            continue;
        };
        let mut found: Vec<(String, String)> = Vec::new();
        for button in buttons_of(&source) {
            if button.1 == label {
                found.push(button);
            }
        }
        if found.is_empty() {
            offenders.push(format!("{file} has no button named {label}"));
            continue;
        }
        for (variant, _) in found {
            if !is_primary(&variant) {
                offenders.push(format!("{file}: {label} wears {variant}"));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "the action of a row must wear the primary variant: {offenders:?}"
    );
}
