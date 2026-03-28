# System Architecture

## Overview

The platform is organized around a local industrial context plane. Raw network and protocol data enters the system through discovery and protocol adapters, is resolved into named assets and operational records, and is then exposed through the segmented query plane and conversational gateway.

Architecturally, this should be understood as a segmented-network-native alternative to a traditional flat-network UNS. Each zone maintains useful local context. Cross-zone visibility is achieved through gateways or explicit subscriptions rather than assuming one shared broker or one shared network plane.

## Core Components

### 1. Discovery Service

Purpose:
- detect assets on a zone or cell network
- infer device identity from passive and safe active methods
- maintain an up-to-date asset inventory

Inputs:
- packet captures
- switch and infrastructure data
- protocol fingerprints
- optional user confirmations

Outputs:
- discovered assets
- confidence scores
- topology hints

### 2. Semantic Registry

Purpose:
- assign stable names to assets
- store descriptive metadata and relationships
- provide a source of truth for asset identity

Responsibilities:
- naming rules
- aliases
- topology references
- zone, line, cell, and facility membership
- operator-approved overrides

This may use DNS-style naming semantics, but the parent repo should treat the registry as a product capability rather than assume one storage implementation too early.

Current implementation note:
- [`/Users/butterbones/semantic-dns`](/Users/butterbones/semantic-dns) already
  implements a large part of this layer, including semantic records, ISA-95
  naming, observation merge logic, DNS publishing, DHCP workflows, audit, and
  API surfaces.

### 3. Historian

Purpose:
- persist normalized operational records locally
- capture change over time
- support replay, troubleshooting, and trend queries

Sources:
- protocol adapters
- discovery events
- registry updates
- infrastructure telemetry

### 4. Correlation Engine

Purpose:
- derive higher-level events and patterns from local data
- detect relationships between devices, states, alarms, and network conditions

Examples:
- hot motor clusters
- repeated line stops
- device communication loss preceding downtime

### 5. Segmented Query Plane

Purpose:
- expose read-only access to assets, state, history, and derived events
- provide deterministic machine-facing contracts for UIs, gateways, and automation across segmented networks

Expected query categories:
- asset lookup
- topology lookup
- current state
- historical trends
- event search

### 6. Conversational Gateway

Purpose:
- translate conversational or tool-driven requests into structured segmented-query-plane calls
- provide AI clients and other consumers with grounded industrial context
- keep the platform model-agnostic

Model Context Protocol is one packaging option for this layer, but the product capability is broader than MCP itself. The conversational gateway should be an interface over the segmented query plane and registry, not the place where source-of-truth logic lives.

## Initial Data Flow

1. Discovery identifies or updates an asset.
2. Semantic registry resolves the asset into a stable identity.
3. Protocol adapters emit normalized records tied to that identity.
4. Historian stores time-series and event data.
5. Correlation engine derives higher-order signals.
6. The segmented query plane and conversational gateway expose the results in read-only form.

## Segmentation Model

- intra-zone: semantic records and supporting context can be replicated locally within a zone
- inter-zone: queries cross boundaries through gateways, or zones push selected data to downstream subscribers
- transport: MQTT may be used where it fits, but it is an implementation option rather than the architectural center
- trust boundary: the system should never assume flat, globally routable OT connectivity

## Trust and Security Model

- field-device writes are out of scope for the first release
- user overrides are explicit and auditable
- local deployments must remain useful even when disconnected
- external AI access should terminate at controlled read-only interfaces
- protocol parsing and collection should run with least privilege

## Parent Repo Boundaries

This repository should own:

- shared schemas and contracts
- common terminology and naming rules
- service interfaces
- example datasets and reference flows
- lightweight reference implementations

This repository should avoid:

- product-specific marketing language
- premature vendor-specific branching in the core model
- coupling to a single cloud provider or LLM vendor
