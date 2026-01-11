use similar::{ChangeTag, TextDiff};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineKind {
    Unchanged,
    Added,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SideBySideLine {
    pub left: Option<String>,
    pub right: Option<String>,
    pub kind: DiffLineKind,
}

#[must_use]
pub fn build_unified_diff(
    original: &str,
    updated: &str,
    original_label: &str,
    updated_label: &str,
) -> String {
    let text_difference = TextDiff::from_lines(original, updated);
    text_difference
        .unified_diff()
        .header(original_label, updated_label)
        .to_string()
}

#[must_use]
pub fn build_side_by_side_lines(original: &str, updated: &str) -> Vec<SideBySideLine> {
    let text_difference = TextDiff::from_lines(original, updated);

    text_difference
        .iter_all_changes()
        .map(|change| {
            let change_tag = change.tag();
            let line = normalize_line(change.value());

            match change_tag {
                ChangeTag::Delete => SideBySideLine {
                    left: Some(line),
                    right: None,
                    kind: DiffLineKind::Removed,
                },
                ChangeTag::Insert => SideBySideLine {
                    left: None,
                    right: Some(line),
                    kind: DiffLineKind::Added,
                },
                ChangeTag::Equal => SideBySideLine {
                    left: Some(line.clone()),
                    right: Some(line),
                    kind: DiffLineKind::Unchanged,
                },
            }
        })
        .collect()
}

fn normalize_line(value: &str) -> String {
    value.trim_end_matches(['\n', '\r']).to_string()
}
