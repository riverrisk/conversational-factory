# Repo Map

## Intent

This repository is the parent workspace for the core Conversational Factory platform. It should be able to host multiple services and shared contracts without forcing an early implementation choice for every subsystem.

## Current Structure

- `docs/`
  - product thesis
  - system architecture
  - repository guidance
- `schemas/`
  - shared contracts for assets, telemetry, events, and queries
- `services/`
  - runnable services and prototypes
- `examples/`
  - sample datasets, naming examples, and query flows

## Suggested Near-Term Additions

- `schemas/assets/`
- `schemas/events/`
- `schemas/query-plane/`
- `schemas/gateway/`
- `services/discovery/`
- `services/registry/`
- `services/historian/`
- `services/query-plane/`
- `services/conversational-gateway/`
- `examples/facility-a/`

## Working Conventions

- keep shared schemas implementation-agnostic where possible
- prefer explicit versioned contracts once services begin to consume them
- keep marketing and product positioning out of service packages
- treat read-only access as the default contract until a stronger control model exists
