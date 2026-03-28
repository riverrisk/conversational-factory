# Conversational Factory

The Conversational Factory is a parent repository for building a segmented-network-native semantic and query layer that delivers UNS-like interoperability without requiring flat networks or IT/OT convergence.

This repo intentionally ignores company-specific branding, partner names, and sales language. It focuses on the technical product thesis.

## Core Thesis

Factories are hard to query because OT data is fragmented across protocols, devices, network zones, and undocumented local conventions. Traditional UNS approaches often assume flatter connectivity and central broker patterns that do not fit segmented industrial environments cleanly. A segmented-network-native semantic and query layer only becomes possible when the platform can:

1. Discover what exists on the network
2. Assign durable, human-readable context to assets
3. Normalize and store data close to the source
4. Expose that data through a secure, read-only interface for higher-level tools and AI

## Platform Pillars

- Asset discovery and classification
- Semantic naming and metadata
- Zone- or cell-level data services
- Intra-zone replication and inter-zone gateway queries
- Local historian and correlation
- Read-only MCP interface for conversational access

## Design Principles

- Read-only first: no device writes in the initial platform
- Standards over lock-in: prefer DNS, DHCP, TLS, SSH, OPC UA, MQTT, and HTTP where they fit
- Zone autonomy: local facilities and cells should remain independently useful
- Segmented by design: intra-zone replication is local; inter-zone access crosses boundaries through gateways or explicit subscriptions
- Brownfield-friendly: work in undocumented, mixed-vendor environments
- Security as architecture: segmentation and least privilege are defaults, not add-ons
- Incremental adoption: discovery and naming must provide value before full conversational features exist

## Initial Scope

This parent repo defines the shared architecture, schemas, and service boundaries for:

- Discovery
- Semantic registry
- Segmented UNS query plane
- Historian
- Correlation
- Conversational gateway
- Shared contracts and example datasets

## Existing Workstream

The semantic naming and DNS/DHCP control-plane work is already underway in
[`/Users/butterbones/semantic-dns`](/Users/butterbones/semantic-dns).

That repository appears to cover much of the semantic registry foundation:

- ISA-95-aware naming
- hardware and application identities
- observation merge logic
- DNS zone publication
- DHCP lease, quarantine, and fingerprint workflows
- audit trail and HTTP/WebSocket APIs
- operator console

This parent repo should treat `semantic-dns` as the current implementation of
the semantic registry layer, not a hypothetical future component.

## Repository Layout

- `docs/` product, architecture, and repository guidance
- `schemas/` shared data contracts for the segmented semantic layer, query plane, events, and conversational gateway
- `services/` executable services such as discovery, registry, historian, segmented query plane, and conversational gateway
- `examples/` sample assets, names, and query flows

## First Build Targets

1. Asset inventory model
2. Shared semantic naming contract extracted from `semantic-dns`
3. Read-only segmented query-plane contract
4. Observation contract for discovery-to-registry ingestion
5. Minimal discovery service
6. Minimal conversational gateway backed by sample data

See [docs/product-thesis.md](/Users/butterbones/conversational-factory/docs/product-thesis.md),
[docs/system-architecture.md](/Users/butterbones/conversational-factory/docs/system-architecture.md),
[docs/workstream-map.md](/Users/butterbones/conversational-factory/docs/workstream-map.md),
and [docs/contracts-roadmap.md](/Users/butterbones/conversational-factory/docs/contracts-roadmap.md)
for the cleaned-up concept and current implementation map.
