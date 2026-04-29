#!/usr/bin/env python3
"""
Ze Theory v2 → v3 pipeline.

Steps:
  1. Apply ALL recommendations from peer_review_v2 to manuscript v2 → v3 markdown
  2. Extract bibliography → run citation reality checks
  3. Meta-analysis of 6 empirical pillars (Ch. 25)
  4. Super-strict peer review for IF 18+ journals (Russian)

All LLM calls go through ~/Desktop/AIM/llm.py (DeepSeek V4).
Outputs in ~/Desktop/LongevityCommon/Ze/docs/.
"""
from __future__ import annotations

import os
import re
import sys
import time
import json
import logging
from pathlib import Path

sys.path.insert(0, str(Path.home() / "Desktop" / "AIM"))

# load DEEPSEEK_API_KEY from ~/.aim_env if not in env
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
log = logging.getLogger("ze.v3")

ROOT = Path.home() / "Desktop"
DOCS = ROOT / "LongevityCommon" / "Ze" / "docs"
DOCS.mkdir(parents=True, exist_ok=True)

V2_PATH = ROOT / "Ze_Theory_v2_2026-04-29.md"
REV_V2_PATH = DOCS / "peer_review_v2_2026-04-29.md"

V3_PATH = ROOT / "Ze_Theory_v3_2026-04-29.md"
CIT_REPORT = DOCS / "citation_verification_v3_2026-04-29.md"
META_REPORT = DOCS / "meta_analysis_pillars_2026-04-29.md"
REV_V3_PATH = DOCS / "peer_review_v3_2026-04-29.md"


def _read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def _write(path: Path, body: str) -> None:
    path.write_text(body, encoding="utf-8")
    log.info("wrote %s (%d bytes)", path, len(body))


# ── Step 1: apply edits → v3 ───────────────────────────────────────────────


SYSTEM_EDITOR = (
    "Ты — научный редактор, специализирующийся на физике, квантовой "
    "информации и теоретической биологии. Получив рукопись v2 и подробное "
    "рецензирование v2, ты создаёшь рукопись v3 — точную интеграцию ВСЕХ "
    "рекомендаций рецензента БЕЗ изменения научного смысла теории. "
    "Сохраняй структуру глав, формулы, обозначения. Удаляй непроверенные "
    "ссылки, явно помечай постулаты, добавляй размерности, исправляй "
    "статистические утверждения, уточняй интерпретации экспериментальных "
    "оснований. Возвращай ТОЛЬКО полный текст v3 без преамбул и комментариев."
)


def step1_apply_edits() -> None:
    if V3_PATH.exists() and V3_PATH.stat().st_size > 5_000:
        log.info("step1: skipping — %s already exists", V3_PATH)
        return
    log.info("step1: applying v2 review edits → v3")
    v2 = _read(V2_PATH)
    rv = _read(REV_V2_PATH)
    prompt = (
        "## Рукопись v2\n\n"
        f"{v2}\n\n"
        "## Сверхстрогая рецензия v2 (рекомендации к применению)\n\n"
        f"{rv}\n\n"
        "## Задача\n"
        "Создай рукопись v3, применив ВСЕ рекомендации рецензента "
        "(включая разделы 6.1, 6.2 и 8.1), не меняя научного смысла теории. "
        "Конкретно:\n"
        "1. Удали ссылки, не прошедшие верификацию: Miller (2025) arXiv:2511.01988, "
        "Sienicki (2025) arXiv:2502.08653, Wauthier et al. (2022) arXiv:2208.08713, "
        "Ryu et al. (2018) arXiv:1812.01494. Если они использовались как опоры — "
        "переформулируй текст так, чтобы аргумент остался, но без этих ссылок.\n"
        "2. dτ_Ze/dt = -αI(Z): явно пометь как постулат (Постулат P1) или дай вывод "
        "из стохастической термодинамики со ссылкой на Pearson et al. 2021.\n"
        "3. Добавь размерности к ВСЕМ ключевым уравнениям.\n"
        "4. В Главе 10 (космология): убери γ_Ze ≈ -0.031 ± 0.007 как "
        "'результат MCMC', либо чётко пометь это как теоретическое предсказание, "
        "ожидающее проверки. Добавь раздел про требуемую процедуру: "
        "χ²/DoF, Bayesian evidence vs ΛCDM, public MCMC code.\n"
        "5. Cheating rate (раздел 13.5): дай операциональное определение с "
        "единицами измерения и предложи минимально-измеримый протокол.\n"
        "6. Глава 25, столпы: явно различай 'прямое экспериментальное "
        "подтверждение' (только Pearson 2021) от 'теоретической интерпретации "
        "результата' (Abboud, Woodhead, Gassner, Proietti).\n"
        "7. О журнале Longevity Horizon (раздел 8.1): убери упоминание DOI "
        "10.65649 либо дай полные данные о регистрации префикса в CrossRef.\n"
        "Возвращай только итоговый markdown v3."
    )
    out = ask_long(prompt, system=SYSTEM_EDITOR, lang="ru")
    _write(V3_PATH, out)


