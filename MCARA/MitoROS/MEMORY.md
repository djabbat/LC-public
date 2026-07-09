# MitoROS — Memory

## History of Decisions

### after completion of prerequisite tasks — Project Initialization
- **Decision:** Created MitoROS project
- **Rationale:** Need for structured development of a real-time operating system for mitochondrial research
- **Status:** Completed

### after completion of prerequisite tasks — Core Architecture Selection
- **Decision:** Adopt microkernel architecture with priority-based scheduling
- **Rationale:** Ensures deterministic behavior required for real-time data acquisition from mitochondrial sensors
- **Status:** Completed

### after completion of prerequisite tasks — Communication Protocol
- **Decision:** Use MQTT-SN for low-power sensor nodes, gRPC for high-bandwidth data streams
- **Rationale:** Balances energy efficiency with throughput for heterogeneous mitochondrial monitoring devices
- **Status:** In Progress

## Goals & Tasks

### Project Objectives (P0 – Critical)
- **Goal 1:** Deliver a stable real-time kernel after previous milestone
  - Task 1.1: Implement task scheduler with O(1) context switching (P0, due after
## 2026-07-09 — Глубокий аудит MCARA
- **Находка:** 4 митохондриальные протеазы (ClpP, YME1L, LONP1, PARL) → cell fate
- **Находка:** MLKL → non-lethal mitochondrial damage → HSC aging (Yamada 2026, Nat Commun)
- **Находка:** UPR^mt → HSC quiescence exit (Mohrin 2018) и NSC aging (Wang 2023)
- **Решение:** Добавлены находки в EVIDENCE.md §v4
