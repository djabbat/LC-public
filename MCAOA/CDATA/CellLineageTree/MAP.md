# MAP — CytogeneticTree

## Dependency graph across subfolders

```
 ┌──────────────────────────────────┐
 │ CytogeneticTree (umbrella) │
 │ ─────────────────────────── │
 │ CONCEPT + 10 core files │
 └───┬──────────────────────────────┘
 │
 ┌──────────────┼──────────────────────┐
 ▼ ▼ ▼
[Biology] [Hardware] [Algorithm]
 │ │ │
 ▼ ▼ ▼
RITE_Centriole LiveCellMicroscopy CellPose_Segmentation
LentiviralTools FluorescentCameras LaserAblation_405
 MicroscopeController AICoordinator
 ImageAnalysis
 StatisticalAnalysis
 GenealogyReconstruction
 DifferentiationAnnotation
```

## Information flow during one experimental run

```
 1. RITE pulse (Cre-ERT2 + tamoxifen) — biology
 │
 ▼
 2. Live-cell imaging (Zeiss IM 35 + 100× + mono camera) — hardware
 │ continuous stream: every 20 min, 4 channels
 ▼
 3. MicroscopeController (Micro-Manager 2.0 + PyMMCore-Plus) — automation
 │ sends frames to AI
 ▼
 4. CellPose_Segmentation — algorithm
 │ outputs: per-cell masks
 ▼
 5. AICoordinator (Claude /overnight) — orchestration
 │ analyzes fluorescence per daughter cell
 │ makes ablation decisions (per experimental arm)
 │
 ├─ → LaserAblation_405 (if arm requires)
 │
 ├─ → division-event log entry
 │
 ▼
 6. GenealogyReconstruction — algorithm
 │ builds DAG from division events
 │
 ▼
 7. ImageAnalysis — algorithm
 │ quantifies polyGlu, Ninein, ARL13B on fixed endpoints
 │
 ▼
 8. StatisticalAnalysis — algorithm
 │ log-rank on survival, Bayesian MCMC on parameters
 │
 ▼
 9. DifferentiationAnnotation — algorithm
 │ maps tree nodes to differentiation states
 │
 ▼
 10. Output: Cytogenetic Tree visualization + dataset + manuscript
```

## Integrations with LC ecosystem

```
 CytogeneticTree ──┬──→ CDATA (theoretical validation data)
 │
 ├──→ MCAOA (Counter #1 lineage-level instance)
 │
 ├──→ AutomatedMicroscopy (shared hardware platform)
 │
 ├──→ BioSense / Ze (orthogonal biomarker context)
 │
 └──→ Impetus LOI (funding vehicle for Phase 1 MVCT)
```

## External data flow

```
 Addgene / Twist Bio
 │
 │ (RITE construct plasmids)
 ▼
 LentiviralTools/ ─────→ live culture ─────→ Phase 1 data
 ▲ │
 │ │
 CCP1/TTLL6 constructs ▼
 (rescue + pro-damage controls) Zenodo dataset release
 (DOI + raw images)
 │
 ▼
 Companion paper
```


## v3 Update (2026-05-13)

См. CONCEPT.md "v3" / "Адрес peer-review concerns" секцию для project-specific changes.