# ── Step 2: extract bibliography ───────────────────────────────────────────


def _extract_citations(text: str) -> list[str]:
    pats = [
        r"arXiv:\s*\d{4}\.\d{4,5}",
        r"PMID[:\s]*\d{6,9}",
        r"doi[:\s]*10\.\d{3,9}/[^\s\)]+",
        r"10\.\d{3,9}/[^\s\)]+",
        r"\bPhys\.\s*Rev\.\s*[A-Z]?\s*\d+\s*,\s*\d+",
        r"\bNature\s+\d+",
        r"\bScience\s+\d+",
        r"\b[A-Z][a-zé]+(?:\s+(?:et\s+al\.?|&|and)\s+[A-Z][a-zé]+)?\s*\(\d{4}[a-z]?\)",
    ]
    seen: set[str] = set()
    out: list[str] = []
    for p in pats:
        for m in re.findall(p, text, flags=re.I):
            key = m.strip()
            if key.lower() not in seen:
                seen.add(key.lower())
                out.append(key)
    return out


def step2_citations() -> None:
    if CIT_REPORT.exists() and CIT_REPORT.stat().st_size > 5_000:
        log.info("step2: skipping — %s exists", CIT_REPORT)
        return
    log.info("step2: citation verification")
    v3 = _read(V3_PATH)
    cites = _extract_citations(v3)
    log.info("extracted %d citation tokens", len(cites))
    bib_section = re.search(
        r"(?:## *(?:Литература|References|Библиография).*?)(?=\n## |\Z)",
        v3, flags=re.I | re.S,
    )
    bib_text = bib_section.group(0) if bib_section else ""
    prompt = (
        "Перед тобой список цитат-токенов и (если найден) раздел литературы из "
        "рукописи v3. Для каждой ссылки определи:\n"
        "(a) реальна ли публикация (arXiv, PubMed, CrossRef, Semantic Scholar, "
        "INSPIRE-HEP, NASA ADS — оперируй своим знанием);\n"
        "(b) соответствует ли заявленный сюжет реальному содержанию;\n"
        "(c) уровень достоверности: VERIFIED / PROBABLE / UNVERIFIED / "
        "INCONSISTENT / FABRICATED.\n"
        "ВАЖНО: если ID arXiv в формате 'YYMM.nnnnn' имеет YY > 25 (т.е. "
        "2026+), помечай как 'требует прямой проверки в arXiv listing — "
        "может быть валидным или несуществующим'.\n"
        "Формат вывода: markdown-таблица + блок с резюме (сколько подтверждено, "
        "сколько отвергнуто) + рекомендации, какие ссылки удалить из v3.\n\n"
        "## Цитаты-токены (извлечены автоматически)\n\n"
        + "\n".join(f"- {c}" for c in cites)
        + "\n\n## Раздел литературы из v3\n\n"
        + (bib_text if bib_text else "(не обнаружен в виде явной секции)\n")
    )
    out = ask_deep(prompt, system="Ты — научный библиограф высочайшего уровня, "
                   "верифицирующий цитаты для журналов с импакт-фактором >18.",
                   lang="ru")
    _write(CIT_REPORT, out)


# ── Step 3: meta-analysis ──────────────────────────────────────────────────


def step3_meta() -> None:
    if META_REPORT.exists() and META_REPORT.stat().st_size > 5_000:
        log.info("step3: skipping — %s exists", META_REPORT)
        return
    log.info("step3: meta-analysis of 6 empirical pillars")
    v3 = _read(V3_PATH)
    prompt = (
        "В Главе 25 рукописи v3 заявлены ШЕСТЬ эмпирических столпов "
        "Ze Theory:\n"
        "  25.1 I = ⟨ΔS⟩_gen (Burgholzer 2015)\n"
        "  25.2 Термодинамическая цена хронометража (Pearson et al. 2021, "
        "Phys. Rev. X 11, 021029)\n"
        "  25.3 CHSH-деформация (Gassner et al. 2021)\n"
        "  25.4 LGI-QFI (Abboud et al. 2026)\n"
        "  25.5 Asymmetric CHSH (Woodhead et al. 2021)\n"
        "  25.6 Wigner's friend (Proietti et al. 2019)\n\n"
        "Проведи МЕТА-АНАЛИЗ каждого столпа по схеме:\n"
        "(1) первичная публикация: дизайн, выборка, эффект-сайз, p-value/CI;\n"
        "(2) независимые воспроизведения (с указанием DOI/PMID/arXiv, если "
        "доступны в твоей памяти);\n"
        "(3) согласованность результатов между лабораториями;\n"
        "(4) выявленные систематические ошибки/критика;\n"
        "(5) насколько столп РЕАЛЬНО поддерживает Ze Theory vs является "
        "переинтерпретацией существующего результата;\n"
        "(6) уровень достоверности: A (multi-lab replicated) / "
        "B (single-lab solid) / C (theoretical prediction tested) / "
        "D (interpretation only).\n\n"
        "Заверши сводной таблицей и общим выводом: какие столпы образуют "
        "реальное эмпирическое основание, а какие — теоретическая надстройка.\n\n"
        "## v3 (для контекста)\n\n"
        + v3[:80_000]  # cap for safety
    )
    out = ask_deep(prompt, system="Ты — методолог meta-analysis "
                   "(уровень Cochrane / PRISMA) для физики и квантовой "
                   "информации.", lang="ru")
    _write(META_REPORT, out)


