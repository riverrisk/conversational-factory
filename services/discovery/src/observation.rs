use cf_shared::assets::Observation;
use cf_shared::identity::{HardwareIdentity, HardwareIdentityKind, ObservationSource};
use chrono::Utc;
use uuid::Uuid;

use crate::probe::ProbeResult;

pub fn observation_from_probe(probe: &ProbeResult, mac: Option<&str>) -> Observation {
    let ip_str = probe.ip.to_string();

    // Prefer MAC for device_id stability across IP changes; fall back to IP.
    let device_id = match mac {
        Some(m) => Uuid::new_v5(&Uuid::NAMESPACE_OID, m.as_bytes()),
        None => Uuid::new_v5(&Uuid::NAMESPACE_DNS, ip_str.as_bytes()),
    };

    let protocols: Vec<String> = probe
        .open_ports
        .iter()
        .filter_map(|p| port_to_protocol(*p).map(str::to_string))
        .collect();

    let hardware_identities = mac.map(|m| {
        vec![HardwareIdentity {
            kind: HardwareIdentityKind::MacAddress,
            value: m.to_string(),
            label: Some("primary-nic".into()),
        }]
    });

    let aliases = probe.hostname.as_ref().map(|h| vec![h.clone()]);

    Observation {
        id: Uuid::new_v4(),
        device_id,
        observed_at: Utc::now(),
        source: ObservationSource::Discovery,
        node_kind: None,
        external_ip: None,
        internal_ip: Some(ip_str),
        class: None,
        vendor: None,
        model: None,
        protocols: (!protocols.is_empty()).then_some(protocols),
        mac: mac.map(str::to_string),
        switch_port: None,
        enterprise: None,
        site: None,
        area: None,
        work_center: None,
        work_center_kind: None,
        work_unit: None,
        facility: None,
        zone: None,
        cell: None,
        process: None,
        function: None,
        hardware_identities,
        application_identities: None,
        aliases,
        relations: None,
        status: None,
    }
}

fn port_to_protocol(port: u16) -> Option<&'static str> {
    match port {
        22 => Some("ssh"),
        80 => Some("http"),
        102 => Some("s7comm"),
        443 => Some("https"),
        502 => Some("modbus-tcp"),
        1883 => Some("mqtt"),
        4840 => Some("opcua"),
        8883 => Some("mqtt-tls"),
        20000 => Some("dnp3"),
        44818 => Some("ethernet-ip"),
        _ => None,
    }
}

pub fn default_ports() -> Vec<u16> {
    vec![22, 80, 102, 443, 502, 1883, 4840, 8883, 20000, 44818]
}
