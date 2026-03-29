//! Deterministic `card_json` → searchable skills text.
//!
//! Order per skill: `name`, `tags` (joined by space), `description`, `examples` (flattened).
//! Skills are separated by `\n\n`. Unknown skill shapes are skipped without panicking.

use serde_json::Value;

/// Max characters per indexed text field (stable truncation suffix).
pub const MAX_INDEXED_CHARS: usize = 50_000;

const TRUNC_SUFFIX: &str = "\n...[truncated]";

/// Build a single blob from `card_json["skills"]` for full-text indexing.
pub fn skills_blob_from_card_json(card_json: &Value) -> String {
    let Some(skills) = card_json.get("skills").and_then(|v| v.as_array()) else {
        return String::new();
    };
    let mut parts = Vec::new();
    for skill in skills {
        let Some(chunk) = skill_blob_one(skill) else {
            continue;
        };
        if !chunk.is_empty() {
            parts.push(chunk);
        }
    }
    truncate_field(parts.join("\n\n"))
}

fn skill_blob_one(skill: &Value) -> Option<String> {
    let obj = skill.as_object()?;
    let mut segments: Vec<String> = Vec::new();

    if let Some(s) = obj.get("name").and_then(Value::as_str) {
        let t = s.trim();
        if !t.is_empty() {
            segments.push(t.to_string());
        }
    }

    if let Some(tags) = obj.get("tags").and_then(|v| v.as_array()) {
        let tag_strs: Vec<&str> = tags
            .iter()
            .filter_map(|t| t.as_str().map(str::trim).filter(|s| !s.is_empty()))
            .collect();
        if !tag_strs.is_empty() {
            segments.push(tag_strs.join(" "));
        }
    }

    if let Some(s) = obj.get("description").and_then(Value::as_str) {
        let t = s.trim();
        if !t.is_empty() {
            segments.push(t.to_string());
        }
    }

    if let Some(ex) = obj.get("examples").and_then(|v| v.as_array()) {
        let flat = flatten_examples(ex);
        if !flat.is_empty() {
            segments.push(flat);
        }
    }

    if segments.is_empty() {
        None
    } else {
        Some(segments.join("\n"))
    }
}

fn flatten_examples(examples: &[Value]) -> String {
    let mut out = Vec::new();
    for ex in examples {
        match ex {
            Value::String(s) => {
                let t = s.trim();
                if !t.is_empty() {
                    out.push(t.to_string());
                }
            }
            Value::Object(map) => {
                for key in ["text", "input", "output", "description"] {
                    if let Some(Value::String(s)) = map.get(key) {
                        let t = s.trim();
                        if !t.is_empty() {
                            out.push(t.to_string());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out.join("\n")
}

/// Truncate to at most [`MAX_INDEXED_CHARS`] Unicode scalars with stable suffix.
pub fn truncate_field(s: String) -> String {
    let n = s.chars().count();
    if n <= MAX_INDEXED_CHARS {
        return s;
    }
    let take = MAX_INDEXED_CHARS.saturating_sub(TRUNC_SUFFIX.chars().count());
    let mut t = s.chars().take(take).collect::<String>();
    t.push_str(TRUNC_SUFFIX);
    t
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn empty_and_missing_skills() {
        assert_eq!(skills_blob_from_card_json(&json!({})), "");
        assert_eq!(skills_blob_from_card_json(&json!({"skills": []})), "");
        assert_eq!(skills_blob_from_card_json(&json!({"skills": null})), "");
    }

    #[test]
    fn one_skill_ordering() {
        let v =
            json!({"skills":[{"name":"N","tags":["a","b"],"description":"D","examples":["e1"]}]});
        assert_eq!(skills_blob_from_card_json(&v), "N\na b\nD\ne1");
    }

    #[test]
    fn unicode_and_long_truncation() {
        let long = "あ".repeat(MAX_INDEXED_CHARS + 100);
        let v = json!({"skills":[{"name": long}]});
        let out = skills_blob_from_card_json(&v);
        assert!(out.ends_with(TRUNC_SUFFIX));
        assert!(out.chars().count() <= MAX_INDEXED_CHARS);
    }

    #[test]
    fn skips_non_objects_in_array() {
        let v = json!({"skills":[1,"x",{"name":"ok"}]});
        assert_eq!(skills_blob_from_card_json(&v), "ok");
    }
}
