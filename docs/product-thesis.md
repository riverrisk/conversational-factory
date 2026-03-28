# Product Thesis

## Problem

Operational technology data is difficult to access because:

- assets are poorly documented or undocumented
- protocols and addressing models vary by vendor
- plant-floor data is split across isolated systems
- business and operations users depend on specialists to interpret basic questions
- existing paths to data often run through enterprise systems instead of the equipment boundary

The result is a factory that is observable only through specialists, not directly queryable by the people who need answers.

## Thesis

The Conversational Factory is a segmented-network-native semantic and query layer that delivers UNS-like interoperability without requiring flat networks or IT/OT convergence.

That requires a stack with four properties:

1. It can see the assets and traffic that already exist.
2. It can assign stable, human-readable context to those assets.
3. It can turn raw protocol activity into local operational records.
4. It can expose those records through a safe interface that higher-level software can query conversationally.

In practice, that means:

- intra-zone semantic context can be replicated locally
- inter-zone access happens through gateways or explicit subscriptions
- MQTT can be used where helpful, but the architecture does not depend on a single broker-centric UNS model

## Product Definition

The Conversational Factory is a layered OT data platform composed of:

- discovery and asset classification
- semantic identity and metadata
- zone-level storage and views
- a segmented UNS query plane
- local historian and correlation
- conversational access through a read-only gateway interface

## What Makes It Different

This is not just another historian, dashboard, or AI wrapper.

The essential value is the semantic layer between industrial reality and software consumers. Without that layer, AI sees addresses and values. With that layer, software can reason about assets, locations, roles, and relationships.

It also differs from a traditional flat-network UNS by treating segmentation as a first-class architectural constraint. The goal is to fulfill the role people want from a UNS without assuming a single shared network plane.

## Architectural Principles

### Read-Only First

Early versions should not write to field devices. The platform should start as an observation and query system.

### Context Before Intelligence

Advanced analytics and LLM features are only useful after naming, topology, and asset context are reliable enough to ground them.

### Brownfield Compatibility

The platform must assume mixed vendors, legacy networks, incomplete documentation, and inconsistent naming.

### Local Value

Each zone or cell should remain useful on its own. Cloud or enterprise integration is an extension, not a requirement for basic utility.

### Segmented-Network-Native

The platform should assume IEC 62443-style segmentation from the start. Local replication happens inside a zone. Cross-zone visibility should happen through gateways, controlled subscriptions, or other explicitly bounded paths.

### Standards-Based Interfaces

Prefer boring, interoperable foundations over new proprietary protocols wherever possible.

## User Outcomes

- Operators can ask direct questions about the equipment they work with.
- Maintenance teams can identify assets and conditions without decoding vendor-specific interfaces.
- Engineers get a live inventory and a searchable operational memory.
- Enterprise consumers get UNS-like structured OT context without owning the plant-floor integration problem or flattening the network.

## Recommended Build Sequence

1. Discovery and asset inventory
2. Semantic registry and naming rules
3. Historian and normalized event model
4. Segmented UNS query plane
5. Conversational gateway
6. Correlation and higher-order reasoning

## Non-Goals for the First Iteration

- full closed-loop control
- autonomous writes to production devices
- perfect ontology coverage across all industrial vendors
- cloud-first assumptions
- dependence on a single LLM provider
