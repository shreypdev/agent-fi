//! Execute boosted lexical search over an [`Index`](tantivy::Index).

use crate::error::IndexError;
use crate::schema::{AgentSchema, BOOST_DESCRIPTION, BOOST_NAME, BOOST_SKILLS};
use tantivy::collector::TopDocs;
use tantivy::query::{BooleanQuery, BoostQuery, Occur, Query, QueryParser};
use tantivy::{Index, IndexReader, Score, TantivyDocument};
use uuid::Uuid;

/// One search hit (agent id + BM25 score).
#[derive(Debug, Clone, PartialEq)]
pub struct SearchHit {
    pub agent_id: Uuid,
    pub score: Score,
}

/// Parse `query` and search; returns hits sorted by score descending.
pub fn search_agents(
    reader: &IndexReader,
    index: &Index,
    ag: &AgentSchema,
    query: &str,
    limit: usize,
) -> Result<Vec<SearchHit>, IndexError> {
    let parser_name = QueryParser::for_index(index, vec![ag.name]);
    let parser_skills = QueryParser::for_index(index, vec![ag.skills_blob]);
    let parser_desc = QueryParser::for_index(index, vec![ag.description]);

    let q_name = parser_name
        .parse_query(query)
        .map_err(IndexError::QueryParse)?;
    let q_skills = parser_skills
        .parse_query(query)
        .map_err(IndexError::QueryParse)?;
    let q_desc = parser_desc
        .parse_query(query)
        .map_err(IndexError::QueryParse)?;

    let boolean = BooleanQuery::from(vec![
        (
            Occur::Should,
            Box::new(BoostQuery::new(q_name, BOOST_NAME)) as Box<dyn Query>,
        ),
        (
            Occur::Should,
            Box::new(BoostQuery::new(q_skills, BOOST_SKILLS)) as Box<dyn Query>,
        ),
        (
            Occur::Should,
            Box::new(BoostQuery::new(q_desc, BOOST_DESCRIPTION)) as Box<dyn Query>,
        ),
    ]);

    let searcher = reader.searcher();
    let top = searcher
        .search(&boolean, &TopDocs::with_limit(limit))
        .map_err(IndexError::Tantivy)?;

    let mut out = Vec::with_capacity(top.len());
    for (_score, doc_address) in top {
        let doc: TantivyDocument = searcher.doc(doc_address).map_err(IndexError::Tantivy)?;
        let stored = doc.get_first(ag.agent_id);
        let Some(tantivy::schema::OwnedValue::Str(s)) = stored else {
            continue;
        };
        let Ok(uid) = Uuid::parse_str(s.as_str()) else {
            continue;
        };
        out.push(SearchHit {
            agent_id: uid,
            score: _score,
        });
    }
    Ok(out)
}
