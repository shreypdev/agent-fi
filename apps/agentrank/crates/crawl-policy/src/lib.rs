//! Crawl policy for AgentBot: `robots.txt` parse/cache, outbound URL rules (SSRF mitigation).

mod cache;
mod fetch;
mod parse;
mod url_policy;

pub use cache::{CachedRobots, RobotsCache};
pub use fetch::{
    agent_robots_cache_key, agent_robots_url_for_card_url, merged_robots_allow,
    origin_key_from_url, refresh_agent_robots_for_url, refresh_robots_for_url,
    robots_url_for_card_url, RobotsFetchError,
};
pub use parse::{parse_robots_txt, ParsedRobots};
pub use url_policy::{validate_outbound_url, validate_outbound_url_str, UrlPolicyError};
