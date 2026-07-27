#!/bin/bash
# =============================================================================
# CEDAR AUTOFIX SCRIPT v1.0
# Адаптирован из DEEP_AUDIT_ALGORITHM.md специально для CEDAR
# =============================================================================

PROJECT_ROOT="/home/oem/Desktop/LC/MCARA/CEDAR"
cd "$PROJECT_ROOT"

SCORE=0
MAX_SCORE=100
ISSUES=()
FIXED=()

log_issue() { ISSUES+=("$1"); }
log_fixed() { FIXED+=("$1"); }

echo "╔══════════════════════════════════════════════════════════╗"
echo "║     CEDAR AUTOFIX — Цикл проверки и исправлений        ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

# ==========================================
# ЦИКЛ 1: СТРУКТУРНАЯ ЦЕЛОСТНОСТЬ (20 баллов)
# ==========================================
echo "=== ЦИКЛ 1: Структурная целостность ==="

CORE_FILES=("_pi.md" "CONCEPT.md" "TODO.md" "PARAMETERS.md" "MAP.md" "STATE.md" "MEMORY.md" "README.md" "DESIGN.md" "THEORY.md" "EVIDENCE.md")
CORE_SCORE=0

for f in "${CORE_FILES[@]}"; do
    if [ -f "$f" ]; then
        SIZE=$(wc -c < "$f")
        if [ "$SIZE" -lt 100 ]; then
            echo "  ⚠️ $f — слишком мал ($SIZE байт)"
            log_issue "$f: too small ($SIZE bytes)"
        elif [ "$SIZE" -lt 500 ] && [ "$f" != "_pi.md" ]; then
            echo "  🟡 $f — маловат ($SIZE байт)"
            log_issue "$f: thin ($SIZE bytes)"
        else
            echo "  ✅ $f ($SIZE байт)"
            ((CORE_SCORE++))
        fi
    else
        echo "  ❌ $f — ОТСУТСТВУЕТ"
        log_issue "$f: MISSING"
    fi
done

CORE_POINTS=$(( CORE_SCORE * 20 / 11 ))
echo "  → Core-файлы: $CORE_SCORE/11 → $CORE_POINTS/20 баллов"
SCORE=$((SCORE + CORE_POINTS))

# ==========================================
# ЦИКЛ 1b: ЧИСТОТА КОРНЯ (10 баллов)
# ==========================================
echo ""
echo "=== ЦИКЛ 1b: Чистота корневого уровня ==="

ROOT_ISSUES=0
# Проверяем каждый файл в корне
for item in $(find . -maxdepth 1 -type f -name "*.md" -o -name "*.sh" -o -name "*.py" -o -name "*.json" -o -name "*.csv" -o -name "*.db" -o -name "*.pid" -o -name "*.heartbeat" 2>/dev/null); do
    BASENAME=$(basename "$item")
    # Пропускаем core-файлы и инфраструктуру
    case "$BASENAME" in
        _pi.md|CONCEPT.md|TODO.md|PARAMETERS.md|MAP.md|STATE.md|MEMORY.md|README.md|DESIGN.md|THEORY.md|EVIDENCE.md|MASTER.md|FILE_MAP.md)
            continue ;;
        Cargo.toml|Cargo.lock|LICENSE|.gitignore|.editorconfig|package.json|Makefile|.zenodo.json|CITATION.cff|rust-toolchain.toml|deny.toml)
            continue ;;
        *)
            echo "  ❌ Подозрительный файл в корне: $BASENAME"
            log_issue "Root-level non-core file: $BASENAME"
            ((ROOT_ISSUES++))
            ;;
    esac
done

if [ $ROOT_ISSUES -eq 0 ]; then
    echo "  ✅ Корень чист — все файлы на своих местах"
fi

ROOT_POINTS=$(( (10 - ROOT_ISSUES * 3) > 0 ? (10 - ROOT_ISSUES * 3) : 0 ))
echo "  → Чистота корня: $ROOT_POINTS/10 баллов"
SCORE=$((SCORE + ROOT_POINTS))

# ==========================================
# ЦИКЛ 2: GIT-СТАТУС (5 баллов)
# ==========================================
echo ""
echo "=== ЦИКЛ 2: Git-статус ==="

UNCOMMITTED=$(git diff --name-only 2>/dev/null | wc -l)
UNPUSHED=$(git log @{u}.. --oneline 2>/dev/null | wc -l)

