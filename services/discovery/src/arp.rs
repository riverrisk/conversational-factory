use std::collections::HashMap;
use std::net::Ipv4Addr;

pub async fn scrape() -> HashMap<Ipv4Addr, String> {
    tokio::task::spawn_blocking(scrape_blocking)
        .await
        .unwrap_or_default()
}

#[cfg(target_os = "linux")]
fn scrape_blocking() -> HashMap<Ipv4Addr, String> {
    let mut out = HashMap::new();
    let Ok(contents) = std::fs::read_to_string("/proc/net/arp") else {
        return out;
    };
    for line in contents.lines().skip(1) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 4 {
            continue;
        }
        let Ok(ip) = fields[0].parse::<Ipv4Addr>() else {
            continue;
        };
        if let Some(mac) = normalize_mac(fields[3]) {
            if mac != "00:00:00:00:00:00" {
                out.insert(ip, mac);
            }
        }
    }
    out
}

#[cfg(not(target_os = "linux"))]
fn scrape_blocking() -> HashMap<Ipv4Addr, String> {
    let mut out = HashMap::new();
    let Ok(output) = std::process::Command::new("arp").arg("-an").output() else {
        return out;
    };
    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        // Expected: ? (10.0.1.1) at aa:bb:cc:dd:ee:ff on en0 ifscope [ethernet]
        let Some(open) = line.find('(') else { continue };
        let Some(close) = line.find(')') else { continue };
        let Some(at) = line.find(" at ") else { continue };
        if close <= open + 1 || at <= close {
            continue;
        }
        let ip_str = &line[open + 1..close];
        let after_at = &line[at + 4..];
        let mac_str = after_at.split_whitespace().next().unwrap_or("");
        let Ok(ip) = ip_str.parse::<Ipv4Addr>() else {
            continue;
        };
        if let Some(mac) = normalize_mac(mac_str) {
            out.insert(ip, mac);
        }
    }
    out
}

fn normalize_mac(raw: &str) -> Option<String> {
    let parts: Vec<&str> = raw.split(':').collect();
    if parts.len() != 6 {
        return None;
    }
    let mut octets = Vec::with_capacity(6);
    for part in parts {
        if part.is_empty() || part.len() > 2 {
            return None;
        }
        if !part.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        octets.push(format!("{:0>2}", part.to_uppercase()));
    }
    Some(octets.join(":"))
}

#[cfg(test)]
mod tests {
    use super::normalize_mac;

    #[test]
    fn pads_short_octets_and_uppercases() {
        assert_eq!(
            normalize_mac("dc:a6:32:0:1:2").as_deref(),
            Some("DC:A6:32:00:01:02")
        );
    }

    #[test]
    fn rejects_incomplete_marker() {
        assert!(normalize_mac("(incomplete)").is_none());
    }

    #[test]
    fn rejects_wrong_octet_count() {
        assert!(normalize_mac("aa:bb:cc:dd:ee").is_none());
        assert!(normalize_mac("aa:bb:cc:dd:ee:ff:00").is_none());
    }
}