# ── Step 4: super-strict peer review ───────────────────────────────────────


def step4_peer_review() -> None:
    if REV_V3_PATH.exists() and REV_V3_PATH.stat().st_size > 5_000:
        log.info("step4: skipping — %s exists", REV_V3_PATH)
        return
    log.info("step4: super-strict v3 peer review for IF 18+")
    v3 = _read(V3_PATH)
    cit_rep = _read(CIT_REPORT) if CIT_REPORT.exists() else ""
    meta_rep = _read(META_REPORT) if META_REPORT.exists() else ""
    prompt = (
        "Ты — экспертная группа из 3 ведущих рецензентов журналов класса "
        "Nature, Physical Review X, Reviews of Modern Physics. Импакт-фактор "
        "целевого журнала ≥18. Рецензирование тройное слепое, СВЕРХСТРОГОЕ.\n\n"
        "Твоя задача: дать максимально жёсткий, технически глубокий, "
        "конструктивный отзыв на рукопись v3 'Ze Theory: Entropic-Geometric "
        "Theory of Everything'. Используй приложенные результаты "
        "верификации цитат и мета-анализ эмпирических столпов как "
        "проверенный фактический бэкграунд.\n\n"
        "Структура отзыва (на русском, для редактора и автора):\n"
        "1. Executive summary с явной оценкой (Accept / Minor / Major / Reject) "
        "для целевого журнала с IF ≥18.\n"
        "2. Оценка новизны: что РЕАЛЬНО ново в Ze Theory сверх FEP "
        "(Friston), IIT (Tononi), GNW (Dehaene), стохастической термо-"
        "динамики, информационной геометрии. Если новизна отсутствует — "
        "указать честно.\n"
        "3. Математическая строгость: пройдись по ВСЕМ ключевым уравнениям "
        "(Постулат P1, метрика, CHSH-деформация, γ_Ze, ц.д. потребления "
        "proper-time). Размерности, выводимость, граничные условия.\n"
        "4. Эмпирическая верифицируемость: используй мета-анализ; для "
        "каждого предсказания дай falsifiability score 0–10 и предложи "
        "конкретный эксперимент с оценкой стоимости и времени.\n"
        "5. Цитирование: используй приложенный отчёт. Перечисли все "
        "non-VERIFIED ссылки.\n"
        "6. Сравнение с альтернативными теориями (Wheeler-DeWitt, "
        "Verlinde entropic gravity, Penrose OR, ER=EPR) — даёт ли Ze Theory "
        "что-то новое или повторяет известные предложения.\n"
        "7. Этика и прозрачность: проблема DOI 10.65649 / Longevity Horizon; "
        "self-citation rate; conflict of interest.\n"
        "8. Список MUST-FIX до повторной подачи (нумерованный).\n"
        "9. Список SHOULD-FIX (важно, но не блокирующее).\n"
        "10. Альтернативные журналы, если IF≥18 недостижим: "
        "Foundations of Physics, Entropy, Quantum, JHEP — с обоснованием.\n"
        "11. Заключительный вердикт.\n\n"
        "ТРЕБОВАНИЯ к стилю: жёсткий академический русский, без воды, "
        "без эпитетов 'интересно/амбициозно'. Технические термины в "
        "оригинале или с пояснением. Никакого подхалимства автору.\n\n"
        "## Рукопись v3\n\n"
        f"{v3}\n\n"
        "## Отчёт верификации цитат\n\n"
        f"{cit_rep}\n\n"
        "## Мета-анализ эмпирических столпов\n\n"
        f"{meta_rep}\n"
    )
    out = ask_deep(
        prompt,
        system="Ты — старший рецензент уровня Nature/PRX. Не делаешь скидок.",
        lang="ru",
    )
    _write(REV_V3_PATH, out)


def main() -> None:
    t0 = time.time()
    step1_apply_edits()
    step2_citations()
    step3_meta()
    step4_peer_review()
    log.info("pipeline done in %.1fs", time.time() - t0)
    log.info("outputs:")
    for p in (V3_PATH, CIT_REPORT, META_REPORT, REV_V3_PATH):
        sz = p.stat().st_size if p.exists() else 0
        log.info("  %s (%d bytes)", p, sz)


if __name__ == "__main__":
    main()
