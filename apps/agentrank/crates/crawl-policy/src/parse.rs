//! Minimal RFC 9305–style `robots.txt` parsing for crawl allow/deny decisions.
//!
//! **Malformed content:** If the file cannot be parsed into coherent rules, we treat it as
//! **no rules** (allow all paths). This avoids blocking whole sites on typos in `robots.txt`.

/// Parsed rules: disallow path prefixes per user-agent group, optional crawl-delay (seconds).
#[derive(Debug, Clone, Default)]
pub struct ParsedRobots {
    /// Groups in file order (later groups can override semantics in some crawlers; we union matching groups).
    groups: Vec<Group>,
}

#[derive(Debug, Clone, Default)]
struct Group {
    agent_patterns: Vec<String>,
    disallow_prefixes: Vec<String>,
    crawl_delay_secs: Option<f64>,
}

impl ParsedRobots {
    /// `user_agent` is the full product string (e.g. `AgentBot/1.0`).
    /// `path` must start with `/` (URL path).
    pub fn is_allowed(&self, user_agent: &str, path: &str) -> bool {
        let path = if path.is_empty() {
            "/"
        } else if !path.starts_with('/') {
            return true;
        } else {
            path
        };

        let mut best_block: Option<&str> = None;
        let mut best_len = 0usize;

        for g in &self.groups {
            if !g.agent_patterns.iter().any(|p| ua_matches(p, user_agent)) {
                continue;
            }
            for prefix in &g.disallow_prefixes {
                if prefix.is_empty() {
                    continue;
                }
                let p = if prefix == "/" {
                    "/"
                } else if !prefix.starts_with('/') {
                    continue;
                } else {
                    prefix.as_str()
                };
                if path == p || path.starts_with(p) {
                    let plen = p.len();
                    if plen >= best_len {
                        best_len = plen;
                        best_block = Some(p);
                    }
                }
            }
        }

        best_block.is_none()
    }

    /// Crawl-delay from the first matching group that defines one (seconds).
    pub fn crawl_delay_secs(&self, user_agent: &str) -> Option<f64> {
        for g in &self.groups {
            if g.agent_patterns.iter().any(|p| ua_matches(p, user_agent)) {
                if let Some(d) = g.crawl_delay_secs.filter(|x| *x > 0.0) {
                    return Some(d);
                }
            }
        }
        None
    }
}

fn ua_matches(pattern: &str, client: &str) -> bool {
    let p = pattern.trim();
    if p == "*" {
        return true;
    }
    if p.is_empty() {
        return false;
    }
    let pl = p.to_ascii_lowercase();
    let cl = client.to_ascii_lowercase();
    cl.contains(&pl)
}

/// Parse `robots.txt` body. Lines without `key: value` are skipped. **Severely broken** input
/// still yields best-effort groups; empty file ⇒ allow all.
pub fn parse_robots_txt(input: &str) -> ParsedRobots {
    parse_robots_txt_inner(input)
}

fn parse_robots_txt_inner(input: &str) -> ParsedRobots {
    let mut groups: Vec<Group> = Vec::new();
    let mut current: Option<Group> = None;

    for line in input.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();

        match key.as_str() {
            "user-agent" => {
                if current
                    .as_ref()
                    .is_some_and(|g: &Group| !g.agent_patterns.is_empty())
                {
                    groups.push(current.take().unwrap());
                }
                if current.is_none() {
                    current = Some(Group::default());
                }
                current
                    .as_mut()
                    .unwrap()
                    .agent_patterns
                    .push(value.to_string());
            }
            "disallow" => {
                let g = current.get_or_insert_with(Group::default);
                if g.agent_patterns.is_empty() {
                    g.agent_patterns.push("*".into());
                }
                g.disallow_prefixes.push(value.to_string());
            }
            "allow" => {
                // MVP: ignore Allow (fewer false blocks; can extend later)
            }
            "crawl-delay" => {
                let g = current.get_or_insert_with(Group::default);
                if g.agent_patterns.is_empty() {
                    g.agent_patterns.push("*".into());
                }
                if let Ok(v) = value.parse::<f64>() {
                    g.crawl_delay_secs = Some(v);
                }
            }
            _ => {}
        }
    }

    if let Some(g) = current {
        if !g.agent_patterns.is_empty() || !g.disallow_prefixes.is_empty() {
            groups.push(g);
        }
    }

    ParsedRobots { groups }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_allows_all() {
        let p = parse_robots_txt("");
        assert!(p.is_allowed("AgentBot/1", "/.well-known/agent.json"));
    }

    #[test]
    fn disallow_root_blocks() {
        let txt = "User-agent: *\nDisallow: /\n";
        let p = parse_robots_txt(txt);
        assert!(!p.is_allowed("AgentBot/1", "/.well-known/agent.json"));
        assert!(!p.is_allowed("AgentBot/1", "/"));
    }

    #[test]
    fn agentbot_specific_group() {
        let txt = "User-agent: OtherBot\nDisallow: /\n\nUser-agent: AgentBot\nDisallow:\n";
        let p = parse_robots_txt(txt);
        assert!(p.is_allowed("AgentBot/1.0", "/x"));
    }

    #[test]
    fn star_agent_blocks() {
        let txt = "User-agent: *\nDisallow: /private\n";
        let p = parse_robots_txt(txt);
        assert!(!p.is_allowed("X", "/private/foo"));
        assert!(p.is_allowed("X", "/public"));
    }

    #[test]
    fn crawl_delay_parsed() {
        let txt = "User-agent: *\nCrawl-delay: 2.5\nDisallow:\n";
        let p = parse_robots_txt(txt);
        assert_eq!(p.crawl_delay_secs("AgentBot/1"), Some(2.5));
    }

    #[test]
    fn junk_lines_skipped_allow_all() {
        let p = parse_robots_txt("this is not robots at all {{{\n\n");
        assert!(p.is_allowed("Bot", "/anything"));
    }
}
