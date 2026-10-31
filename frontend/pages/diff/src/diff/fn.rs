use super::*;

/// Splits a text into its lines.
///
/// The empty line a text ends with is dropped, so a text and the same text
/// with a final newline compare as equal.
///
/// # Arguments
///
/// - `&str` - The text to split.
///
/// # Returns
///
/// - `Vec<&str>` - The lines of the text.
pub fn split_lines(text: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = text.split('\n').collect();
    if lines.last().is_some_and(|line: &&str| line.is_empty()) {
        lines.pop();
    }
    lines
}

/// Splits a line into its words.
///
/// The separator stays attached to the word it follows, so the segments of a
/// word comparison still add up to the line they were cut from.
///
/// # Arguments
///
/// - `&str` - The line to split.
///
/// # Returns
///
/// - `Vec<&str>` - The words of the line.
pub fn split_words(line: &str) -> Vec<&str> {
    let mut words: Vec<&str> = Vec::new();
    let bytes: &[u8] = line.as_bytes();
    let mut start: usize = 0;
    let mut index: usize = 0;
    while index < bytes.len() {
        if !is_boundary(bytes[index]) {
            index += 1;
            continue;
        }
        let mut end: usize = index + 1;
        while end < bytes.len() && is_boundary(bytes[end]) {
            end += 1;
        }
        words.push(&line[start..end]);
        start = end;
        index = end;
    }
    if start < line.len() {
        words.push(&line[start..]);
    }
    words
}

/// Compares two sequences and returns the segments of their difference.
///
/// The comparison walks the two sequences along the diagonals of the Myers
/// table, so it reports the shortest edit script between them.
///
/// # Arguments
///
/// - `&[&str]` - The original sequence.
/// - `&[&str]` - The modified sequence.
///
/// # Returns
///
/// - `Vec<DiffPart>` - The segments of the difference.
pub fn myers_diff(original: &[&str], modified: &[&str]) -> Vec<DiffPart> {
    let old_len: usize = original.len();
    let new_len: usize = modified.len();
    if old_len == 0 && new_len == 0 {
        return Vec::new();
    }
    if old_len == 0 {
        return modified
            .iter()
            .map(|line: &&str| DiffPart {
                kind: DiffKind::Added,
                value: (*line).to_string(),
            })
            .collect();
    }
    if new_len == 0 {
        return original
            .iter()
            .map(|line: &&str| DiffPart {
                kind: DiffKind::Removed,
                value: (*line).to_string(),
            })
            .collect();
    }
    let max: usize = old_len + new_len;
    let center: usize = max;
    let size: usize = TABLE_SLOTS_PER_UNIT * max + 1;
    let mut table: Vec<isize> = vec![0; size];
    let mut trace: Vec<Vec<isize>> = Vec::new();
    let mut found: bool = false;
    let mut depth: usize = 0;
    while depth <= max && depth <= MAX_DIAGONALS && !found {
        trace.push(table.clone());
        let mut diagonal: isize = -(depth as isize);
        while diagonal <= depth as isize {
            let slot: usize = slot_of(center, diagonal);
            let mut x: isize = if diagonal == -(depth as isize)
                || (diagonal != depth as isize && table[slot - 1] < table[slot + 1])
            {
                table[slot + 1]
            } else {
                table[slot - 1] + 1
            };
            let mut y: isize = x - diagonal;
            while (x as usize) < old_len
                && (y as usize) < new_len
                && original[x as usize] == modified[y as usize]
            {
                x += 1;
                y += 1;
            }
            table[slot] = x;
            if x as usize >= old_len && y as usize >= new_len {
                found = true;
                break;
            }
            diagonal += 2;
        }
        depth += 1;
    }
    if !found {
        return fallback_diff(original, modified);
    }
    backtrack(original, modified, &trace, center)
}