if [ "$UNCOMMITTED" -gt 0 ]; then
    echo "  🟡 Незакоммиченных файлов: $UNCOMMITTED"
    log_issue "Git: $UNCOMMITTED uncommitted files"
    GIT_SCORE=3
else
    echo "  ✅ Все изменения закоммичены"
    GIT_SCORE=5
fi

if [ "$UNPUSHED" -gt 0 ]; then
    echo "  🟡 Незапушенных коммитов: $UNPUSHED"
    GIT_SCORE=$((GIT_SCORE - 1))
fi

echo "  → Git: $GIT_SCORE/5 баллов"
SCORE=$((SCORE + GIT_SCORE))

# ==========================================
# ЦИКЛ 3: КОНСИСТЕНТНОСТЬ МЕТАДАННЫХ (15 баллов)
# ==========================================
echo ""
echo "=== ЦИКЛ 3: Консистентность метаданных ==="

# Parent-ссылка
PARENT_LINE=$(grep -i "parent" _pi.md 2>/dev/null)
if [ -n "$PARENT_LINE" ]; then
    echo "  ✅ Parent-ссылка в _pi.md: $PARENT_LINE"
    PARENT_SCORE=5
else
    echo "  ❌ Нет parent-ссылки в _pi.md"
    log_issue "_pi.md: missing parent reference"
    PARENT_SCORE=0
fi

# MAP.md vs реальность
# Извлекаем имена папок из ASCII-дерева — ТОЛЬКО верхнего уровня (строки с ├── или └── на первом уровне, заканчивающиеся на /)
# Первый уровень: отступ ровно 0 (начало строки — ├── или └──)
MAP_DIRS=$(grep -E "^[├└]──" MAP.md | grep -oP '(?<=── )[a-zA-Z_][a-zA-Z0-9_\-]*/' | tr -d '/' | sort -u)
REAL_DIRS=$(find . -maxdepth 1 -type d -not -name ".*" -not -name "target" -not -name "node_modules" | sed 's|^\./||' | sort)

MAP_MISMATCH=0
for dir in $MAP_DIRS; do
    if [ ! -d "$dir" ]; then
        echo "  ⚠️ MAP.md содержит '$dir', но папки нет на диске"
        log_issue "MAP.md: '$dir' in map but not on disk"
        ((MAP_MISMATCH++))
    fi
done

for dir in $REAL_DIRS; do
    if ! echo "$MAP_DIRS" | grep -q "$dir"; then
        echo "  ⚠️ Папка '$dir' есть на диске, но отсутствует в MAP.md"
        log_issue "MAP.md: '$dir' on disk but not in map"
        ((MAP_MISMATCH++))
    fi
done

if [ $MAP_MISMATCH -eq 0 ]; then
    echo "  ✅ MAP.md соответствует реальной структуре"
fi

MAP_SCORE=$(( 10 - MAP_MISMATCH * 3 ))
[ $MAP_SCORE -lt 0 ] && MAP_SCORE=0
echo "  → Метаданные: $((PARENT_SCORE + MAP_SCORE))/15 баллов"
SCORE=$((SCORE + PARENT_SCORE + MAP_SCORE))

# ==========================================
# ЦИКЛ 4: АКТУАЛЬНОСТЬ КОНТЕНТА (15 баллов)
# ==========================================
echo ""
echo "=== ЦИКЛ 4: Актуальность контента ==="

# STATE.md age
STATE_DATE=$(grep -oP "2026-\d{2}-\d{2}" STATE.md | head -1)
if [ -n "$STATE_DATE" ]; then
    DAYS_OLD=$(( ($(date +%s) - $(date -d "$STATE_DATE" +%s)) / 86400 ))
    if [ $DAYS_OLD -le 30 ]; then
        echo "  ✅ STATE.md обновлён $STATE_DATE ($DAYS_OLD дн. назад)"
        STATE_SCORE=5
    else
        echo "  ⚠️ STATE.md устарел: $STATE_DATE ($DAYS_OLD дн. назад)"
        log_issue "STATE.md: last updated $DAYS_OLD days ago"
        STATE_SCORE=2
    fi
else
    echo "  ❌ Не найдена дата в STATE.md"
    STATE_SCORE=0
fi

