//! Cross-source dedup at ingest: same canonical or endpoint URL merges into one agent row.

use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ExistingAgentRow {
    pub id: Uuid,
    pub external_id: String,
    pub trust_tier: String,
    pub card_json: Value,
}

fn trust_rank(tier: &str) -> i32 {
    match tier {
        "indexed" => 0,
        "established" => 1,
        "verified" => 2,
        "trusted" => 3,
        "authoritative" => 4,
        _ => 0,
    }
}

/// Union `skills` arrays (by JSON string) from `incoming` into `base` card JSON.
pub fn union_skills_in_card(mut base: Value, incoming: &Value) -> Value {
    let Some(inc_arr) = incoming.get("skills").and_then(|v| v.as_array()) else {
        return base;
    };
    if inc_arr.is_empty() {
        return base;
    }
    let base_skills = base.get_mut("skills").and_then(|v| v.as_array_mut());
    if base_skills.is_none() {
        base.as_object_mut()
            .map(|o| o.insert("skills".into(), json!([])));
    }
    let base_skills = base
        .get_mut("skills")
        .and_then(|v| v.as_array_mut())
        .expect("skills array");

    let mut seen = std::collections::HashSet::new();
    for x in base_skills.iter() {
        if let Ok(s) = serde_json::to_string(x) {
            seen.insert(s);
        }
    }
    for x in inc_arr {
        if let Ok(s) = serde_json::to_string(x) {
            if seen.insert(s) {
                base_skills.push(x.clone());
            }
        }
    }
    base
}

pub async fn find_existing_by_urls(
    pool: &PgPool,
    canonical_url: &str,
    endpoint_url: &str,
) -> Result<Option<ExistingAgentRow>, sqlx::Error> {
    let row: Option<(Uuid, String, String, Value)> = sqlx::query_as(
        r#"
        SELECT a.id, a.external_id, COALESCE(tr.trust_tier, 'indexed'), a.card_json
        FROM agents a
        LEFT JOIN trust_records tr ON tr.agent_id = a.id
        WHERE a.canonical_url = $1 OR a.endpoint_url = $2
        LIMIT 1
        "#,
    )
    .bind(canonical_url)
    .bind(endpoint_url)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(
        |(id, external_id, trust_tier, card_json)| ExistingAgentRow {
            id,
            external_id,
            trust_tier,
            card_json,
        },
    ))
}

/// Prefer higher-trust card; always union skills.
pub fn merge_cards(
    existing: &ExistingAgentRow,
    incoming_card: &Value,
    incoming_tier: &str,
) -> Value {
    let use_incoming = trust_rank(incoming_tier) > trust_rank(&existing.trust_tier);
    let base = if use_incoming {
        incoming_card.clone()
    } else {
        existing.card_json.clone()
    };
    let other = if use_incoming {
        &existing.card_json
    } else {
        incoming_card
    };
    union_skills_in_card(base, other)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn union_skills_dedupes() {
        let a = json!({"skills": [{"id":"1"}]});
        let b = json!({"skills": [{"id":"1"},{"id":"2"}]});
        let out = union_skills_in_card(a, &b);
        let arr = out["skills"].as_array().unwrap();
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn merge_cards_prefers_higher_trust_base() {
        let existing = ExistingAgentRow {
            id: Uuid::nil(),
            external_id: "a".into(),
            trust_tier: "verified".into(),
            card_json: json!({"name": "Old", "skills": []}),
        };
        let incoming = json!({"name": "New", "skills": [{"id": "s1"}]});
        let out = merge_cards(&existing, &incoming, "indexed");
        assert_eq!(out["name"], "Old");
        assert_eq!(out["skills"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn merge_cards_incoming_wins_when_higher_tier() {
        let existing = ExistingAgentRow {
            id: Uuid::nil(),
            external_id: "a".into(),
            trust_tier: "indexed".into(),
            card_json: json!({"name": "Old", "skills": [{"id": "x"}]}),
        };
        let incoming = json!({"name": "New", "skills": [{"id": "y"}]});
        let out = merge_cards(&existing, &incoming, "verified");
        assert_eq!(out["name"], "New");
        let skills = out["skills"].as_array().unwrap();
        assert!(!skills.is_empty());
    }
}
