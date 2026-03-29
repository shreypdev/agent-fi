//! Parse `agent-sitemap.xml` and sitemap index documents for discovery.

mod parse;

pub use parse::{
    gunzip_if_needed, parse_sitemap_document, record_cards_discovered, record_sitemap_metrics,
    SitemapEntry, SitemapParseError, SitemapParseResult,
};
