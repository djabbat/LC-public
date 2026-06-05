# CDATA — Concept

## Description
CDATA is a next-generation data integration and analytics platform designed to unify disparate data sources into a single, queryable interface. It eliminates the need for complex ETL pipelines by providing real-time access to data from databases, APIs, and files.

## Purpose & Motivation
**Purpose:** To enable organizations to make data-driven decisions by providing instant, unified access to all their data without moving or transforming it beforehand.  
**Motivation:** Traditional data warehouses are slow, expensive, and require extensive upfront modeling. CDATA offers a lightweight, schema-on-read approach that reduces time-to-insight from weeks to minutes.

## Approach
CDATA utilizes a distributed query engine with in-memory caching to connect to over 50 data sources. Users can run standard SQL queries across multiple sources simultaneously. The platform supports both cloud and on-premise deployments, with automatic schema discovery and query optimization.

## Key Metrics
- Query latency: < 100ms (P99)
- Data freshness: < 1 minute for most sources
- Number of supported connectors: > 50
- Uptime: 99.9% SLA

## Constraints
- Must comply with GDPR and SOC2 regulations
- Must support on-premise deployment for air-gapped environments
- Maximum query timeout: 30 seconds

## Status
🟡 In development

## Goals & Tasks
- **P0** – Launch MVP with 10 core connectors (PostgreSQL, MySQL, Snowflake, BigQuery, S3, REST API, etc.) by **upon approval**
- **P0** – Achieve sub-100ms query latency for single-source queries by **after completion of prerequisite tasks**
- **P1** – Add support for 20 additional connectors (MongoDB, Redshift, Kafka, etc.) by **after previous milestone**
- **P1** – Implement cross-source JOINs with sub-500ms latency by **after previous milestone**
- **P2** – Introduce streaming data ingestion (Kafka, Kinesis) by **after previous milestone**
- **P0** – Obtain SOC2 Type II certification by **next phase**
- **P2** – Build a visual query builder UI by **next phase**