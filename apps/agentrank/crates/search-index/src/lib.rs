//! Tantivy index over AgentRank `agents` (name, description, skills text from `card_json`).
//!
//! Lifecycle: full [`store::rebuild_index`], single [`store::upsert_agent`], read-only
//! [`store::index_reader`] + [`search::search_agents`].

mod document;
mod error;
mod extract;
mod row;
pub mod schema;
pub mod search;
pub mod store;

pub use error::IndexError;
pub use extract::{skills_blob_from_card_json, MAX_INDEXED_CHARS};
pub use row::AgentIndexRow;
pub use schema::AgentSchema;
pub use search::{search_agents, SearchHit};
