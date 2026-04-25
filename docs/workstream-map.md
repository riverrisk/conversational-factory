# Workstream Map

## Purpose

This document maps the parent-repo architecture to work that already exists in
the local workspace.

## Current Mapping

### Semantic Registry

Status:
- active implementation exists

Workspace:
- [`/Users/butterbones/semantic-dns`](/Users/butterbones/semantic-dns)

Observed capabilities:
- semantic record model
- ISA-95-aligned naming
- DNS zone publishing
- DHCP leases, quarantine, and fingerprinting
- audit ledger
- HTTP and WebSocket API
- operator console
- Fathom import path

Likely parent-repo role:
- reference implementation for the semantic registry layer
- source for shared schema extraction once contracts stabilize

### Discovery

Status:
- minimal implementation exists in this repo

Workspace:
- `services/discovery` — TCP connect-scan over a CIDR, reverse-DNS, ARP-cache enrichment, observations posted to `semantic-dns` or stdout

Current capabilities:
- per-host port probe with configurable port list and concurrency
- reverse DNS via `dns-lookup` (populates `aliases`)
- ARP scrape (populates `mac` and `hardware_identities`); MAC is preferred for stable `device_id` minting via UUID v5
- pluggable sink: stdout or HTTP POST

Known gaps:
- no active mDNS / SNMP / banner-grab probes yet (port from `~/fathomips/_active_probe.py`)
- no MAC OUI vendor enrichment yet
- no DPI / passive capture path (deferred until raw-socket or pcap-input story)

### Historian

Status:
- concept defined here
- implementation not yet established in this parent repo

Likely relationship:
- should consume normalized records tied to semantic identities

### Correlation

Status:
- concept defined here
- implementation not yet established in this parent repo

Likely relationship:
- should operate over historian data plus semantic context

### Segmented Query Plane

Status:
- minimal implementation exists in this repo

Workspace:
- `services/query-plane`

Current capabilities:
- read-only HTTP service exposing `record-filter` and `resolve-target` against a pluggable `AssetProvider`
- `UpstreamProvider` proxies to the `semantic-dns` HTTP API
- `SampleProvider` returns a hand-built fleet for offline development

### Conversational Gateway

Status:
- minimal implementation exists in this repo

Workspace:
- `services/conversational-gateway`

Current capabilities:
- intent parser and tool catalog that map conversational requests onto query-plane calls
- summary formatter that returns grounded responses without acting as a source of truth

## Practical Implication

The parent repo should not rebuild the semantic DNS foundation from scratch.
It should define shared direction, schemas, and integration points around the
existing `semantic-dns` implementation while the other layers take shape.
