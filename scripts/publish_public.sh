#!/usr/bin/env bash
# publish_public.sh — выборочная публикация научного контента LC → LC-public (публичный репозиторий)
#
# Правило Джабы (19.11.2026): в публичный LC-public идёт ТОЛЬКО научное.
#   ALLOW:  MCARA/CEDAR/simulator/** , Ze/** , MCARA/ARGUS_*.md
#   DENY:   MEMORY.md, TODO.md, EVIDENCE.md, STATE.md, PARAMETERS.md (на любом уровне)
# Всё остальное остаётся в приватной ветке main и никуда не уходит.
#
# Использование:  ./scripts/publish_public.sh [--dry-run]
set -euo pipefail

cd "$(dirname "$0")/.."          # корень LC
DRY="${1:-}"
BR=public-clean
REMOTE=public
DENY_RE='(^|/)(MEMORY|TODO|EVIDENCE|STATE|PARAMETERS)\.md$'
ALLOW_RE='^MCARA/CEDAR/simulator/|^Ze/|^MCARA/ARGUS_[^/]*\.md$'

echo "▶ обновляю refs из $REMOTE"
git fetch "$REMOTE" --prune

echo "▶ собираю список файлов из main (без gitlink-субмодулей)"
git ls-tree -r main | awk '$1!="160000"{print $4}' \
  | grep -E "$ALLOW_RE" | grep -vE "$DENY_RE" > /tmp/lc_publish_list.txt
echo "  файлов к публикации: $(wc -l < /tmp/lc_publish_list.txt)"

CUR=$(git rev-parse --abbrev-ref HEAD)
trap 'git checkout -q "$CUR" 2>/dev/null || true' EXIT

echo "▶ ветка $BR из $REMOTE/main"
git checkout -q -B "$BR" "$REMOTE/main"
xargs -a /tmp/lc_publish_list.txt -d '\n' git checkout main --

if [ -n "${DRY:-}" ]; then
  echo "--- DRY RUN: изменения, которые ушли бы ---"
  git diff --cached --stat | tail -20
  exit 0
fi

if git diff --cached --quiet; then
  echo "✅ публиковать нечего — LC-public уже актуален"
  exit 0
fi

echo "▶ фиксирую и пушу"
git -c user.name="pi" -c user.email="pi@local" commit -q \
  -m "public: science-only snapshot $(date '+%Y-%m-%d %H:%M') — CEDAR simulator, Ze, ARGUS docs (no MEMORY/TODO/EVIDENCE/STATE/PARAMETERS)"
git push "$REMOTE" "$BR:main"
echo "✅ опуб��иковано: $(git rev-parse --short HEAD)"
