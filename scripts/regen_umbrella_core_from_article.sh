#!/usr/bin/env bash
# regen_umbrella_core_from_article.sh — helper for regenerating umbrella core
# .md files from the canonical article ~/Desktop/LongevityCommon.md.
#
# This script DOES NOT auto-rewrite the .md files (that requires LLM judgment).
# It DOES:
#   1. Compute md5 of the article
#   2. Compare with last-pinned md5 (in CONCEPT.md if present)
#   3. Check that all 10 required core .md files exist
#   4. Archive existing core .md to _archive/v_pre_<YYYY-MM-DD>/
#   5. Print a checklist for the LLM/human to follow
#   6. Optionally invoke an Agent task via Claude Code if --auto is passed
#
# Usage:
#   bash scripts/regen_umbrella_core_from_article.sh           # check + archive
#   bash scripts/regen_umbrella_core_from_article.sh --auto    # + invoke Claude Code
#
# Aligned with umbrella CONCEPT v5.6 (2026-04-28).

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ARTICLE="${HOME}/Desktop/LongevityCommon.md"
DATE_TAG="$(date +%Y-%m-%d)"
ARCHIVE_DIR="${ROOT}/_archive/v_pre_${DATE_TAG}"

REQUIRED=(CONCEPT.md THEORY.md DESIGN.md PARAMETERS.md MAP.md STATE.md EVIDENCE.md OPEN_PROBLEMS.md TODO.md README.md)

# ── Colours ────────────────────────────────────────────────────────────────
if [ -t 1 ]; then
  R='\033[31m'; G='\033[32m'; Y='\033[33m'; B='\033[34m'; C='\033[36m'; N='\033[0m'
else
  R=''; G=''; Y=''; B=''; C=''; N=''
fi

step() { echo -e "${B}══ $* ══${N}"; }
ok()   { echo -e "${G}✓${N} $*"; }
warn() { echo -e "${Y}⚠${N} $*"; }
err()  { echo -e "${R}✗${N} $*"; }

# ── 1. Article exists ──────────────────────────────────────────────────────
step "Step 1: verify article"
if [ ! -f "$ARTICLE" ]; then
  err "Article not found at $ARTICLE"
  exit 1
fi
NEW_MD5="$(md5sum "$ARTICLE" | awk '{print $1}')"
ok "Article: $ARTICLE"
ok "md5sum: $NEW_MD5"

# ── 2. Compare with pinned md5 ─────────────────────────────────────────────
step "Step 2: compare with pinned md5"
PINNED_MD5="$(grep -oE 'article_md5:\s*[a-f0-9]{32}' "${ROOT}/CONCEPT.md" 2>/dev/null | awk '{print $NF}' || true)"
if [ -z "${PINNED_MD5:-}" ]; then
  warn "No pinned article_md5 in CONCEPT.md; treating as first regeneration."
elif [ "$PINNED_MD5" = "$NEW_MD5" ]; then
  ok "md5 unchanged ($PINNED_MD5) — no regeneration needed."
  echo
  echo "If you want to force regeneration, edit CONCEPT.md to remove the article_md5 line."
  exit 0
else
  warn "md5 changed:"
  echo "    pinned: $PINNED_MD5"
  echo "    actual: $NEW_MD5"
fi

# ── 3. Check required core .md files exist ────────────────────────────────
step "Step 3: required core .md files"
missing=()
for f in "${REQUIRED[@]}"; do
  if [ ! -f "${ROOT}/$f" ]; then
    missing+=("$f")
  fi
done
if [ ${#missing[@]} -eq 0 ]; then
  ok "All ${#REQUIRED[@]} required files present."
else
  warn "Missing files: ${missing[*]}"
fi

# ── 4. Archive existing core .md ───────────────────────────────────────────
step "Step 4: archive existing core .md"
if [ -d "$ARCHIVE_DIR" ]; then
  warn "Archive directory $ARCHIVE_DIR already exists — appending."
fi
mkdir -p "$ARCHIVE_DIR"
archived=0
for f in "${REQUIRED[@]}"; do
  if [ -f "${ROOT}/$f" ]; then
    cp -p "${ROOT}/$f" "${ARCHIVE_DIR}/$f"
    archived=$((archived + 1))
  fi
done
ok "Archived $archived core .md → $ARCHIVE_DIR"

# ── 5. Print checklist ─────────────────────────────────────────────────────
step "Step 5: regeneration checklist (manual / LLM-assisted)"
cat <<CHECKLIST

The following files need to be regenerated from the article. Use Claude Code
with this prompt template:

────────────────────────────────────────────────────────────────────────────
Read ~/Desktop/LongevityCommon.md (canonical article, md5=${NEW_MD5}).
Regenerate the umbrella core .md files at ~/Desktop/LongevityCommon/:

  1. CONCEPT.md      — cross-cutting status, 5 components, falsifiability,
                       threat model. Pin article_md5: ${NEW_MD5} in §10.
  2. THEORY.md       — cross-subproject math summary; quantity propagation;
                       Lemma authority pointers to subprojects.
  3. DESIGN.md       — repository layout; 2-layer architecture (scientific
                       + social); cross-subproject API matrix.
  4. PARAMETERS.md   — cross-cutting constants (v*, weights, ε, ports);
                       falsifiability thresholds.
  5. MAP.md          — subproject ↔ EIC role mapping; quantity propagation
                       DAG; invariants U1-U7.
  6. STATE.md        — current state; subproject status snapshot; live
                       services; code audit list.
  7. EVIDENCE.md     — verified PMID/DOI/arXiv only; deleted/unverified
                       list; self-citation count.
  8. OPEN_PROBLEMS.md — scientific + engineering + governance issues;
                       cross-cutting only (subproject-level in
                       <subproject>/OPEN_PROBLEMS.md).
  9. TODO.md         — 5 phases plan with concrete deadlines.
 10. README.md       — public-facing intro.

Authority order on conflict: umbrella CONCEPT.md > <subproject>/CONCEPT.md
> <subproject>/THEORY.md > simulator code.

After writing, run:
  bash scripts/regen_umbrella_core_from_article.sh
to verify the new md5 is pinned.
────────────────────────────────────────────────────────────────────────────

CHECKLIST

# ── 6. Optional auto-mode ──────────────────────────────────────────────────
if [ "${1:-}" = "--auto" ]; then
  step "Step 6: --auto requested"
  warn "Auto-mode requires Claude Code with Agent tool. This script does not"
  warn "spawn an agent itself. Open Claude Code in this directory and paste"
  warn "the checklist prompt above."
fi

ok "regen_umbrella_core_from_article.sh complete."
echo "Next: regenerate the 10 .md files using the checklist above, then re-run"
echo "this script — it will detect the new md5 and confirm pinning."
