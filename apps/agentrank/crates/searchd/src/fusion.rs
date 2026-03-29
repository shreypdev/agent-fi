//! Weighted RRF fusion (lexical + semantic ranks).

use std::collections::HashMap;
use uuid::Uuid;

/// Reciprocal rank fusion with per-list weights (todo: k=60, w_lex=0.4, w_sem=0.6).
pub fn rrf_fuse_weighted(
    lexical_ids_in_order: &[Uuid],
    semantic_ids_in_order: &[Uuid],
    k: f64,
    w_lex: f64,
    w_sem: f64,
) -> Vec<Uuid> {
    let mut scores: HashMap<Uuid, f64> = HashMap::new();
    for (i, id) in lexical_ids_in_order.iter().enumerate() {
        let rank = (i + 1) as f64;
        *scores.entry(*id).or_insert(0.0) += w_lex * (1.0 / (k + rank));
    }
    for (i, id) in semantic_ids_in_order.iter().enumerate() {
        let rank = (i + 1) as f64;
        *scores.entry(*id).or_insert(0.0) += w_sem * (1.0 / (k + rank));
    }
    let mut pairs: Vec<(Uuid, f64)> = scores.into_iter().collect();
    pairs.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.0.cmp(&b.0))
    });
    pairs.into_iter().map(|(id, _)| id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn tie_break_stable() {
        let a = Uuid::nil();
        let b = Uuid::from_u128(1);
        let out = rrf_fuse_weighted(&[a, b], &[b, a], 60.0, 0.4, 0.6);
        assert_eq!(out.len(), 2);
        assert!(out.contains(&a) && out.contains(&b));
    }

    #[test]
    fn empty_lists_yield_empty() {
        let out = rrf_fuse_weighted(&[], &[], 60.0, 0.4, 0.6);
        assert!(out.is_empty());
    }

    #[test]
    fn lexical_only_semantic_empty() {
        let a = Uuid::from_u128(10);
        let b = Uuid::from_u128(11);
        let out = rrf_fuse_weighted(&[a, b], &[], 60.0, 0.4, 0.6);
        assert_eq!(out, vec![a, b]);
    }

    #[test]
    fn semantic_only_lexical_empty() {
        let a = Uuid::from_u128(20);
        let b = Uuid::from_u128(21);
        let out = rrf_fuse_weighted(&[], &[a, b], 60.0, 0.4, 0.6);
        assert_eq!(out, vec![a, b]);
    }

    #[test]
    fn duplicate_id_in_one_list_sums_rrf_contributions() {
        let a = Uuid::from_u128(30);
        let b = Uuid::from_u128(31);
        // a appears twice in lexical — should rank higher than single-hit b if weights align
        let out = rrf_fuse_weighted(&[a, a, b], &[b, a], 60.0, 0.4, 0.6);
        assert_eq!(out[0], a);
    }

    #[test]
    fn disjoint_lists_order_by_weighted_score() {
        let a = Uuid::from_u128(40);
        let b = Uuid::from_u128(41);
        let c = Uuid::from_u128(42);
        // Lexical: a, b. Semantic: b, c — b in both lists should win.
        let out = rrf_fuse_weighted(&[a, b], &[b, c], 60.0, 0.4, 0.6);
        assert_eq!(out[0], b);
    }

    #[test]
    fn symmetric_weights_lexical_semantic_same_order_stable_uuid_tiebreak() {
        let low = Uuid::from_u128(1);
        let high = Uuid::from_u128(2);
        // Identical rank patterns under w_lex = w_sem; secondary sort is UUID ascending.
        let out = rrf_fuse_weighted(&[low, high], &[low, high], 60.0, 0.5, 0.5);
        assert_eq!(out, vec![low, high]);
    }
}
