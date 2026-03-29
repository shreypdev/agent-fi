//! Build Tantivy documents from DB rows.

use crate::extract::{skills_blob_from_card_json, truncate_field};
use crate::row::AgentIndexRow;
use crate::schema::AgentSchema;
use tantivy::TantivyDocument;

impl AgentSchema {
    pub fn doc_from_row(&self, row: &AgentIndexRow) -> TantivyDocument {
        let skills = skills_blob_from_card_json(&row.card_json.0);
        let name = truncate_field(row.name.clone());
        let description = truncate_field(row.description.clone());
        let mut doc = TantivyDocument::default();
        doc.add_text(self.agent_id, row.id.to_string());
        doc.add_text(self.name, name);
        doc.add_text(self.description, description);
        doc.add_text(self.skills_blob, skills);
        doc
    }
}

pub type AgentDocument = TantivyDocument;
