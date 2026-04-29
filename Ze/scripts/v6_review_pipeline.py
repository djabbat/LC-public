#!/usr/bin/env python3
"""
Ze Theory v6 review pipeline (3 steps, NO new manuscript generation).

User explicitly placed v6 on Desktop. To avoid yet another round of LLM
hallucinated citations (the v5 pipeline added 4 fabricated refs while
trying to "expand" the empirical base), this pipeline DOES NOT produce
v7 manuscript. Instead it:

  Step 1: verify every citation in v6 against arXiv/PubMed/CrossRef/etc.
  Step 2: Cochrane-style meta-analysis of empirical claims in v6.
  Step 3: super-strict peer review (Russian, IF≥18) of v6 AS IS.

Outputs:
  ~/Desktop/LongevityCommon/Ze/docs/citation_verification_v6_2026-04-29.md
  ~/Desktop/LongevityCommon/Ze/docs/meta_analysis_v6_2026-04-29.md
  ~/Desktop/LongevityCommon/Ze/docs/peer_review_v6_2026-04-29.md
"""
from __future__ import annotations
import os, re, sys, time, logging
from pathlib import Path

sys.path.insert(0, str(Path.home() / "Desktop" / "AIM"))
if not os.environ.get("DEEPSEEK_API_KEY"):
    env = Path.home() / ".aim_env"
    if env.exists():
        for line in env.read_text().splitlines():
            line = line.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            k, _, v = line.partition("=")
            os.environ.setdefault(k.strip(), v.strip())
from llm import ask_long, ask_deep  # noqa: E402

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s %(levelname)s %(message)s",
    datefmt="%H:%M:%S",
)
log = logging.getLogger("ze.v6")

ROOT = Path.home() / "Desktop"
DOCS = ROOT / "LongevityCommon" / "Ze" / "docs"
DOCS.mkdir(parents=True, exist_ok=True)

V6_PATH = ROOT / "Ze_v6.md"
CIT = DOCS / "citation_verification_v6_2026-04-29.md"
META = DOCS / "meta_analysis_v6_2026-04-29.md"
REV = DOCS / "peer_review_v6_2026-04-29.md"


def _read(p): return p.read_text(encoding="utf-8")


def _write(p, body):
    p.write_text(body, encoding="utf-8")
    log.info("wrote %s (%d bytes)", p, len(body))


def _extract(text):
    pats = [
        r"arXiv:\s*\d{4}\.\d{4,5}",
        r"PMID[:\s]*\d{6,9}",
        r"doi[:\s]*10\.\d{3,9}/[^\s\)\]]+",
        r"10\.\d{3,9}/[^\s\)\]]+",
        r"\bPhys\.\s*Rev\.\s*[A-Z]?\s*\d+\s*,\s*\d+",
        r"\bNature(?:\s+Physics)?\s+\d+",
        r"\bScience\s+\d+",
        r"\bPNAS\s+\d+",
        r"\b[A-Z][a-zé]+(?:\s+(?:et\s+al\.?|&|and)\s+[A-Z][a-zé]+)?\s*\(\d{4}[a-z]?\)",
    ]
    seen, out = set(), []
    for p in pats:
        for m in re.findall(p, text, flags=re.I):
            k = m.strip()
            if k.lower() not in seen:
                seen.add(k.lower()); out.append(k)
    return out


def step1_cit():
    if CIT.exists() and CIT.stat().st_size > 5_000:
        return
    log.info("step1: citation verification")
    v6 = _read(V6_PATH)
    cites = _extract(v6)
    bib = re.search(
        r"(?:## *(?:Литература|References|Библиография|Bibliography).*?)(?=\n## |\Z)",
        v6, flags=re.I | re.S)
    bib_text = bib.group(0) if bib else ""
    prompt = (
        "Перед тобой ВСЯ статья Ze Theory v6 (235 строк). Верифицируй "
        "КАЖДУЮ цитату по схеме (a)-реальна-ли, (b)-совпадает-ли-сюжет, "
        "(c)-VERIFIED/PROBABLE/UNVERIFIED/INCONSISTENT/FABRICATED.\n\n"
        "ВАЖНО: arXiv ID формата YYMM.nnnnn с YY > 25 (т.е. с 2026+) — "
        "ПОМЕЧАЙ как FABRICATED, поскольку на момент рецензирования (май "
        "2025) такие препринты ещё не могли быть присвоены.\n\n"
        "Также проверь ВСЕ DOI-префиксы на регистрацию в CrossRef. "
        "Префиксы 10.65649, 10.105099 и подобные несуществующие — "
        "FABRICATED.\n\n"
        "Формат: markdown-таблица, резюме-статистика, рекомендации "
        "по правкам.\n\n"
        "## Полный текст v6\n\n"
        + v6
        + "\n\n## Извлечённые токены\n\n"
        + "\n".join(f"- {c}" for c in cites)
        + "\n\n## Раздел литературы\n\n"
        + (bib_text or "(не выделена)\n")
    )
    out = ask_deep(
        prompt,
        system="Ты — научный библиограф высочайшего уровня (Nature/PRX). "
               "Не делаешь скидок. Не доверяешь автору на слово.",
        lang="ru",
    )
    _write(CIT, out)


