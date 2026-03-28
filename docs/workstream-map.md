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
- concept defined here
- implementation not yet established in this parent repo

Likely relationship:
- should produce observations that can feed `semantic-dns`

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
- concept defined here
- implementation not yet established in this parent repo

Likely relationship:
- should expose read-only UNS-like access across segmented networks
- should sit above semantic context and below any conversational tooling

### Conversational Gateway

Status:
- concept defined here
- implementation not yet established in this parent repo

Likely relationship:
- should sit above the segmented query plane and semantic context, not replace them

## Practical Implication

The parent repo should not rebuild the semantic DNS foundation from scratch.
It should define shared direction, schemas, and integration points around the
existing `semantic-dns` implementation while the other layers take shape.
