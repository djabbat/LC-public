# OPEN_PROBLEMS.md — ARGUS

## P1: Phase B laser upgrade

Q-switched 355 nm UV laser ($15-20K) + UV-transmitting objective ($8-12K)
required для single-centriole ablation в biology phase. Budget impact +$23-32K.

## P2: Centriolin-RITE marker stability

Royall 2023 PMID 37882444 показал >20 days stability — смешанные сигналы
tdTomato/NeonGreen при age determination. Mitigation: add Centrin-1-GFP secondary marker.

## P3: Actomyosin compensatory mechanism

Schaeffer 2025 PMID 40243666 — centrosome positioning не sole-MT-dependent. Phase B
control: blebbistatin 50 µM, time-lapse ≥6h post-ablation.

## P4: Tissue-specific generalizability

Mouse vs human NPC vs fibroblast — model choice matters. ARGUS-Aubrey targets human
fibroblasts BJ-hTERT, may need iPSC-derived NPCs для translation.

## P5: Photo-conversion timing (Centrin1-Kaede)

405 nm pulse duration / intensity для irreversible green→red switch не optimized.
Phase A: TetraSpeck beads characterization; Phase B: BJ-hTERT validation.

## P6: Physical Beacon implementation (NEW v4.0)

Hardware 10 Hz LED decoder on ESP32-S3 не реализован. Требуется:
- Frequency detection algorithm (FFT or zero-crossing)
- Hardware laser kill relay independent of AI
- 10+ stress test protocol (beacon hidden → laser blocked)
- Cost: ~$20 (LED + photodiode + comparator)

## P7: CUSUM control chart integration (NEW v4.0)

Python monitoring loop для CUSUM не реализован. Требуется:
- Sliding window of 50 cycles
- Lower control limit at 93% accuracy
- Auto-stop on boundary crossing
- Integration with Pi / Telegram alerts (overnight reporting)

## P8: AI Constitution enforcement (NEW v4.0)

5 prohibited actions (§12 CONCEPT) require static analysis hooks:
- bandit for Python tool functions
- Pre-commit hooks against constitution rules
- Claude Code CRFM guard prompt injection
- None implemented in current stack
