# AutomatedMicroscopy — LC subproject

**Purpose:** Low-cost ($4,500) AI-operated time-lapse microscopy platform for round-the-clock live-cell imaging, enabling single-PI labs to conduct industrial-grade imaging experiments without human shift overhead.

**Parent ecosystem:** LC (longevity research ecosystem)
**Flagship role:** Experimental infrastructure for CEDAR Phase A (Impetus Grant 2026-04-25) + future MCARA Counter validation experiments

**Status:** Engineering design complete (2026-04-21). Bill-of-materials ready. Assembly expected Months 1-2 of Phase A Impetus grant (if funded).

**Core innovation:** Claude Code `/overnight` mode controls the microscope, interpreting natural-language PROMPT (description of experiment goals and tasks), making routine decisions autonomously and signaling a human only at strategically important events.

**Budget target:** $4,500 retrofit (Option A DIY) vs $12,700 mid-tier (Option B) vs $25-50k turnkey (Option C).

## Quick links

- **Theory:** see `THEORY.md`
- **Evidence / references:** see `EVIDENCE.md`
- **Open problems / research questions:** see `OPEN_PROBLEMS.md`
- **Bill of materials / quantitative params:** see `PARAMETERS.md`
- **System architecture / code structure:** see `DESIGN.md`
- **AI agent instructions:** see `AGENTS.md`
- **Changelog / decisions:** see `JOURNAL.md`
- **Future roadmap:** see `ROADMAP.md`

## Context in the LC ecosystem

AutomatedMicroscopy — **infrastructure layer** for experimental subprojects (CEDAR, Telomere, MitoROS, EpigeneticDrift, Proteostasis) that require prolonged live-cell imaging.

Comparison with other subprojects:
- **CEDAR, Telomere, etc.** — scientific hypotheses / damage counters
- **FCLC** — federated data sharing infrastructure
- **MCARA** — theoretical framework
- **AutomatedMicroscopy (this)** — experimental infrastructure for data collection

## Links

- Parent: `~/Desktop/LC/CONCEPT.md`
- Related grant: `~/Documents/Grants/LC/CEDAR/docs/IMPETUS_2026-04-25/`
- External source: `~/Documents/Engineering/AutomatedMicroscopy_2026-04-21/`

## License

MIT (all code + BOM + PROMPT templates released post-Phase A).