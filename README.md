# Conversational Factory

The Conversational Factory is a parent repository for building a segmented-network-native semantic and query layer that delivers UNS-like interoperability without requiring flat networks or IT/OT convergence. The end state is a factory that is *speakable* — where operators, existing tools, and AI systems can all ask questions in business terms and get answers grounded in live operational data.

This repo intentionally ignores company-specific branding, partner names, and sales language. It focuses on the technical product thesis.

## The Fundamental Problem

Industrial facilities are drowning in operational data while starving for operational intelligence. Networks already contain comprehensive production information, but it remains locked behind technical complexity — protocol fragmentation, undocumented local conventions, segmented zones, and devices that only make sense to the engineer who installed them. Both human operators and AI systems are blocked by the same wall.

Three converging forces make this urgent:

1. **Compliance pressure.** IEC 62443 and related mandates require comprehensive network documentation, segmentation verification, and continuous monitoring. Most organizations lack the tooling to meet these deadlines without flattening their networks.
2. **Skills attrition.** Engineers who hold the tacit knowledge of industrial networks — IP assignments, device relationships, troubleshooting patterns — are retiring faster than they can be replaced. That expertise must be captured into infrastructure before it walks out the door.
3. **AI integration failures.** Industrial AI initiatives consistently stall at the data layer. Models receive raw technical signals they cannot interpret, because no semantic layer translates between OT reality and the contextualized data AI requires.

## Core Thesis

Industrial networks do not need replacement — they need translation. By applying semantic intelligence at the network edge, every existing tool, monitoring platform, and AI system becomes industrially intelligent without modification to the devices themselves.

A segmented-network-native semantic and query layer only becomes possible when the platform can:

1. Discover what exists on the network
2. Assign durable, human-readable context to assets
3. Normalize and store data close to the source
4. Expose that data through a secure, read-only interface for higher-level tools and AI

## Solution Architecture

The platform is composed of layered capabilities that each provide standalone value while composing into the full conversational stack:

1. **Semantic DNS** — transforms `192.168.1.47` into `Cell3.Conveyor.DriveVFD`. The foundation that makes everything else conversational.
2. **Proxy ARP / gateway addressing** — consistent addressing across identical cells, solving routing without disrupting device configurations.
3. **Brownfield discovery** — fast visibility into existing networks without changes, making installed infrastructure accessible.
4. **SPAN automation** — guided monitoring configuration that does not require a network engineer to set up.
5. **Network time-series capture** — operational data harvested from protocol intelligence, with minimal configuration burden.
6. **Switch intelligence** — infrastructure that self-reports operational state, enabling predictive maintenance of the network itself.
7. **Conversational gateway (MCP)** — a read-only Model Context Protocol interface so that natural-language queries like "show me all motors running hot" work everywhere.

These layers naturally evolve toward a **central correlation engine** that aggregates semantic data across cells, lines, facilities, and enterprises — and toward an **industrial Model Context Protocol** that defines how AI systems access factory operations universally.

## Phased Value Delivery

The architecture delivers value in three stages, each independently useful:

- **Stage 1 — Universal Data Archaeology (immediate):** vendor-agnostic, protocol-agnostic discovery and identification of operational data across every device, system, and network regardless of vintage.
- **Stage 2 — Cell-Level Data Warehouse (immediate):** local storage and organization of discovered data at the cell level, creating the structured foundation required for ML and LLM integration.
- **Stage 3 — Conversational Intelligence Layer (future):** natural-language data exchange with operators on the plant floor — technicians asking "why is my line running slow?" and getting actionable answers in their language, not IT speak.

## Design Principles

- **Translation, not replacement.** Meet brownfield networks where they are; do not require devices, protocols, or topologies to change.
- **Read-only first.** No device writes in the initial platform.
- **Standards over lock-in.** Prefer DNS, DHCP, TLS, SSH, OPC UA, MQTT, and HTTP where they fit.
- **Zone autonomy.** Local facilities and cells should remain independently useful.
- **Segmented by design.** Intra-zone replication is local; inter-zone access crosses boundaries through gateways or explicit subscriptions.
- **Brownfield-friendly.** Work in undocumented, mixed-vendor environments by default.
- **Security as architecture.** Segmentation and least privilege are defaults, not add-ons.
- **Incremental adoption.** Discovery and naming must provide value before full conversational features exist.
- **Capture tacit expertise.** Naming, fingerprinting, and correlation patterns should encode the knowledge of retiring engineers as durable infrastructure.

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

The semantic naming and DNS/DHCP control-plane work lives at https://github.com/eris-securitas/semantic-dns

That repository covers much of the semantic registry foundation:

- ISA-95-aware naming
- hardware and application identities
- observation merge logic
- DNS zone publication
- DHCP lease, quarantine, and fingerprint workflows
- audit trail and HTTP/WebSocket APIs
- operator console

The remaining services in this repo (`services/`) compose around that registry: a discovery service that feeds it observations, a read-only query plane that fronts it, and a conversational gateway that turns natural-language requests into query-plane calls.

## Repository Layout

- `docs/` product, architecture, and repository guidance
- `schemas/` shared data contracts for the segmented semantic layer, query plane, events, and conversational gateway
- `services/` executable services — `shared` (contracts), `discovery`, `query-plane`, `conversational-gateway`
- `examples/` sample assets, names, and query flows

## Build Status

**Implemented in this repo:**

- `services/shared` — Rust contract types (assets, identity, query, gateway, events, state, system)
- `services/query-plane` — read-only HTTP query service over the semantic registry, with a `semantic-dns` upstream provider and a sample-data fallback
- `services/conversational-gateway` — intent parser + tool catalog that maps conversational requests onto query-plane calls
- `services/discovery` — TCP connect-scan over a CIDR, reverse-DNS via `dns-lookup`, ARP-cache enrichment, observations posted to `semantic-dns` (or stdout for dry-run)
- Schemas extracted from `semantic-dns` for observation, semantic-record, identity surfaces, query filter/response, sync status

**Next:**

- Historian prototype emitting `historian-event` and deriving `current-state-snapshot`
- Correlation worker emitting `correlation-event`
- Active mDNS / SNMP / banner-grab probes (port from `~/fathomips/_active_probe.py`)
- DPI dissector path — wire `marlinspike-dpi` once the discovery service has captured packets to feed it

See [docs/product-thesis.md](/Users/butterbones/conversational-factory/docs/product-thesis.md),
[docs/system-architecture.md](/Users/butterbones/conversational-factory/docs/system-architecture.md),
[docs/workstream-map.md](/Users/butterbones/conversational-factory/docs/workstream-map.md),
and [docs/contracts-roadmap.md](/Users/butterbones/conversational-factory/docs/contracts-roadmap.md)
for the cleaned-up concept and current implementation map.

## License

This work is licensed under [Creative Commons Attribution-NonCommercial 4.0 International (CC BY-NC 4.0)](https://creativecommons.org/licenses/by-nc/4.0/). See [LICENSE](./LICENSE).

Commercial use requires a separate written commercial license agreement. To request one, contact **river@riverman.io**.
