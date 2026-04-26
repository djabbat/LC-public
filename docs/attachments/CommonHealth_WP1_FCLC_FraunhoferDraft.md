# WP1 — FCLC Federated Clinical Learning Cooperative
## Scope for Fraunhofer IESE Technical Partnership

**EIC Pathfinder Open 2026** · LongevityCommon · €0.6M · 12 months · TRL 3→5

---

## Objective

Build a **production-grade, EU-compliant federated learning infrastructure** for multi-institutional clinical data analysis, achieving system-level privacy guarantee ε ≤ 1.0 and demonstrating functionality in a real-world clinical pilot (WP5 — anemia cohort N=50) by Month 24.

## Technical stack (current v6.2, to be hardened)

| Layer | Technology | Current status | Target |
|-------|-----------|----------------|--------|
| **Secure aggregation** | SecAgg+ with ChaCha20-Poly1305 + SHA-256 seed + Shamir GF(257) dropout recovery | ✅ Implemented, 96/96 tests pass | Production audit |
| **Differential privacy** | Laplace mechanism + Rényi DP accounting | ε = 2.0 (research prototype) | **ε ≤ 1.0** (compliance target) |
| **Coordination layer** | Elixir/Phoenix LiveView | Functional | Multi-node scaling |
| **Data schema** | BioSenseExport JSON, FHIR-compatible | ✅ Specified | FHIR-Starter integration |
| **Compliance** | GDPR Art. 28 DPA template | ✅ Drafted | EU AI Act Art. 22 alignment |

## Tasks

| Task | Months | Lead | Effort |
|------|--------|------|--------|
| **T1.1** Architecture refinement + security threat model | 1-6 | Fraunhofer IESE (proposed) | 6 PM |
| **T1.2** DP hardening to ε ≤ 1.0 (PATE/DP-SGD redesign) | 3-12 | Fraunhofer + GLA | 10 PM |
| **T1.3** Multi-node production deployment (3-node Synthea demo) | 6-15 | GLA + Fraunhofer | 8 PM |
| **T1.4** WP5 clinical integration (anemia cohort FL) | 12-24 | GLA clinical + Fraunhofer | 6 PM |

## Deliverables

- **D1.1** (M6): Architecture specification + threat model
- **D1.2** (M12): FCLC v2.0 open-source release (GitHub)
- **D1.3** (M18): Independent cryptographic + privacy audit report
- **D1.4** (M24): Multi-institutional clinical deployment demo

## Milestones

- **M1.1** (M6): Target ε ≤ 1.0 achieved on synthetic data
- **M1.2** (M18): 3-node production demo complete
- **M1.3** (M24): WP5 clinical integration live (Aqtivirebuli anemia pilot)

## Role for Fraunhofer IESE

**Co-lead WP1** with Georgia Longevity Alliance. Primary responsibilities:
1. Production-grade software engineering rigor (CI/CD, compliance testing)
2. Regulatory alignment (EU AI Act Art. 22 for high-risk medical AI)
3. FHIR/medical data standards integration (leverage FHIR-Starter expertise)
4. Cryptographic audit coordination with partner (KU Leuven COSIC proposed)
5. Technical deliverables D1.1-D1.4

## Budget allocation (indicative)

| Category | % |
|----------|---|
| Personnel (software engineers + researchers) | 65% |
| Equipment (compute, test infrastructure) | 5% |
| Travel (consortium meetings, conferences) | 10% |
| Subcontracting (external audit) | 15% |
| Other (dissemination, overhead) | 5% |

## Risks

| Risk | Impact | Mitigation |
|------|--------|-----------|
| ε ≤ 1.0 target technically challenging | High | PATE/DP-SGD hybrid redesign in T1.2 |
| EU AI Act Art. 22 evolving interpretation | Medium | Fraunhofer regulatory expertise; quarterly review |
| Clinical data availability (WP5 dependency) | Medium | Synthea synthetic data for pre-deployment testing |

## Why Fraunhofer IESE

- Institute mission alignment: Digital Health + medical AI software engineering
- Existing project reference: FHIR-Starter (AI-based software for structuring medical data)
- Production-grade engineering rigor — critical differentiator vs. academic-only partners
- Regulatory compliance track record in EU medical AI context
- Geographic/institutional independence from Host (strengthens Consortium quality section)

---

*This scope is non-binding at LoI stage. Detailed PM allocation, budget, and deliverable refinement will follow Grant Agreement negotiation.*
