//! Tantivy schema and field ids for agents.

use tantivy::schema::*;

/// On-disk marker next to the index (`AGENTRANK_INDEX_VERSION`).
pub const INDEX_VERSION: &str = "1";
pub const VERSION_FILENAME: &str = "AGENTRANK_INDEX_VERSION";

/// BM25 field boosts (opus §11.2.1 direction; Week 3 frozen constants).
pub const BOOST_NAME: f32 = 3.0;
pub const BOOST_SKILLS: f32 = 2.0;
pub const BOOST_DESCRIPTION: f32 = 1.0;

/// Built-in schema: `agent_id` (keyword), `name`, `description`, `skills_blob` (all TEXT + STORED).
#[derive(Clone)]
pub struct AgentSchema {
    pub schema: Schema,
    pub agent_id: Field,
    pub name: Field,
    pub description: Field,
    pub skills_blob: Field,
}

impl AgentSchema {
    pub fn new() -> Self {
        let mut builder = Schema::builder();
        let agent_id = builder.add_text_field(
            "agent_id",
            TextOptions::default()
                .set_indexing_options(
                    TextFieldIndexing::default()
                        .set_tokenizer("raw")
                        .set_index_option(IndexRecordOption::Basic),
                )
                .set_stored(),
        );
        let text_options = TextOptions::default()
            .set_indexing_options(
                TextFieldIndexing::default()
                    .set_tokenizer("default")
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions),
            )
            .set_stored();
        let name = builder.add_text_field("name", text_options.clone());
        let description = builder.add_text_field("description", text_options.clone());
        let skills_blob = builder.add_text_field("skills_blob", text_options);
        let schema = builder.build();
        Self {
            schema,
            agent_id,
            name,
            description,
            skills_blob,
        }
    }
}

impl Default for AgentSchema {
    fn default() -> Self {
        Self::new()
    }
}
