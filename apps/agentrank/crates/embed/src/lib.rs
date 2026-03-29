//! Text embeddings for hybrid search. Production: set `AGENTRANK_EMBEDDER=hash` for deterministic
//! 768-dim unit vectors (CI / dev); use `bge` when linked with `fastembed` (optional future).

/// Vector dimension (BGE-base-en-v1.5).
pub const EMBEDDING_DIM: usize = 768;

/// Embed card text for indexing / query. Uses `name`, `description`, and skills blob.
pub fn embed_document(name: &str, description: &str, skills_blob: &str) -> Vec<f32> {
    let text = format!(
        "{} {} {}",
        name.trim(),
        description.trim(),
        skills_blob.trim()
    );
    embed_text(&text)
}

/// Embed arbitrary query or document string.
pub fn embed_text(text: &str) -> Vec<f32> {
    // `AGENTRANK_EMBEDDER=hash|deterministic` — default for CI. Future: `bge` via ONNX/fastembed.
    let _ = std::env::var("AGENTRANK_EMBEDDER");
    hash_embed(text)
}

/// Deterministic 768-d Blake3-derived unit vector (L2-normalized). Preserves fusion geometry for tests.
pub fn hash_embed(text: &str) -> Vec<f32> {
    let h = blake3::hash(text.as_bytes());
    let bytes = h.as_bytes();
    let mut out = vec![0.0f32; EMBEDDING_DIM];
    for i in 0..EMBEDDING_DIM {
        let a = bytes[i % 32] as f32;
        let b = bytes[(i + 7) % 32] as f32;
        out[i] = (a / 255.0) * 2.0 - 1.0 + ((b / 255.0) * 2.0 - 1.0) * 0.01;
    }
    normalize_l2(&mut out);
    out
}

fn normalize_l2(v: &mut [f32]) {
    let s: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if s > 1e-12 {
        for x in v.iter_mut() {
            *x /= s;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dim_and_normalized() {
        let v = hash_embed("hello");
        assert_eq!(v.len(), EMBEDDING_DIM);
        let n: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((n - 1.0).abs() < 1e-4);
    }

    #[test]
    fn stable() {
        assert_eq!(hash_embed("a"), hash_embed("a"));
        assert_ne!(hash_embed("a"), hash_embed("b"));
    }
}
