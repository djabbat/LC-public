<!-- AUTO-TRANSLATED from README.md via DeepSeek 2026-05-13. Source language: russian. Original (README.md) is canonical; re-run scripts/translate_core_files.py after edits. -->

# AIM v8.0 — Architecture Integration Matrix

**The heart of the LongevityCommon ecosystem.** Not AI. Not medicine. An integration hub.

## What It Is

A central registry, dependency graph, and cross-project validation for 15 LongevityCommon projects.

## Launch

```bash
# Show status of all projects
python3 dashboard/status.py

# Generate ecosystem graph
dot graph/ecosystem.dot -Tpng -o graph/ecosystem.png

# Check consistency
python3 validate/counter_numbering.py
python3 validate/ze_vstar.py
python3 validate/concept_versions.py
```

## Structure

- `registry.json` — machine-readable registry (canon)
- `validate/` — cross-project validation scripts
- `dashboard/` — status dashboard
- `graph/` — graph in DOT/Mermaid
- `MAP.md` — dependency map (human-readable)

## Archive

`_archive/v7_ai_code/` — complete AIM v7.0 code (AI medical system). Removed from concept on 2026-05-09.

## v3 Update (2026-05-13)

CONCEPT.md updated with TBPR peer-review responses:
- Verified PMIDs through PubMed esummary (per `feedback_pmid_verify_always`)
- Removed fabricated references
- Addressed top blocking/critical reviewer concerns
- Statistical protocol additions where applicable
- Honest TODO sections for unmet requirements

See `CONCEPT.md` Section marked "v3" / "Address peer-review concerns"
for project-specific changes.