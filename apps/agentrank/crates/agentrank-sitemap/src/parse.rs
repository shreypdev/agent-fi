//! XML parsing for agent sitemap and standard sitemap index.

use chrono::{DateTime, Utc};
use flate2::read::GzDecoder;
use metrics::counter;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::{Cursor, Read};
use thiserror::Error;

/// One discovered card URL from a sitemap `urlset`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SitemapEntry {
    pub card_url: String,
    pub lastmod: Option<DateTime<Utc>>,
    /// Optional status hint from the document (e.g. `active`).
    pub status: Option<String>,
    /// Optional skills summary if present.
    pub skills_summary: Option<String>,
}

/// Result of parsing a top-level XML document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SitemapParseResult {
    /// Child sitemap URLs to fetch (sitemap index).
    Index(Vec<String>),
    /// Card rows from a leaf sitemap.
    Urlset(Vec<SitemapEntry>),
}

#[derive(Debug, Error)]
pub enum SitemapParseError {
    #[error("invalid gzip: {0}")]
    Gzip(String),
    #[error("xml: {0}")]
    Xml(String),
    #[error("empty document")]
    Empty,
    #[error("unknown root element")]
    UnknownRoot,
}

#[derive(Default)]
struct UrlBuilder {
    card_url: Option<String>,
    lastmod: Option<String>,
    status: Option<String>,
    skills: Option<String>,
    text_target: Option<&'static str>,
    text_buf: String,
}

impl UrlBuilder {
    fn reset(&mut self) {
        self.card_url = None;
        self.lastmod = None;
        self.status = None;
        self.skills = None;
        self.text_target = None;
        self.text_buf.clear();
    }

    fn flush_text(&mut self) {
        let t = self.text_buf.trim();
        if t.is_empty() {
            self.text_buf.clear();
            self.text_target = None;
            return;
        }
        match self.text_target {
            Some("card_url") | Some("loc") => {
                self.card_url = Some(t.to_string());
            }
            Some("lastmod") => {
                self.lastmod = Some(t.to_string());
            }
            Some("status") => {
                self.status = Some(t.to_string());
            }
            Some("skills") => {
                self.skills = Some(t.to_string());
            }
            _ => {}
        }
        self.text_buf.clear();
        self.text_target = None;
    }

    fn take_as_entry(&mut self) -> Option<SitemapEntry> {
        self.flush_text();
        let card_url = self.card_url.take()?;
        let lastmod = self
            .lastmod
            .take()
            .and_then(|s| s.trim().parse::<DateTime<Utc>>().ok());
        let status = self.status.take().map(|s| s.trim().to_string());
        let skills_summary = self.skills.take().map(|s| s.trim().to_string());
        Some(SitemapEntry {
            card_url,
            lastmod,
            status,
            skills_summary,
        })
    }
}

/// Decode gzip-compressed bytes if the header is gzip.
pub fn gunzip_if_needed(bytes: &[u8]) -> Result<Vec<u8>, SitemapParseError> {
    if bytes.len() >= 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
        let mut dec = GzDecoder::new(Cursor::new(bytes));
        let mut out = Vec::new();
        dec.read_to_end(&mut out)
            .map_err(|e| SitemapParseError::Gzip(e.to_string()))?;
        return Ok(out);
    }
    Ok(bytes.to_vec())
}

/// Parse `bytes` as XML (already uncompressed unless using [`gunzip_if_needed`] first).
pub fn parse_sitemap_document(bytes: &[u8]) -> Result<SitemapParseResult, SitemapParseError> {
    if bytes.is_empty() {
        return Err(SitemapParseError::Empty);
    }

    let mut reader = Reader::from_reader(bytes);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let root = loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Decl(_)) => {}
            Ok(Event::DocType(_)) => {}
            Ok(Event::Comment(_)) => {}
            Ok(Event::PI(_)) => {}
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                break e.local_name().as_ref().to_vec();
            }
            Ok(Event::Eof) => return Err(SitemapParseError::Empty),
            Err(e) => return Err(SitemapParseError::Xml(e.to_string())),
            _ => {}
        }
        buf.clear();
    };
    let root_str = String::from_utf8_lossy(&root);

    if root_str.ends_with("sitemapindex") || root_str == "sitemapindex" {
        return parse_index(&mut reader, &mut buf);
    }
    if root_str.ends_with("urlset") || root_str == "urlset" {
        return parse_urlset(&mut reader, &mut buf);
    }

    Err(SitemapParseError::UnknownRoot)
}

