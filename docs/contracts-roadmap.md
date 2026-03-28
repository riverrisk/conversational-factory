# Contracts Roadmap

## Purpose

This document separates contracts that were extracted from working code from
contracts that are proposed here to guide the next services.

## Extracted From `semantic-dns`

These contracts are grounded in the current implementation at
[`/Users/butterbones/semantic-dns`](/Users/butterbones/semantic-dns):

- [`identity-surfaces.schema.json`](/Users/butterbones/conversational-factory/schemas/common/v1/identity-surfaces.schema.json)
- [`observation.schema.json`](/Users/butterbones/conversational-factory/schemas/assets/v1/observation.schema.json)
- [`semantic-record.schema.json`](/Users/butterbones/conversational-factory/schemas/assets/v1/semantic-record.schema.json)
- [`record-filter.schema.json`](/Users/butterbones/conversational-factory/schemas/query-plane/v1/record-filter.schema.json)
- [`resolve-target.schema.json`](/Users/butterbones/conversational-factory/schemas/query-plane/v1/resolve-target.schema.json)
- [`query-records-response.schema.json`](/Users/butterbones/conversational-factory/schemas/query-plane/v1/query-records-response.schema.json)
- [`sync-status.schema.json`](/Users/butterbones/conversational-factory/schemas/system/v1/sync-status.schema.json)

These should be treated as the most concrete shared contracts in the repo right
now.

## Proposed In The Parent Repo

These contracts are not yet backed by a concrete implementation here, but they
define the interfaces the next services should converge on:

- [`asset-reference.schema.json`](/Users/butterbones/conversational-factory/schemas/assets/v1/asset-reference.schema.json)
- [`current-state-snapshot.schema.json`](/Users/butterbones/conversational-factory/schemas/state/v1/current-state-snapshot.schema.json)
- [`historian-event.schema.json`](/Users/butterbones/conversational-factory/schemas/events/v1/historian-event.schema.json)
- [`correlation-event.schema.json`](/Users/butterbones/conversational-factory/schemas/events/v1/correlation-event.schema.json)
- [`gateway-request.schema.json`](/Users/butterbones/conversational-factory/schemas/gateway/v1/gateway-request.schema.json)
- [`gateway-response.schema.json`](/Users/butterbones/conversational-factory/schemas/gateway/v1/gateway-response.schema.json)
- [`tool-catalog.schema.json`](/Users/butterbones/conversational-factory/schemas/gateway/v1/tool-catalog.schema.json)

Together these describe two higher-level capabilities:

- a segmented UNS query plane
- a conversational gateway that packages that query plane for MCP or similar tool ecosystems

## Recommended Implementation Order

1. Add a small query service that exposes the extracted query contracts against
   `semantic-dns`.
2. Add a minimal conversational gateway that maps tool or conversational requests into
   `record-filter` and `resolve-target`.
3. Add a historian prototype that emits `historian-event` and derives
   `current-state-snapshot`.
4. Add a first correlation worker that emits `correlation-event`.

## Practical Rule

If a contract conflicts with `semantic-dns` behavior today, the extracted
contract wins until the implementation is deliberately changed. Proposed
contracts should evolve to match reality, not the other way around.
