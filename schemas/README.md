# Schemas

Shared contracts for the segmented semantic layer, UNS-like query plane,
telemetry, events, and conversational gateway live here.

## Current Contracts

- [`common/v1/identity-surfaces.schema.json`](/Users/butterbones/conversational-factory/schemas/common/v1/identity-surfaces.schema.json)
- [`assets/v1/asset-reference.schema.json`](/Users/butterbones/conversational-factory/schemas/assets/v1/asset-reference.schema.json)
- [`assets/v1/observation.schema.json`](/Users/butterbones/conversational-factory/schemas/assets/v1/observation.schema.json)
- [`assets/v1/semantic-record.schema.json`](/Users/butterbones/conversational-factory/schemas/assets/v1/semantic-record.schema.json)
- [`query-plane/v1/record-filter.schema.json`](/Users/butterbones/conversational-factory/schemas/query-plane/v1/record-filter.schema.json)
- [`query-plane/v1/resolve-target.schema.json`](/Users/butterbones/conversational-factory/schemas/query-plane/v1/resolve-target.schema.json)
- [`query-plane/v1/query-records-response.schema.json`](/Users/butterbones/conversational-factory/schemas/query-plane/v1/query-records-response.schema.json)
- [`system/v1/sync-status.schema.json`](/Users/butterbones/conversational-factory/schemas/system/v1/sync-status.schema.json)
- [`state/v1/current-state-snapshot.schema.json`](/Users/butterbones/conversational-factory/schemas/state/v1/current-state-snapshot.schema.json)
- [`events/v1/historian-event.schema.json`](/Users/butterbones/conversational-factory/schemas/events/v1/historian-event.schema.json)
- [`events/v1/correlation-event.schema.json`](/Users/butterbones/conversational-factory/schemas/events/v1/correlation-event.schema.json)
- [`gateway/v1/gateway-request.schema.json`](/Users/butterbones/conversational-factory/schemas/gateway/v1/gateway-request.schema.json)
- [`gateway/v1/gateway-response.schema.json`](/Users/butterbones/conversational-factory/schemas/gateway/v1/gateway-response.schema.json)
- [`gateway/v1/tool-catalog.schema.json`](/Users/butterbones/conversational-factory/schemas/gateway/v1/tool-catalog.schema.json)

## Source Of Truth

These schemas are extracted from the currently running semantic registry work in
[`/Users/butterbones/semantic-dns`](/Users/butterbones/semantic-dns), especially
the `sdns-core` domain model. The parent repo owns the shared contract shape;
the implementation repo still owns runtime behavior until those contracts are
stabilized more formally.

The `queries/` and `system/` schemas are mostly direct extractions from the
current semantic-dns API surface and form the base of the segmented UNS query
plane. The `state/`, `events/`, and `mcp/` schemas are parent-repo proposals
for the layers that sit above that read-only query plane.

## Next Contracts To Add

- historian query windows and aggregations
- topology graph response
- correlation rule definition
- conversational-gateway capability advertisement with auth and tenant metadata
