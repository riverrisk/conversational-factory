use std::collections::{BTreeMap, BTreeSet};

use cf_shared::assets::SemanticRecord;
use cf_shared::query::RecordFilter;

pub fn summarize_record(r: &SemanticRecord) -> String {
    let mut parts = vec![format!("**{}**", r.fqdn)];

    if let (Some(vendor), Some(model)) = (&r.vendor, &r.model) {
        parts.push(format!("{} {}", capitalize(vendor), model));
    } else if let Some(vendor) = &r.vendor {
        parts.push(capitalize(vendor));
    }

    if let Some(class) = &r.class {
        parts.push(format!("({})", class.to_uppercase()));
    }

    let mut location = Vec::new();
    if let Some(s) = &r.site {
        location.push(s.clone());
    }
    if let Some(a) = &r.area {
        location.push(a.clone());
    }
    if let Some(wc) = &r.work_center {
        location.push(wc.clone());
    }
    if let Some(wu) = &r.work_unit {
        location.push(wu.clone());
    }
    if !location.is_empty() {
        parts.push(format!("at {}", location.join(" > ")));
    }

    if !r.protocols.is_empty() {
        parts.push(format!("protocols: {}", r.protocols.join(", ")));
    }

    if let Some(ip) = &r.external_ip {
        parts.push(format!("IP: {}", ip));
    }

    parts.join(" | ")
}

pub fn summarize_query_results(records: &[SemanticRecord], filter: &RecordFilter) -> String {
    if records.is_empty() {
        return format!("No assets found matching the given filter.");
    }

    let filter_desc = describe_filter(filter);
    let mut lines = vec![format!(
        "Found **{}** asset(s){}.",
        records.len(),
        if filter_desc.is_empty() {
            String::new()
        } else {
            format!(" matching {}", filter_desc)
        }
    )];

    for r in records.iter().take(10) {
        lines.push(format!("- {}", summarize_record(r)));
    }
    if records.len() > 10 {
        lines.push(format!("...and {} more.", records.len() - 10));
    }

    lines.join("\n")
}

pub fn describe_topology(records: &[SemanticRecord]) -> String {
    let mut sites: BTreeMap<String, BTreeMap<String, BTreeSet<String>>> = BTreeMap::new();

    for r in records {
        let site = r.site.as_deref().unwrap_or("(unknown site)");
        let area = r.area.as_deref().unwrap_or("(unknown area)");
        let wc = r.work_center.as_deref().unwrap_or("(unassigned)");
        sites
            .entry(site.to_string())
            .or_default()
            .entry(area.to_string())
            .or_default()
            .insert(wc.to_string());
    }

    let mut lines = vec![format!("**Topology** ({} assets across {} site(s))", records.len(), sites.len())];

    for (site, areas) in &sites {
        lines.push(format!("\n**{}**", site));
        for (area, work_centers) in areas {
            let device_count = records
                .iter()
                .filter(|r| {
                    r.site.as_deref() == Some(site.as_str())
                        && r.area.as_deref() == Some(area.as_str())
                })
                .count();
            lines.push(format!("  {} ({} devices)", area, device_count));
            for wc in work_centers {
                if wc != "(unassigned)" {
                    let wc_count = records
                        .iter()
                        .filter(|r| {
                            r.site.as_deref() == Some(site.as_str())
                                && r.area.as_deref() == Some(area.as_str())
                                && r.work_center.as_deref() == Some(wc.as_str())
                        })
                        .count();
                    lines.push(format!("    {} ({} devices)", wc, wc_count));
                }
            }
        }
    }

    lines.join("\n")
}

pub fn summarize_fleet(records: &[SemanticRecord], filter: &RecordFilter) -> String {
    if records.is_empty() {
        return "No assets found.".into();
    }

    let filter_desc = describe_filter(filter);
    let mut lines = vec![format!(
        "**Fleet summary**: {} asset(s){}",
        records.len(),
        if filter_desc.is_empty() {
            String::new()
        } else {
            format!(" matching {}", filter_desc)
        }
    )];

    // Group by class
    let mut by_class: BTreeMap<String, usize> = BTreeMap::new();
    for r in records {
        let class = r.class.as_deref().unwrap_or("unknown").to_string();
        *by_class.entry(class).or_default() += 1;
    }
    lines.push("\nBy device class:".into());
    for (class, count) in &by_class {
        lines.push(format!("- {}: {}", class.to_uppercase(), count));
    }

    // Group by vendor
    let mut by_vendor: BTreeMap<String, usize> = BTreeMap::new();
    for r in records {
        let vendor = r.vendor.as_deref().unwrap_or("unknown").to_string();
        *by_vendor.entry(vendor).or_default() += 1;
    }
    lines.push("\nBy vendor:".into());
    for (vendor, count) in &by_vendor {
        lines.push(format!("- {}: {}", capitalize(vendor), count));
    }

    // Group by area
    let mut by_area: BTreeMap<String, usize> = BTreeMap::new();
    for r in records {
        let area = r.area.as_deref().unwrap_or("unassigned").to_string();
        *by_area.entry(area).or_default() += 1;
    }
    lines.push("\nBy area:".into());
    for (area, count) in &by_area {
        lines.push(format!("- {}: {}", area, count));
    }

    lines.join("\n")
}

fn describe_filter(f: &RecordFilter) -> String {
    let mut parts = vec![];
    if let Some(q) = &f.q {
        parts.push(format!("\"{}\"", q));
    }
    if let Some(v) = &f.vendor {
        parts.push(format!("vendor={}", v));
    }
    if let Some(c) = &f.class {
        parts.push(format!("class={}", c));
    }
    if let Some(s) = &f.site {
        parts.push(format!("site={}", s));
    }
    if let Some(a) = &f.area {
        parts.push(format!("area={}", a));
    }
    if let Some(wc) = &f.work_center {
        parts.push(format!("work_center={}", wc));
    }
    if let Some(wu) = &f.work_unit {
        parts.push(format!("work_unit={}", wu));
    }
    if let Some(z) = &f.zone {
        parts.push(format!("zone={}", z));
    }
    if let Some(c) = &f.cell {
        parts.push(format!("cell={}", c));
    }
    parts.join(", ")
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
