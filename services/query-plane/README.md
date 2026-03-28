# Query Plane

## Purpose

This service is the read-only segmented query plane that sits above the
semantic registry and below the conversational gateway.

In the near term, it should act as a thin compatibility layer over
[`/Users/butterbones/semantic-dns`](/Users/butterbones/semantic-dns), exposing
shared contracts from this parent repo rather than leaking implementation-local
types.

## Initial Responsibilities

- resolve a single asset target
- query semantic records with `record-filter`
- expose sync and health status
- normalize response shapes against parent-repo schemas
- provide UNS-like read access without assuming flat network reachability

## Initial Contract Set

- [`resolve-target.schema.json`](/Users/butterbones/conversational-factory/schemas/query-plane/v1/resolve-target.schema.json)
- [`record-filter.schema.json`](/Users/butterbones/conversational-factory/schemas/query-plane/v1/record-filter.schema.json)
- [`query-records-response.schema.json`](/Users/butterbones/conversational-factory/schemas/query-plane/v1/query-records-response.schema.json)
- [`sync-status.schema.json`](/Users/butterbones/conversational-factory/schemas/system/v1/sync-status.schema.json)

## First Endpoints

- `GET /health`
- `GET /assets/resolve`
- `GET /assets/query`
- `GET /sync-status`

## Implementation Notes

- keep this service read-only
- translate directly to `semantic-dns` APIs first
- add caching only after correctness is established
- avoid embedding conversational logic here
- treat gateway traversal and subscription boundaries as explicit concerns, not transport details
