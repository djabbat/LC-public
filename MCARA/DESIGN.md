# MCARA Architecture and Design

*Version: 2026-04-22. Describes the reference implementation on Rust, file structure, and API contracts.*

## 1. Architecture Overview

MCARA is implemented as a crate (library) on Rust with a clear separation into:
1. **Core (`mcara_core`):** Pure, deterministic functions implementing the MCARA formalism (counter equations, load, thresholds). Without input/output dependencies.
2. **Simulator (`mcara_simulation`):** Modules for conducting simulations (stochastic processes, cell populations, longitudinal trajectories). Uses the core.
3. **Interfaces (`mcara_interfaces`):** Data type definitions, serialization (JSON/MessagePack), API for integration with other subprojects (CEDAR, FCLC).
4. **Tools (`mcara_tools`):** Command-line utilities (CLI) for calibration, sensitivity analysis, visualization.

Goal: to provide a verifiable, efficient, and portable reference implementation for the scientific community.

## 2. Project File Structure


mcara_reference_impl/
├── Cargo.toml # Crate configuration and dependencies

## MCARA Phase III Update (2026-05-15)

CONCEPT.md replaced with MCARA Phase III v2.0 — corrected version with verified PMID.
See CONCEPT.md for details of Phase III design (6 arms, power calculation, budget).