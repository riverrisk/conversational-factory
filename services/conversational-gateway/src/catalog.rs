use cf_shared::gateway::{ToolCatalog, ToolDefinition};

pub fn tool_catalog() -> ToolCatalog {
    ToolCatalog {
        server_name: "conversational-factory".into(),
        version: "0.1.0".into(),
        tools: vec![
            ToolDefinition {
                name: "resolve_asset".into(),
                description: "Resolve a single asset by FQDN, IP address, MAC address, alias, or application identity. Returns the full semantic record for the matched device.".into(),
                input_schema: r#"{"type":"object","properties":{"target":{"type":"string","description":"FQDN, IP, MAC, alias, or application identity to resolve"}},"required":["target"]}"#.into(),
                output_schema: Some("schemas/assets/v1/semantic-record.schema.json".into()),
                read_only: true,
            },
            ToolDefinition {
                name: "query_assets".into(),
                description: "Query factory assets using composable filters. Filter by vendor, class, ISA-95 hierarchy (site, area, work center, work unit), protocol, or free-text search.".into(),
                input_schema: r#"{"type":"object","properties":{"q":{"type":"string"},"vendor":{"type":"string"},"class":{"type":"string"},"site":{"type":"string"},"area":{"type":"string"},"work_center":{"type":"string"},"work_unit":{"type":"string"},"zone":{"type":"string"},"cell":{"type":"string"},"node_kind":{"type":"string","enum":["site","area","work-center","work-unit","device"]}}}"#.into(),
                output_schema: Some("schemas/query-plane/v1/query-records-response.schema.json".into()),
                read_only: true,
            },
            ToolDefinition {
                name: "ask_factory".into(),
                description: "Ask a natural-language question about factory assets and topology. Translates the question into structured queries and returns a grounded summary with citations.".into(),
                input_schema: r#"{"type":"object","properties":{"question":{"type":"string","description":"Natural-language question about factory assets"}},"required":["question"]}"#.into(),
                output_schema: Some("schemas/gateway/v1/gateway-response.schema.json".into()),
                read_only: true,
            },
        ],
    }
}
