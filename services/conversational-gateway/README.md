# Conversational Gateway

## Purpose

This service is the conversational gateway for the platform. It should
translate natural-language or tool-driven intents into deterministic read-only
queries against the segmented query plane and semantic registry.

## Initial Responsibilities

- advertise available tools and read-only scope
- accept conversational query envelopes
- map requests to `resolve-target` and `record-filter`
- return grounded summaries plus matching semantic records
- package the segmented UNS query plane for MCP clients without making MCP the architecture

## Initial Contract Set

- [`gateway-request.schema.json`](/Users/butterbones/conversational-factory/schemas/gateway/v1/gateway-request.schema.json)
- [`gateway-response.schema.json`](/Users/butterbones/conversational-factory/schemas/gateway/v1/gateway-response.schema.json)
- [`tool-catalog.schema.json`](/Users/butterbones/conversational-factory/schemas/gateway/v1/tool-catalog.schema.json)

## First Tools

- `resolve_asset`
- `query_assets`
- `ask_factory`

## Guardrails

- no writes to field devices
- no direct source-of-truth ownership for semantic records
- every summary should be grounded in returned records or explicit not-found
- keep model-specific prompt logic separate from contract definitions
