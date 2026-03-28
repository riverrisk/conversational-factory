use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;

use chrono::Utc;
use uuid::Uuid;

use cf_shared::assets::SemanticRecord;
use cf_shared::identity::*;
use cf_shared::query::RecordFilter;
use cf_shared::system::SyncStatus;

use crate::routes::AssetProvider;

pub struct SampleProvider {
    records: Vec<SemanticRecord>,
}

impl SampleProvider {
    pub fn new() -> Self {
        Self { records: build_sample_fleet() }
    }
}

impl AssetProvider for SampleProvider {
    fn resolve(
        &self,
        target: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<SemanticRecord>, String>> + Send + '_>> {
        let target = target.to_lowercase();
        Box::pin(async move {
            let found = self.records.iter().find(|r| {
                r.fqdn.to_lowercase() == target
                    || r.device_id.to_string() == target
                    || r.external_ip.as_deref().is_some_and(|ip| ip == target)
                    || r.internal_ip.as_deref().is_some_and(|ip| ip == target)
                    || r.mac.as_deref().map(|m| m.to_lowercase()).is_some_and(|m| m == target)
                    || r.aliases.iter().any(|a| a.to_lowercase() == target)
                    || r.hardware_identities.iter().any(|h| h.value.to_lowercase() == target)
                    || r.application_identities.iter().any(|a| a.value.to_lowercase() == target)
            });
            Ok(found.cloned())
        })
    }

    fn query(
        &self,
        filter: &RecordFilter,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SemanticRecord>, String>> + Send + '_>> {
        let results: Vec<SemanticRecord> = self.records.iter().filter(|r| {
            matches_filter(r, filter)
        }).cloned().collect();
        Box::pin(async move { Ok(results) })
    }

    fn sync_status(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<SyncStatus, String>> + Send + '_>> {
        let count = self.records.len() as u64;
        Box::pin(async move {
            Ok(SyncStatus {
                total_leases: count,
                dns_records_synced: count,
                pending_updates: 0,
                failed_updates: 0,
                last_reconciliation: Some(Utc::now()),
            })
        })
    }
}

fn matches_filter(r: &SemanticRecord, f: &RecordFilter) -> bool {
    if let Some(q) = &f.q {
        let q = q.to_lowercase();
        let haystack = format!(
            "{} {} {} {} {} {}",
            r.fqdn,
            r.vendor.as_deref().unwrap_or(""),
            r.class.as_deref().unwrap_or(""),
            r.model.as_deref().unwrap_or(""),
            r.aliases.join(" "),
            r.site.as_deref().unwrap_or(""),
        )
        .to_lowercase();
        if !haystack.contains(&q) {
            return false;
        }
    }
    if let Some(nk) = &f.node_kind {
        if &r.node_kind != nk {
            return false;
        }
    }
    if let Some(c) = &f.class {
        if r.class.as_deref().map(|v| v.to_lowercase()) != Some(c.to_lowercase()) {
            return false;
        }
    }
    if let Some(v) = &f.vendor {
        if r.vendor.as_deref().map(|v2| v2.to_lowercase()) != Some(v.to_lowercase()) {
            return false;
        }
    }
    if let Some(s) = &f.site {
        if r.site.as_deref().map(|v| v.to_lowercase()) != Some(s.to_lowercase()) {
            return false;
        }
    }
    if let Some(a) = &f.area {
        if r.area.as_deref().map(|v| v.to_lowercase()) != Some(a.to_lowercase()) {
            return false;
        }
    }
    if let Some(wc) = &f.work_center {
        if r.work_center.as_deref().map(|v| v.to_lowercase()) != Some(wc.to_lowercase()) {
            return false;
        }
    }
    if let Some(wck) = &f.work_center_kind {
        if r.work_center_kind.as_ref() != Some(wck) {
            return false;
        }
    }
    if let Some(wu) = &f.work_unit {
        if r.work_unit.as_deref().map(|v| v.to_lowercase()) != Some(wu.to_lowercase()) {
            return false;
        }
    }
    if let Some(z) = &f.zone {
        if r.zone.as_deref().map(|v| v.to_lowercase()) != Some(z.to_lowercase())
            && r.area.as_deref().map(|v| v.to_lowercase()) != Some(z.to_lowercase())
        {
            return false;
        }
    }
    if let Some(c) = &f.cell {
        if r.cell.as_deref().map(|v| v.to_lowercase()) != Some(c.to_lowercase())
            && r.work_center.as_deref().map(|v| v.to_lowercase()) != Some(c.to_lowercase())
        {
            return false;
        }
    }
    if let Some(e) = &f.enterprise {
        if r.enterprise.as_deref().map(|v| v.to_lowercase()) != Some(e.to_lowercase()) {
            return false;
        }
    }
    if let Some(hw) = &f.hardware_identity {
        let hw_lower = hw.to_lowercase();
        if !r.hardware_identities.iter().any(|h| h.value.to_lowercase() == hw_lower) {
            return false;
        }
    }
    if let Some(app) = &f.application_id {
        let app_lower = app.to_lowercase();
        if !r.application_identities.iter().any(|a| a.value.to_lowercase() == app_lower) {
            return false;
        }
    }
    if let Some(alias) = &f.alias {
        let alias_lower = alias.to_lowercase();
        if !r.aliases.iter().any(|a| a.to_lowercase() == alias_lower) {
            return false;
        }
    }
    true
}