/// Compares two lines word by word.
///
/// # Arguments
///
/// - `&str` - The original line.
/// - `&str` - The modified line.
///
/// # Returns
///
/// - `Vec<DiffPart>` - The segments of the difference.
pub fn word_diff(original: &str, modified: &str) -> Vec<DiffPart> {
    let original_words: Vec<&str> = split_words(original);
    let modified_words: Vec<&str> = split_words(modified);
    myers_diff(&original_words, &modified_words)
}

/// Builds the rows of the result of a comparison.
///
/// The rows hold the segments of the original text and of the modified text
/// side by side, so the two panes of the split view stay aligned.
///
/// # Arguments
///
/// - `&str` - The original text.
/// - `&str` - The modified text.
///
/// # Returns
///
/// - `Vec<DiffRow>` - The rows of the result.
pub fn build_rows(original: &str, modified: &str) -> Vec<DiffRow> {
    let original_lines: Vec<&str> = split_lines(original);
    let modified_lines: Vec<&str> = split_lines(modified);
    let parts: Vec<DiffPart> = myers_diff(&original_lines, &modified_lines);
    rows_from_parts(&parts)
}

/// Counts the lines a comparison added, removed and kept.
///
/// # Arguments
///
/// - `&[DiffRow]` - The rows of the result.
///
/// # Returns
///
/// - `(usize, usize, usize)` - The number of added, removed and unchanged lines.
pub fn count_rows(rows: &[DiffRow]) -> (usize, usize, usize) {
    let mut added: usize = 0;
    let mut removed: usize = 0;
    let mut unchanged: usize = 0;
    for row in rows {
        match row.kind {
            DiffRowKind::Unchanged => unchanged += 1,
            DiffRowKind::Changed => {
                added += 1;
                removed += 1;
            }
            DiffRowKind::Added => added += 1,
            DiffRowKind::Removed => removed += 1,
        }
    }
    (added, removed, unchanged)
}

/// Turns the segments of a line comparison into the rows of the result.
///
/// # Arguments
///
/// - `&[DiffPart]` - The segments of the line comparison.
///
/// # Returns
///
/// - `Vec<DiffRow>` - The rows of the result.
fn rows_from_parts(parts: &[DiffPart]) -> Vec<DiffRow> {
    let mut rows: Vec<DiffRow> = Vec::new();
    let mut index: usize = 0;
    while index < parts.len() {
        if parts[index].kind == DiffKind::Unchanged {
            rows.push(DiffRow {
                kind: DiffRowKind::Unchanged,
                original: vec![parts[index].clone()],
                modified: vec![parts[index].clone()],
            });
            index += 1;
            continue;
        }
        let mut removed: Vec<&DiffPart> = Vec::new();
        let mut added: Vec<&DiffPart> = Vec::new();
        while index < parts.len() && parts[index].kind != DiffKind::Unchanged {
            if parts[index].kind == DiffKind::Removed {
                removed.push(&parts[index]);
            } else {
                added.push(&parts[index]);
            }
            index += 1;
        }
        let pairs: usize = removed.len().min(added.len());
        for pair in 0..pairs {
            rows.push(changed_row(&removed[pair].value, &added[pair].value));
        }
        for part in removed.iter().skip(pairs) {
            rows.push(DiffRow {
                kind: DiffRowKind::Removed,
                original: vec![(*part).clone()],
                modified: Vec::new(),
            });
        }
        for part in added.iter().skip(pairs) {
            rows.push(DiffRow {
                kind: DiffRowKind::Added,
                original: Vec::new(),
                modified: vec![(*part).clone()],
            });
        }
    }
    rows
}

/// Builds the row of a line that was replaced by another one.
///
/// The words of the two lines are compared, so the row marks the words that
/// differ on both of its sides.
///
/// # Arguments
///
/// - `&str` - The original line.
/// - `&str` - The modified line.
///
/// # Returns
///
/// - `DiffRow` - The row of the replacement.
fn changed_row(original: &str, modified: &str) -> DiffRow {
    if original == modified {
        return DiffRow {
            kind: DiffRowKind::Unchanged,
            original: vec![DiffPart {
                kind: DiffKind::Unchanged,
                value: original.to_string(),
            }],
            modified: vec![DiffPart {
                kind: DiffKind::Unchanged,
                value: modified.to_string(),
            }],
        };
    }
    let words: Vec<DiffPart> = word_diff(original, modified);
    let left: Vec<DiffPart> = words
        .iter()
        .filter(|part: &&DiffPart| part.kind != DiffKind::Added)
        .cloned()
        .collect();
    let right: Vec<DiffPart> = words
        .iter()
        .filter(|part: &&DiffPart| part.kind != DiffKind::Removed)
        .cloned()
        .collect();
    DiffRow {
        kind: DiffRowKind::Changed,
        original: left,
        modified: right,
    }
}