fn parse_index(
    reader: &mut Reader<&[u8]>,
    buf: &mut Vec<u8>,
) -> Result<SitemapParseResult, SitemapParseError> {
    let mut out = Vec::new();
    let mut in_loc = false;
    let mut loc_buf = String::new();

    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) => {
                if e.local_name().as_ref() == b"loc" {
                    in_loc = true;
                    loc_buf.clear();
                }
            }
            Ok(Event::Empty(e)) => {
                if e.local_name().as_ref() == b"loc" {
                    for a in e.attributes().flatten() {
                        if a.key.local_name().as_ref() == b"href" {
                            if let Ok(v) = a.unescape_value() {
                                let u = v.trim().to_string();
                                if !u.is_empty() {
                                    out.push(u);
                                }
                            }
                        }
                    }
                }
            }
            Ok(Event::Text(t)) => {
                if in_loc {
                    loc_buf.push_str(&t.unescape().unwrap_or_default());
                }
            }
            Ok(Event::CData(t)) => {
                if in_loc {
                    loc_buf.push_str(&String::from_utf8_lossy(t.as_ref()));
                }
            }
            Ok(Event::End(e)) => {
                if e.local_name().as_ref() == b"loc" {
                    in_loc = false;
                    let u = loc_buf.trim().to_string();
                    if !u.is_empty() {
                        out.push(u);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(SitemapParseError::Xml(e.to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(SitemapParseResult::Index(out))
}

fn field_tag(name: &[u8]) -> Option<&'static str> {
    match name {
        b"loc" => Some("loc"),
        b"card_url" => Some("card_url"),
        b"lastmod" => Some("lastmod"),
        b"status" => Some("status"),
        b"skills" => Some("skills"),
        _ => None,
    }
}

fn parse_urlset(
    reader: &mut Reader<&[u8]>,
    buf: &mut Vec<u8>,
) -> Result<SitemapParseResult, SitemapParseError> {
    let mut entries = Vec::new();
    let mut current = UrlBuilder::default();
    let mut depth_url = 0u32;

    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) => {
                let ln = e.local_name();
                let name = ln.as_ref();
                if name == b"url" {
                    depth_url += 1;
                    if depth_url == 1 {
                        current.reset();
                    }
                } else if depth_url > 0 {
                    if let Some(tag) = field_tag(name) {
                        current.text_target = Some(tag);
                        current.text_buf.clear();
                    }
                }
            }
            Ok(Event::Empty(e)) => {
                if depth_url > 0 {
                    let ln = e.local_name();
                    let name = ln.as_ref();
                    for a in e.attributes().flatten() {
                        if a.key.local_name().as_ref() != b"href" {
                            continue;
                        }
                        if let Ok(v) = a.unescape_value() {
                            let s = v.trim();
                            if s.is_empty() {
                                continue;
                            }
                            match name {
                                b"loc" | b"card_url" => current.card_url = Some(s.to_string()),
                                b"lastmod" => current.lastmod = Some(s.to_string()),
                                b"status" => current.status = Some(s.to_string()),
                                b"skills" => current.skills = Some(s.to_string()),
                                _ => {}
                            }
                        }
                    }
                }
            }
            Ok(Event::Text(t)) => {
                if current.text_target.is_some() {
                    current.text_buf.push_str(&t.unescape().unwrap_or_default());
                }
            }
            Ok(Event::CData(t)) => {
                if current.text_target.is_some() {
                    current
                        .text_buf
                        .push_str(&String::from_utf8_lossy(t.as_ref()));
                }
            }
            Ok(Event::End(e)) => {
                let ln = e.local_name();
                let name = ln.as_ref();
                if depth_url > 0 && field_tag(name).is_some() {
                    current.flush_text();
                }
                if name == b"url" && depth_url > 0 {
                    depth_url -= 1;
                    if depth_url == 0 {
                        if let Some(ent) = current.take_as_entry() {
                            entries.push(ent);
                        }
                        current.reset();
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(SitemapParseError::Xml(e.to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(SitemapParseResult::Urlset(entries))
}

/// Record metrics for a parse attempt (`status` = ok|gzip_error|parse_error|empty).
pub fn record_sitemap_metrics(status: &str) {
    let status = status.to_owned();
    counter!("agentrank_sitemap_fetch_total", "status" => status).increment(1);
}

/// Count discovered card rows (for leaf urlsets).
pub fn record_cards_discovered(n: u64, phase: &str) {
    let phase = phase.to_owned();
    counter!("agentrank_sitemap_cards_discovered_total", "phase" => phase).increment(n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_urlset_loc() {
        let xml = br#"<?xml version="1.0"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url><loc>https://a.example/card.json</loc></url>
</urlset>"#;
        let r = parse_sitemap_document(xml).unwrap();
        match r {
            SitemapParseResult::Urlset(v) => {
                assert_eq!(v.len(), 1);
                assert_eq!(v[0].card_url, "https://a.example/card.json");
            }
            _ => panic!("expected urlset"),
        }
    }

    #[test]
    fn agent_card_url_and_lastmod() {
        let xml = br#"<?xml version="1.0"?>
<urlset xmlns="https://example.com/ns">
  <url>
    <card_url>https://p.example/.well-known/agent.json</card_url>
    <lastmod>2025-01-01T00:00:00Z</lastmod>
    <status>active</status>
  </url>
</urlset>"#;
        let r = parse_sitemap_document(xml).unwrap();
        let SitemapParseResult::Urlset(v) = r else {
            panic!();
        };
        assert_eq!(v[0].card_url, "https://p.example/.well-known/agent.json");
        assert!(v[0].lastmod.is_some());
        assert_eq!(v[0].status.as_deref(), Some("active"));
    }

    #[test]
    fn index_two_children() {
        let xml = br#"<?xml version="1.0"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <sitemap><loc>https://x.example/s1.xml</loc></sitemap>
  <sitemap><loc>https://x.example/s2.xml</loc></sitemap>
</sitemapindex>"#;
        let r = parse_sitemap_document(xml).unwrap();
        let SitemapParseResult::Index(urls) = r else {
            panic!();
        };
        assert_eq!(urls.len(), 2);
    }

    #[test]
    fn gzip_roundtrip() {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;
        let xml = br#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"><url><loc>https://z.example/c</loc></url></urlset>"#;
        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
        enc.write_all(xml).unwrap();
        let gz = enc.finish().unwrap();
        let plain = gunzip_if_needed(&gz).unwrap();
        let r = parse_sitemap_document(&plain).unwrap();
        let SitemapParseResult::Urlset(v) = r else {
            panic!();
        };
        assert_eq!(v[0].card_url, "https://z.example/c");
    }

    #[test]
    fn malformed_xml_errors() {
        let err = parse_sitemap_document(b"<urlset><notclosed").unwrap_err();
        assert!(matches!(err, SitemapParseError::Xml(_)));
    }

    #[test]
    fn duplicate_urls_preserved() {
        let xml = br#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
        <url><loc>https://dup.example/a</loc></url>
        <url><loc>https://dup.example/a</loc></url>
        </urlset>"#;
        let SitemapParseResult::Urlset(v) = parse_sitemap_document(xml).unwrap() else {
            panic!();
        };
        assert_eq!(v.len(), 2);
    }
}