fn build_sample_fleet() -> Vec<SemanticRecord> {
    let now = Utc::now();

    let make_field_sources = |fields: &[(&str, &str, ObservationSource)]| -> BTreeMap<String, cf_shared::identity::MetadataField> {
        fields
            .iter()
            .map(|(k, v, src)| {
                (
                    k.to_string(),
                    cf_shared::identity::MetadataField {
                        value: v.to_string(),
                        source: src.clone(),
                        updated_at: now,
                    },
                )
            })
            .collect()
    };

    vec![
        // VFD on conveyor in Cell5, Zone3, Milwaukee
        SemanticRecord {
            device_id: Uuid::parse_str("22222222-2222-7222-8222-222222222222").unwrap(),
            fqdn: "DriveVFD.Conveyor.Cell5.Zone3.Milwaukee.local".into(),
            node_kind: Isa95NodeKind::Device,
            external_ip: Some("10.50.3.47".into()),
            internal_ip: Some("192.168.1.47".into()),
            class: Some("vfd".into()),
            vendor: Some("rockwell".into()),
            model: Some("PowerFlex525".into()),
            protocols: vec!["ethernet-ip".into(), "modbus-tcp".into()],
            mac: Some("00:00:BC:12:34:56".into()),
            switch_port: Some("Gi1/0/5".into()),
            enterprise: None,
            site: Some("Milwaukee".into()),
            facility: Some("Milwaukee".into()),
            area: Some("Zone3".into()),
            zone: Some("Zone3".into()),
            work_center: Some("Cell5".into()),
            cell: Some("Cell5".into()),
            work_center_kind: Some(Isa95WorkCenterKind::ProcessCell),
            work_unit: Some("Conveyor".into()),
            process: Some("Conveyor".into()),
            function: Some("DriveVFD".into()),
            hardware_identities: vec![HardwareIdentity {
                kind: HardwareIdentityKind::MacAddress,
                value: "00:00:BC:12:34:56".into(),
                label: Some("primary-nic".into()),
            }],
            application_identities: vec![ApplicationIdentity {
                kind: ApplicationIdentityKind::Urn,
                value: "urn:factory:asset:drivevfd-cell5-conveyor".into(),
                label: Some("cmms-asset".into()),
            }],
            aliases: vec!["ConveyorVFD5".into()],
            relations: vec![SemanticRelation {
                relation: "located-in".into(),
                target: "Conveyor.Cell5.Zone3.Milwaukee.local".into(),
                label: Some("work-unit".into()),
            }],
            status: RecordStatus::Active,
            updated_at: now,
            field_sources: make_field_sources(&[
                ("vendor", "rockwell", ObservationSource::ProtocolAnalysis),
                ("class", "vfd", ObservationSource::ProtocolAnalysis),
                ("site", "Milwaukee", ObservationSource::ManualApi),
                ("area", "Zone3", ObservationSource::ManualApi),
                ("work_center", "Cell5", ObservationSource::ManualApi),
                ("work_unit", "Conveyor", ObservationSource::ManualApi),
                ("function", "DriveVFD", ObservationSource::ManualApi),
            ]),
        },
        // Temperature sensor in Cell5
        SemanticRecord {
            device_id: Uuid::parse_str("33333333-3333-7333-8333-333333333333").unwrap(),
            fqdn: "TempSensor01.Conveyor.Cell5.Zone3.Milwaukee.local".into(),
            node_kind: Isa95NodeKind::Device,
            external_ip: Some("10.50.3.48".into()),
            internal_ip: Some("192.168.1.48".into()),
            class: Some("sensor".into()),
            vendor: Some("siemens".into()),
            model: Some("SITRANS-TS500".into()),
            protocols: vec!["profinet".into()],
            mac: Some("00:1A:2B:33:44:55".into()),
            switch_port: Some("Gi1/0/6".into()),
            enterprise: None,
            site: Some("Milwaukee".into()),
            facility: Some("Milwaukee".into()),
            area: Some("Zone3".into()),
            zone: Some("Zone3".into()),
            work_center: Some("Cell5".into()),
            cell: Some("Cell5".into()),
            work_center_kind: Some(Isa95WorkCenterKind::ProcessCell),
            work_unit: Some("Conveyor".into()),
            process: Some("Conveyor".into()),
            function: Some("TempSensor01".into()),
            hardware_identities: vec![HardwareIdentity {
                kind: HardwareIdentityKind::MacAddress,
                value: "00:1A:2B:33:44:55".into(),
                label: Some("primary-nic".into()),
            }],
            application_identities: vec![],
            aliases: vec!["ConvTempSensor".into()],
            relations: vec![SemanticRelation {
                relation: "located-in".into(),
                target: "Conveyor.Cell5.Zone3.Milwaukee.local".into(),
                label: Some("work-unit".into()),
            }],
            status: RecordStatus::Active,
            updated_at: now,
            field_sources: make_field_sources(&[
                ("vendor", "siemens", ObservationSource::DhcpFingerprint),
                ("class", "sensor", ObservationSource::ProtocolAnalysis),
                ("site", "Milwaukee", ObservationSource::ManualApi),
            ]),
        },
        // PLC in Cell2, Zone1
        SemanticRecord {
            device_id: Uuid::parse_str("44444444-4444-7444-8444-444444444444").unwrap(),
            fqdn: "MainPLC.Stamping.Cell2.Zone1.Milwaukee.local".into(),
            node_kind: Isa95NodeKind::Device,
            external_ip: Some("10.50.1.10".into()),
            internal_ip: Some("192.168.1.10".into()),
            class: Some("plc".into()),
            vendor: Some("rockwell".into()),
            model: Some("CompactLogix-5380".into()),
            protocols: vec!["ethernet-ip".into(), "cip".into()],
            mac: Some("00:00:BC:AA:BB:CC".into()),
            switch_port: Some("Gi2/0/1".into()),
            enterprise: None,
            site: Some("Milwaukee".into()),
            facility: Some("Milwaukee".into()),
            area: Some("Zone1".into()),
            zone: Some("Zone1".into()),
            work_center: Some("Cell2".into()),
            cell: Some("Cell2".into()),
            work_center_kind: Some(Isa95WorkCenterKind::WorkCell),
            work_unit: Some("Stamping".into()),
            process: Some("Stamping".into()),
            function: Some("MainPLC".into()),
            hardware_identities: vec![HardwareIdentity {
                kind: HardwareIdentityKind::MacAddress,
                value: "00:00:BC:AA:BB:CC".into(),
                label: Some("primary-nic".into()),
            }],
            application_identities: vec![ApplicationIdentity {
                kind: ApplicationIdentityKind::Urn,
                value: "urn:factory:asset:mainplc-cell2-stamping".into(),
                label: Some("scada-tag".into()),
            }],
            aliases: vec!["StampingPLC".into()],
            relations: vec![],
            status: RecordStatus::Active,
            updated_at: now,
            field_sources: make_field_sources(&[
                ("vendor", "rockwell", ObservationSource::ProtocolAnalysis),
                ("class", "plc", ObservationSource::ProtocolAnalysis),
                ("site", "Milwaukee", ObservationSource::ManualApi),
            ]),
        },
        // HMI in Cell2, Zone1
        SemanticRecord {
            device_id: Uuid::parse_str("55555555-5555-7555-8555-555555555555").unwrap(),
            fqdn: "OperatorHMI.Stamping.Cell2.Zone1.Milwaukee.local".into(),
            node_kind: Isa95NodeKind::Device,
            external_ip: Some("10.50.1.11".into()),
            internal_ip: Some("192.168.1.11".into()),
            class: Some("hmi".into()),
            vendor: Some("rockwell".into()),
            model: Some("PanelView-Plus-7".into()),
            protocols: vec!["ethernet-ip".into()],
            mac: Some("00:00:BC:DD:EE:FF".into()),
            switch_port: Some("Gi2/0/2".into()),
            enterprise: None,
            site: Some("Milwaukee".into()),
            facility: Some("Milwaukee".into()),
            area: Some("Zone1".into()),
            zone: Some("Zone1".into()),
            work_center: Some("Cell2".into()),
            cell: Some("Cell2".into()),
            work_center_kind: Some(Isa95WorkCenterKind::WorkCell),
            work_unit: Some("Stamping".into()),
            process: Some("Stamping".into()),
            function: Some("OperatorHMI".into()),
            hardware_identities: vec![HardwareIdentity {
                kind: HardwareIdentityKind::MacAddress,
                value: "00:00:BC:DD:EE:FF".into(),
                label: Some("primary-nic".into()),
            }],
            application_identities: vec![],
            aliases: vec!["StampingHMI".into()],
            relations: vec![SemanticRelation {
                relation: "served-by".into(),
                target: "MainPLC.Stamping.Cell2.Zone1.Milwaukee.local".into(),
                label: Some("plc".into()),
            }],
            status: RecordStatus::Active,
            updated_at: now,
            field_sources: make_field_sources(&[
                ("vendor", "rockwell", ObservationSource::DhcpFingerprint),
                ("class", "hmi", ObservationSource::ProtocolAnalysis),
                ("site", "Milwaukee", ObservationSource::ManualApi),
            ]),
        },
        // VFD on conveyor in Cell3, Zone3 (second conveyor drive)
        SemanticRecord {
            device_id: Uuid::parse_str("66666666-6666-7666-8666-666666666666").unwrap(),
            fqdn: "DriveVFD.Conveyor.Cell3.Zone3.Milwaukee.local".into(),
            node_kind: Isa95NodeKind::Device,
            external_ip: Some("10.50.3.20".into()),
            internal_ip: Some("192.168.1.20".into()),
            class: Some("vfd".into()),
            vendor: Some("rockwell".into()),
            model: Some("PowerFlex525".into()),
            protocols: vec!["ethernet-ip".into(), "modbus-tcp".into()],
            mac: Some("00:00:BC:56:78:9A".into()),
            switch_port: Some("Gi1/0/10".into()),
            enterprise: None,
            site: Some("Milwaukee".into()),
            facility: Some("Milwaukee".into()),
            area: Some("Zone3".into()),
            zone: Some("Zone3".into()),
            work_center: Some("Cell3".into()),
            cell: Some("Cell3".into()),
            work_center_kind: Some(Isa95WorkCenterKind::ProcessCell),
            work_unit: Some("Conveyor".into()),
            process: Some("Conveyor".into()),
            function: Some("DriveVFD".into()),
            hardware_identities: vec![HardwareIdentity {
                kind: HardwareIdentityKind::MacAddress,
                value: "00:00:BC:56:78:9A".into(),
                label: Some("primary-nic".into()),
            }],
            application_identities: vec![ApplicationIdentity {
                kind: ApplicationIdentityKind::Urn,
                value: "urn:factory:asset:drivevfd-cell3-conveyor".into(),
                label: Some("cmms-asset".into()),
            }],
            aliases: vec!["ConveyorVFD3".into()],
            relations: vec![],
            status: RecordStatus::Active,
            updated_at: now,
            field_sources: make_field_sources(&[
                ("vendor", "rockwell", ObservationSource::ProtocolAnalysis),
                ("class", "vfd", ObservationSource::ProtocolAnalysis),
                ("site", "Milwaukee", ObservationSource::ManualApi),
            ]),
        },
        // Managed switch in Zone3
        SemanticRecord {
            device_id: Uuid::parse_str("77777777-7777-7777-8777-777777777777").unwrap(),
            fqdn: "Switch01.Infra.Zone3.Milwaukee.local".into(),
            node_kind: Isa95NodeKind::Device,
            external_ip: Some("10.50.3.1".into()),
            internal_ip: Some("192.168.1.1".into()),
            class: Some("switch".into()),
            vendor: Some("cisco".into()),
            model: Some("IE-4000".into()),
            protocols: vec!["snmp".into(), "lldp".into()],
            mac: Some("AA:BB:CC:00:11:22".into()),
            switch_port: None,
            enterprise: None,
            site: Some("Milwaukee".into()),
            facility: Some("Milwaukee".into()),
            area: Some("Zone3".into()),
            zone: Some("Zone3".into()),
            work_center: None,
            cell: None,
            work_center_kind: None,
            work_unit: Some("Infra".into()),
            process: Some("Infra".into()),
            function: Some("Switch01".into()),
            hardware_identities: vec![HardwareIdentity {
                kind: HardwareIdentityKind::MacAddress,
                value: "AA:BB:CC:00:11:22".into(),
                label: Some("mgmt".into()),
            }],
            application_identities: vec![],
            aliases: vec!["Zone3CoreSwitch".into()],
            relations: vec![],
            status: RecordStatus::Active,
            updated_at: now,
            field_sources: make_field_sources(&[
                ("vendor", "cisco", ObservationSource::SwitchIntelligence),
                ("class", "switch", ObservationSource::SwitchIntelligence),
                ("site", "Milwaukee", ObservationSource::ManualApi),
            ]),
        },
    ]
}