/// Walks the table of the comparison backwards and collects the segments.
///
/// # Arguments
///
/// - `&[&str]` - The original sequence.
/// - `&[&str]` - The modified sequence.
/// - `&[Vec<isize>]` - The snapshots of the table.
/// - `usize` - The slot the centre of the table sits at.
///
/// # Returns
///
/// - `Vec<DiffPart>` - The segments of the difference.
fn backtrack(
    original: &[&str],
    modified: &[&str],
    trace: &[Vec<isize>],
    center: usize,
) -> Vec<DiffPart> {
    let mut changes: Vec<DiffPart> = Vec::new();
    let mut x: isize = original.len() as isize;
    let mut y: isize = modified.len() as isize;
    for depth in (0..trace.len()).rev() {
        let current: &Vec<isize> = &trace[depth];
        let diagonal: isize = x - y;
        let slot: usize = slot_of(center, diagonal);
        let previous: isize = if diagonal == -(depth as isize)
            || (diagonal != depth as isize && current[slot - 1] < current[slot + 1])
        {
            diagonal + 1
        } else {
            diagonal - 1
        };
        let previous_x: isize = current[slot_of(center, previous)];
        let previous_y: isize = previous_x - previous;
        while x > previous_x && y > previous_y {
            x -= 1;
            y -= 1;
            changes.push(DiffPart {
                kind: DiffKind::Unchanged,
                value: original[x as usize].to_string(),
            });
        }
        if depth > 0 {
            if x > previous_x {
                x -= 1;
                changes.push(DiffPart {
                    kind: DiffKind::Removed,
                    value: original[x as usize].to_string(),
                });
            } else if y > previous_y {
                y -= 1;
                changes.push(DiffPart {
                    kind: DiffKind::Added,
                    value: modified[y as usize].to_string(),
                });
            }
        }
    }
    changes.reverse();
    changes
}

/// Reports the whole original text as removed and the whole modified text as
/// added.
///
/// The comparison gives up when the two texts have nothing in common, so the
/// result falls back to the simplest difference between them.
///
/// # Arguments
///
/// - `&[&str]` - The original sequence.
/// - `&[&str]` - The modified sequence.
///
/// # Returns
///
/// - `Vec<DiffPart>` - The segments of the difference.
fn fallback_diff(original: &[&str], modified: &[&str]) -> Vec<DiffPart> {
    let mut changes: Vec<DiffPart> = original
        .iter()
        .map(|line: &&str| DiffPart {
            kind: DiffKind::Removed,
            value: (*line).to_string(),
        })
        .collect();
    changes.extend(modified.iter().map(|line: &&str| DiffPart {
        kind: DiffKind::Added,
        value: (*line).to_string(),
    }));
    changes
}

/// Returns the slot of the table that holds the value of a diagonal.
///
/// # Arguments
///
/// - `usize` - The slot the centre of the table sits at.
/// - `isize` - The diagonal to look up.
///
/// # Returns
///
/// - `usize` - The slot of the diagonal.
fn slot_of(center: usize, diagonal: isize) -> usize {
    (center as isize + diagonal) as usize
}

/// Reports whether a byte ends a word.
///
/// # Arguments
///
/// - `u8` - The byte to look at.
///
/// # Returns
///
/// - `bool` - Whether the byte ends a word.
fn is_boundary(byte: u8) -> bool {
    WORD_BOUNDARIES.contains(&(byte as char))
}