# TODO.md check
TODO_TOTAL=$(grep -c "\[ \]" TODO.md 2>/dev/null)
TODO_DONE=$(grep -c "\[x\]" TODO.md 2>/dev/null)
if [ "$TODO_TOTAL" -gt 0 ]; then
    echo "  ✅ TODO.md: $TODO_DONE/$((TODO_TOTAL + TODO_DONE)) задач выполнено"
    TODO_SCORE=5
else
    echo "  ⚠️ TODO.md: все задачи выполнены или список пуст"
    log_issue "TODO.md: no active tasks"
    TODO_SCORE=2
fi

# MEMORY.md last entry
LAST_MEM=$(grep -oP "2026-\d{2}-\d{2}" MEMORY.md | head -1)
if [ -n "$LAST_MEM" ]; then
    MEM_DAYS=$(( ($(date +%s) - $(date -d "$LAST_MEM" +%s)) / 86400 ))
    if [ $MEM_DAYS -le 60 ]; then
        echo "  ✅ MEMORY.md активен ($LAST_MEM, $MEM_DAYS дн. назад)"
        MEM_SCORE=5
    else
        echo "  ⚠️ MEMORY.md не обновлялся $MEM_DAYS дней"
        MEM_SCORE=2
    fi
else
    MEM_SCORE=2
fi

CONTENT_SCORE=$((STATE_SCORE + TODO_SCORE + MEM_SCORE))
echo "  → Актуальность: $CONTENT_SCORE/15 баллов"
SCORE=$((SCORE + CONTENT_SCORE))

# ==========================================
# ЦИКЛ 5: КАЧЕСТВО КОНТЕНТА (20 баллов)
# ==========================================
echo ""
echo "=== ЦИКЛ 5: Качество контента ==="

# PARAMETERS.md sanity check
PARAM_SIZE=$(wc -c < PARAMETERS.md)
PARAM_CONTENT=$(head -20 PARAMETERS.md)
# Проверка на шаблонный мусор и качество контента
if echo "$PARAM_CONTENT" | grep -qi "data transformation\|validation framework\|cloud-native\|declarative schema\|Apache Kafka\|AWS Kinesis"; then
    echo "  🔴 PARAMETERS.md содержит ШАБЛОННЫЙ ТЕКСТ (не центриолярные параметры!)"
    log_issue "PARAMETERS.md: wrong content (data validation template, not centriole params)"
    PARAM_SCORE=0
elif echo "$PARAM_CONTENT" | grep -qi "centriol\|damage\|aging\|HSC\|division rate\|Sobol\|LLPS\|centrosom"; then
    echo "  ✅ PARAMETERS.md: $PARAM_SIZE байт (центриолярные параметры)"
    PARAM_SCORE=5
elif [ "$PARAM_SIZE" -lt 500 ]; then
    echo "  🟡 PARAMETERS.md маловат ($PARAM_SIZE байт)"
    PARAM_SCORE=2
else
    echo "  🟡 PARAMETERS.md: $PARAM_SIZE байт (содержание под вопросом)"
    PARAM_SCORE=3
fi

# CONCEPT.md completeness
CONCEPT_SECTIONS=0
grep -qi "what is\|essence\|что это" CONCEPT.md && ((CONCEPT_SECTIONS++))
grep -qi "purpose\|purpose\|зачем" CONCEPT.md && ((CONCEPT_SECTIONS++))
grep -qi "how it works\|как работает\|mechanism" CONCEPT.md && ((CONCEPT_SECTIONS++))
grep -qi "status\|статус" CONCEPT.md && ((CONCEPT_SECTIONS++))
echo "  → CONCEPT.md: $CONCEPT_SECTIONS/4 ключевых секций"
CONCEPT_SCORE=$((CONCEPT_SECTIONS * 3))

# DESIGN.md
DESIGN_SIZE=$(wc -c < DESIGN.md)
if [ "$DESIGN_SIZE" -lt 500 ]; then
    echo "  🟡 DESIGN.md тонкий ($DESIGN_SIZE байт)"
    DESIGN_SCORE=1
else
    DESIGN_SCORE=3
fi

# THEORY.md
THEORY_SIZE=$(wc -c < THEORY.md)
if [ "$THEORY_SIZE" -gt 5000 ]; then
    echo "  ✅ THEORY.md: $THEORY_SIZE байт (хорошо)"
    THEORY_SCORE=3
else
    THEORY_SCORE=2
fi

