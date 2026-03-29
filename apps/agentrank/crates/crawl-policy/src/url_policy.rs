//! Outbound URL validation (SSRF mitigation) for AgentBot fetches.

use std::net::IpAddr;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UrlPolicyError {
    #[error("only http and https URLs are allowed")]
    SchemeNotAllowed,
    #[error("URL missing host")]
    MissingHost,
    #[error("blocked host: {0}")]
    BlockedHost(&'static str),
    #[error("blocked IP: {0}")]
    BlockedIp(IpAddr),
    #[error("userinfo (embedded credentials) not allowed")]
    UserinfoNotAllowed,
    #[error("invalid URL: {0}")]
    InvalidUrl(String),
}

/// Validate a URL **before** issuing `GET`. Blocks private/reserved IPs when the host resolves
/// to a literal IP; blocks obvious metadata hostnames. **DNS rebinding** is not fully solved here
/// (see `docs/security-fetch-policy.md`).
/// `allow_http_localhost`: permit `http://127.0.0.1` / `localhost` (tests, wiremock).
/// `allow_loopback_https`: permit `https://127.0.0.1` etc. (integration tests with mock TLS).
pub fn validate_outbound_url(
    url: &Url,
    allow_http_localhost: bool,
    allow_loopback_https: bool,
) -> Result<(), UrlPolicyError> {
    if url.username() != "" || url.password().is_some() {
        return Err(UrlPolicyError::UserinfoNotAllowed);
    }

    let scheme = url.scheme();
    if scheme == "http" && allow_http_localhost {
        let host = url.host_str().ok_or(UrlPolicyError::MissingHost)?;
        if host == "localhost" || host == "127.0.0.1" || host == "[::1]" {
            return Ok(());
        }
        return Err(UrlPolicyError::SchemeNotAllowed);
    }

    if scheme != "https" {
        return Err(UrlPolicyError::SchemeNotAllowed);
    }

    if allow_loopback_https {
        if let Some(url::Host::Ipv4(ip4)) = url.host() {
            if ip4.is_loopback() {
                return Ok(());
            }
        }
        if let Some(url::Host::Ipv6(ip6)) = url.host() {
            if ip6.is_loopback() {
                return Ok(());
            }
        }
    }

    let host = url.host_str().ok_or(UrlPolicyError::MissingHost)?;

    let h_lower = host.to_ascii_lowercase();
    if h_lower == "metadata.google.internal"
        || h_lower.ends_with(".internal")
        || h_lower == "metadata"
    {
        return Err(UrlPolicyError::BlockedHost("metadata/internal"));
    }

    if let Some(ip) = url.host() {
        if let url::Host::Ipv4(ip4) = ip {
            if is_blocked_ipv4(ip4) {
                return Err(UrlPolicyError::BlockedIp(IpAddr::V4(ip4)));
            }
        }
        if let url::Host::Ipv6(ip6) = ip {
            if ip6.is_loopback()
                || ip6.is_unspecified()
                || is_unique_local_v6(ip6)
                || is_ula_or_link_local_v6(ip6)
            {
                return Err(UrlPolicyError::BlockedIp(IpAddr::V6(ip6)));
            }
        }
    }

    Ok(())
}

fn is_unique_local_v6(ip: std::net::Ipv6Addr) -> bool {
    (ip.segments()[0] & 0xfe00) == 0xfc00
}

fn is_ula_or_link_local_v6(ip: std::net::Ipv6Addr) -> bool {
    ip.is_unicast_link_local() || is_unique_local_v6(ip)
}

fn is_blocked_ipv4(ip: std::net::Ipv4Addr) -> bool {
    if ip.is_private()
        || ip.is_loopback()
        || ip.is_link_local()
        || ip.is_broadcast()
        || ip.is_documentation()
        || ip.is_unspecified()
    {
        return true;
    }
    // AWS metadata classic
    if octets(&ip) == [169, 254, 169, 254] {
        return true;
    }
    // Shared address space (RFC 6598)
    let o = octets(&ip);
    if o[0] == 100 && (o[1] & 0b1100_0000 == 0b0100_0000) {
        return true;
    }
    false
}

fn octets(ip: &std::net::Ipv4Addr) -> [u8; 4] {
    ip.octets()
}

/// Re-check after redirect: `final_url` must still pass policy.
pub fn validate_outbound_url_str(
    s: &str,
    allow_http_localhost: bool,
    allow_loopback_https: bool,
) -> Result<Url, UrlPolicyError> {
    let u = Url::parse(s).map_err(|e| UrlPolicyError::InvalidUrl(e.to_string()))?;
    validate_outbound_url(&u, allow_http_localhost, allow_loopback_https)?;
    Ok(u)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn https_public_ok() {
        let u = Url::parse("https://example.com/path").unwrap();
        assert!(validate_outbound_url(&u, false, false).is_ok());
    }

    #[test]
    fn http_blocked_by_default() {
        let u = Url::parse("http://example.com/").unwrap();
        assert_eq!(
            validate_outbound_url(&u, false, false),
            Err(UrlPolicyError::SchemeNotAllowed)
        );
    }

    #[test]
    fn localhost_http_allowed_in_test_mode() {
        let u = Url::parse("http://127.0.0.1:8080/x").unwrap();
        assert!(validate_outbound_url(&u, true, false).is_ok());
    }

    #[test]
    fn blocks_private_ipv4() {
        let u = Url::parse("https://10.0.0.1/").unwrap();
        assert!(matches!(
            validate_outbound_url(&u, false, false),
            Err(UrlPolicyError::BlockedIp(_))
        ));
    }

    #[test]
    fn blocks_metadata_ip() {
        let u = Url::parse("https://169.254.169.254/").unwrap();
        assert!(validate_outbound_url(&u, false, false).is_err());
    }

    #[test]
    fn blocks_userinfo() {
        let u = Url::parse("https://user:pass@example.com/").unwrap();
        assert_eq!(
            validate_outbound_url(&u, false, false),
            Err(UrlPolicyError::UserinfoNotAllowed)
        );
    }

    #[test]
    fn loopback_https_when_flagged() {
        let u = Url::parse("https://127.0.0.1:8443/x").unwrap();
        assert!(validate_outbound_url(&u, false, true).is_ok());
    }
}
