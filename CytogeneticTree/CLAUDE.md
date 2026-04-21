# CLAUDE.md — CytogeneticTree

## Sources of truth

1. `CONCEPT.md` (this subproject)
2. `~/Desktop/CommonHealth/CONCEPT.md` (umbrella)
3. `~/Desktop/CommonHealth/CDATA/CONCEPT.md` (parent theory)
4. `~/Desktop/CommonHealth/MCOA/` (parent theoretical framework)
5. Global `~/CLAUDE.md`

## Rules

- **Literature:** NEVER use DeepSeek for literature search (feedback_deepseek_no_citations). ALL PMIDs must be verified via PubMed esummary API before entering any core file (feedback_verify_references).
- **Core files in sub-subprojects:** each technology subfolder has 5 files (CONCEPT + README + PARAMETERS + TODO + UPGRADE). Umbrella-level core (10 files) lives here at top level.
- **Cross-references:** when editing a tech subproject, check if it affects umbrella MAP.md, PARAMETERS.md, or KNOWLEDGE.md and update accordingly (per `feedback_cdata_docs_sync` pattern).
- **Python code:** for algorithms (genealogy reconstruction, image analysis wrappers), live in tech subfolder with own `pyproject.toml` or similar.
- **Experimental data:** raw image data stays in `AutomatedMicroscopy/data/` (gitignored) — CytogeneticTree tracks derived outputs only.

## Git

- Tracked under `djabbat/CommonHealth` (public view, core .md gitignored) AND `djabbat/CommonHealth-private` (full content).
- Large binary outputs (sample images, DAG visualizations >1 MB) → `docs/figures/` with Git LFS if needed, or excluded via .gitignore.

## Naming

- Do not translate "Cytogenetic Tree" into other languages in identifiers. CONCEPT.md preamble + outreach docs can render Russian/Georgian equivalents ("Цитогенетическое дерево дифференцировки" / "ციტოგენეტიკური დიფერენცირების ხე") but folder names stay English.

## What NOT to do

- Don't merge CytogeneticTree into CDATA — they are distinct (CDATA = theoretical mechanism; CytogeneticTree = empirical + computational methodology to test at lineage level).
- Don't abstract technology subfolders prematurely; each needs its own CONCEPT before becoming shared infrastructure.
- Don't start writing a manuscript before Phase 1 data exists.

## Self-citations

Applicable from master list in `~/CLAUDE.md` (≤15% of references). Most relevant for CytogeneticTree:

- Tkemaladze 2023 *Mol Biol Rep* PMID 36583780 — CDATA foundation
- Tkemaladze 2024 *Georgian Scientists* — centriole asymmetry review
- Chichinadze & Tkemaladze 2008 *Adv Gerontol* — centrosomal hypothesis of aging
- Tkemaladze & Chichinadze 2005 *Biochem (Moscow)* — centriolar differentiation mechanisms
