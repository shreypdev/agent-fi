//! Create / rebuild / upsert index on disk.

use crate::document::AgentDocument;
use crate::error::IndexError;
use crate::row::{AgentIndexRow, AGENTS_FOR_INDEX_SQL, AGENT_BY_ID_SQL};
use crate::schema::{AgentSchema, INDEX_VERSION, VERSION_FILENAME};
use futures::TryStreamExt;
use sqlx::PgPool;
use std::path::Path;
use tantivy::directory::MmapDirectory;
use tantivy::Index;
use uuid::Uuid;

const WRITER_HEAP_BYTES: usize = 50 * 1024 * 1024;

fn write_version_file(dir: &Path) -> Result<(), IndexError> {
    let p = dir.join(VERSION_FILENAME);
    std::fs::write(&p, INDEX_VERSION)?;
    Ok(())
}

fn read_version_file(dir: &Path) -> Result<String, IndexError> {
    let p = dir.join(VERSION_FILENAME);
    let s = std::fs::read_to_string(&p).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            IndexError::BadPath(format!("missing {}", p.display()))
        } else {
            IndexError::Io(e)
        }
    })?;
    Ok(s.trim().to_string())
}

/// Ensure index dir exists and version matches; open index.
pub fn open_index(path: &Path) -> Result<(Index, AgentSchema), IndexError> {
    if !path.is_dir() {
        return Err(IndexError::BadPath(format!(
            "not a directory: {}",
            path.display()
        )));
    }
    let found = read_version_file(path)?;
    if found != INDEX_VERSION {
        return Err(IndexError::SchemaVersion {
            found,
            expected: INDEX_VERSION.to_string(),
        });
    }
    let ag = AgentSchema::new();
    let dir = MmapDirectory::open(path).map_err(|e| IndexError::BadPath(e.to_string()))?;
    let index = Index::open(dir).map_err(IndexError::Tantivy)?;
    Ok((index, ag))
}

/// Remove existing index dir and rebuild from all agents.
pub async fn rebuild_index(pool: &PgPool, path: &Path) -> Result<(), IndexError> {
    if path.exists() {
        std::fs::remove_dir_all(path).map_err(IndexError::Io)?;
    }
    std::fs::create_dir_all(path).map_err(IndexError::Io)?;

    let ag = AgentSchema::new();
    let index = Index::create_in_dir(path, ag.schema.clone()).map_err(IndexError::Tantivy)?;

    let mut writer = index
        .writer(WRITER_HEAP_BYTES)
        .map_err(IndexError::Tantivy)?;

    let mut stream = sqlx::query_as::<_, AgentIndexRow>(AGENTS_FOR_INDEX_SQL).fetch(pool);
    while let Some(row) = stream.try_next().await.map_err(IndexError::Sqlx)? {
        let doc: AgentDocument = ag.doc_from_row(&row);
        writer.add_document(doc).map_err(IndexError::Tantivy)?;
    }

    writer.commit().map_err(IndexError::Tantivy)?;
    writer.wait_merging_threads().map_err(IndexError::Tantivy)?;
    write_version_file(path)?;
    Ok(())
}

/// Upsert one agent doc (delete by `agent_id` term if present, then add).
pub async fn upsert_agent(pool: &PgPool, path: &Path, agent_id: Uuid) -> Result<(), IndexError> {
    let row = sqlx::query_as::<_, AgentIndexRow>(AGENT_BY_ID_SQL)
        .bind(agent_id)
        .fetch_optional(pool)
        .await
        .map_err(IndexError::Sqlx)?
        .ok_or_else(|| IndexError::BadPath(format!("no agent row for id {agent_id}")))?;

    let (index, ag) = open_index(path)?;
    let mut writer = index
        .writer(WRITER_HEAP_BYTES)
        .map_err(IndexError::Tantivy)?;

    let term = tantivy::Term::from_field_text(ag.agent_id, &agent_id.to_string());
    writer.delete_term(term);
    let doc = ag.doc_from_row(&row);
    writer.add_document(doc).map_err(IndexError::Tantivy)?;
    writer.commit().map_err(IndexError::Tantivy)?;
    writer.wait_merging_threads().map_err(IndexError::Tantivy)?;
    Ok(())
}

/// Create empty index at `path` (no documents). Prefer [`rebuild_index`] for normal use.
pub fn create_empty_index(path: &Path) -> Result<(), IndexError> {
    if path.exists() {
        let mut read = std::fs::read_dir(path).map_err(IndexError::Io)?;
        if read.next().is_some() {
            return Err(IndexError::IndexExists(path.display().to_string()));
        }
    } else {
        std::fs::create_dir_all(path).map_err(IndexError::Io)?;
    }
    let ag = AgentSchema::new();
    let _index = Index::create_in_dir(path, ag.schema).map_err(IndexError::Tantivy)?;
    write_version_file(path)?;
    Ok(())
}

use tantivy::IndexReader;

/// Search the index at `path` (read-only).
pub fn index_reader(path: &Path) -> Result<(IndexReader, Index, AgentSchema), IndexError> {
    let (index, ag) = open_index(path)?;
    let reader = index.reader().map_err(IndexError::Tantivy)?;
    Ok((reader, index, ag))
}