QUALITY_SCORE=$((PARAM_SCORE + CONCEPT_SCORE + DESIGN_SCORE + THEORY_SCORE))
echo "  → Качество: $QUALITY_SCORE/20 баллов"
SCORE=$((SCORE + QUALITY_SCORE))

# ==========================================
# ЦИКЛ 6: МЕЖПРОЕКТНЫЕ СВЯЗИ (10 баллов)
# ==========================================
echo ""
echo "=== ЦИКЛ 6: Межпроектные связи ==="

# Проверка parent-директории
PARENT_DIR=$(dirname $(dirname "$PROJECT_ROOT"))
if [ -d "$PARENT_DIR" ]; then
    echo "  ✅ Родительская директория существует: $PARENT_DIR"
    LINK_SCORE=5
else
    echo "  ❌ Родительская директория не найдена"
    LINK_SCORE=0
fi

# Проверка ссылок на дочерние проекты
CHILD_SCORE=5
for child in "simulator" "Aubrey-Platform" "CellLineageTree" "articles"; do
    if [ -d "$child" ] && [ -f "$child/_pi.md" ]; then
        echo "  ✅ $child/ (_pi.md присутствует)"
    elif [ -d "$child" ]; then
        echo "  🟡 $child/ (без _pi.md)"
        ((CHILD_SCORE--))
    fi
done

LINK_TOTAL=$((LINK_SCORE + CHILD_SCORE))
echo "  → Связи: $LINK_TOTAL/10 баллов"
SCORE=$((SCORE + LINK_TOTAL))

# ==========================================
# ЦИКЛ 9: ГЛУБИННАЯ ПРОВЕРКА (5 баллов)
# ==========================================
echo ""
echo "=== ЦИКЛ 9: Глубинная проверка содержимого ==="

# Проверка на файлы не в своих папках
MISPLACED=0
# .md файлы в scripts/
MD_IN_SCRIPTS=$(find scripts/ -name "*.md" 2>/dev/null | wc -l)
if [ "$MD_IN_SCRIPTS" -gt 0 ]; then
    echo "  ⚠️ .md файлы в scripts/: $MD_IN_SCRIPTS"
    ((MISPLACED++))
fi
# .sh файлы в docs/
SH_IN_DOCS=$(find docs/ -name "*.sh" 2>/dev/null | wc -l)
if [ "$SH_IN_DOCS" -gt 0 ]; then
    echo "  ⚠️ .sh файлы в docs/: $SH_IN_DOCS"
    ((MISPLACED++))
fi
# .json данные в корне (не конфиги)
JSON_IN_ROOT=$(find . -maxdepth 1 -name "*.json" ! -name ".zenodo.json" ! -name "package.json" 2>/dev/null | wc -l)
if [ "$JSON_IN_ROOT" -gt 0 ]; then
    echo "  ⚠️ .json данные в корне: $JSON_IN_ROOT"
    ((MISPLACED++))
fi

if [ $MISPLACED -eq 0 ]; then
    echo "  ✅ Все файлы в правильных папках"
fi

DEEP_SCORE=$((5 - MISPLACED * 2))
[ $DEEP_SCORE -lt 0 ] && DEEP_SCORE=0
echo "  → Глубинная проверка: $DEEP_SCORE/5 баллов"
SCORE=$((SCORE + DEEP_SCORE))

# ==========================================
# ИТОГО
# ==========================================
echo ""
echo "╔══════════════════════════════════════════════════════════╗"
echo "║  ИТОГО: $SCORE / $MAX_SCORE баллов                          ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

# Вывод проблем
if [ ${#ISSUES[@]} -gt 0 ]; then
    echo "🔴 Найдено проблем: ${#ISSUES[@]}"
    for issue in "${ISSUES[@]}"; do
        echo "  • $issue"
    done
fi

if [ ${#FIXED[@]} -gt 0 ]; then
    echo ""
    echo "🟢 Исправлено: ${#FIXED[@]}"
    for fix in "${FIXED[@]}"; do
        echo "  ✓ $fix"
    done
fi

echo ""
echo "Критерий выхода: 95/100"
if [ $SCORE -ge 95 ]; then
    echo "✅ ПРОЙДЕНО!"
else
    echo "🔴 Требуется ещё цикл autofix (не хватает $((95 - SCORE)) баллов)"
fi

# Возвращаем score как exit code
exit $((100 - SCORE))