def step2_meta():
    if META.exists() and META.stat().st_size > 5_000:
        return
    log.info("step2: meta-analysis")
    v6 = _read(V6_PATH)
    cit = _read(CIT) if CIT.exists() else ""
    prompt = (
        "Проведи МЕТА-АНАЛИЗ всех экспериментов, цитируемых в v6 как "
        "эмпирическое подтверждение Ze Theory.\n\n"
        "Для каждого: (1) первичная публикация — дизайн, выборка, эффект, "
        "p/CI; (2) независимые воспроизведения с DOI; (3) "
        "согласованность multi-lab; (4) систематические ошибки/критика; "
        "(5) РЕАЛЬНО ли подтверждает Ze Theory как уникальную теорию vs "
        "стандартную стохастическую термодинамику (Crooks/Jarzynski/"
        "Landauer/Sagawa-Ueda); (6) уровень: A (multi-lab + unique Ze "
        "signature) / B (single-lab + unique Ze) / C (multi-lab, "
        "consistent with standard thermo) / D (interpretation only).\n\n"
        "Forest-plot-style сводная таблица с эффект-сайзами и CI. "
        "Чёткое разделение 'unique Ze signature' vs 'consistent with "
        "standard thermo'.\n\n"
        "Если в citation report какие-то ссылки помечены FABRICATED — "
        "ИСКЛЮЧИ их из мета-анализа и явно перечисли отдельно.\n\n"
        "## v6\n\n" + v6
        + "\n\n## Citation verification (для контекста)\n\n" + cit
    )
    out = ask_deep(
        prompt,
        system="Методолог Cochrane/PRISMA для квантовой информации и "
               "стохастической термодинамики.",
        lang="ru",
    )
    _write(META, out)


def step3_rev():
    if REV.exists() and REV.stat().st_size > 5_000:
        return
    log.info("step3: super-strict review")
    v6 = _read(V6_PATH)
    cit = _read(CIT) if CIT.exists() else ""
    meta = _read(META) if META.exists() else ""
    prompt = (
        "Ты — экспертная панель из 3 ведущих рецензентов журналов "
        "класса Nature, Physical Review X, Reviews of Modern Physics, "
        "Nature Physics. Импакт-фактор целевого журнала ≥18. Тройное "
        "слепое рецензирование рукописи v6 'Ze Theory'.\n\n"
        "Рукопись v6 короче предыдущих версий (235 строк, "
        "позиционируется автором как теоретическое предложение для "
        "Foundations of Physics / Entropy). Твоя задача — оценить v6 "
        "СТРОГО по критериям IF≥18, даже если автор сам не претендует "
        "на эту планку. Это сверхстрогое рецензирование.\n\n"
        "Структура отзыва (русский, ВАЖНО):\n"
        "1. Executive summary с явной оценкой Accept/Minor/Major/Reject "
        "для журналов с IF≥18, и отдельно — для Foundations of Physics "
        "/ Entropy (IF~1.5-2.5) как реалистичная альтернатива.\n"
        "2. Новизна сверх FEP (Friston), IIT (Tononi), GNW (Dehaene), "
        "стохастической термодинамики, информационной геометрии. Что "
        "РЕАЛЬНО ново vs переименование известных идей.\n"
        "3. Математическая строгость каждого ключевого уравнения "
        "(импеданс, постулат P1 о времени, постулат P4 о вероятностях, "
        "CHSH-деформация, метрика). Размерности, выводимость, граничные "
        "условия.\n"
        "4. Эмпирическая верифицируемость: используй мета-анализ. Для "
        "КАЖДОГО предсказания дай falsifiability score 0-10 + конкретный "
        "эксперимент с оценкой стоимости и времени.\n"
        "5. Цитирование: используй citation report. Перечисли non-"
        "VERIFIED ссылки.\n"
        "6. Сравнение с альтернативами: Wheeler-DeWitt, Verlinde "
        "entropic gravity, Penrose OR, ER=EPR, Pearson/Erker thermo-"
        "dynamic clocks.\n"
        "7. Этика, прозрачность, COI (особенно: журнал Longevity "
        "Horizon, DOI-префикс 10.65649, self-citation).\n"
        "8. MUST-FIX (нумерованный список) для подачи в IF≥18.\n"
        "9. SHOULD-FIX (важно, не блокирующее).\n"
        "10. Альтернативные журналы если IF≥18 не достижим.\n"
        "11. Заключительный вердикт.\n\n"
        "СТИЛЬ: жёсткий академический русский, без воды, без эпитетов "
        "'интересно/амбициозно'. Без подхалимства автору. Технические "
        "термины с пояснением. Выводы — категоричные.\n\n"
        "## Рукопись v6\n\n" + v6
        + "\n\n## Citation verification report\n\n" + cit
        + "\n\n## Meta-analysis report\n\n" + meta
    )
    out = ask_deep(
        prompt,
        system="Старший рецензент Nature/PRX. Не делаешь скидок. "
               "Пишешь по-русски, технически, жёстко.",
        lang="ru",
    )
    _write(REV, out)


def main():
    t0 = time.time()
    step1_cit()
    step2_meta()
    step3_rev()
    log.info("done in %.1fs", time.time() - t0)
    for p in (CIT, META, REV):
        sz = p.stat().st_size if p.exists() else 0
        log.info("  %s (%d bytes)", p, sz)


if __name__ == "__main__":
    main()
