#!/usr/bin/env python3
"""
Ze Theory v3 → v5 pipeline.

Solves the THREE fundamental problems from v4 peer review:
  P1. Circular logic — formal proof of existence/uniqueness of the
      self-consistent dynamical system.
  P2. Derivation of key equations — derive Postulate P1 and Born-like
      Postulate P4 from stochastic-thermodynamic first principles
      (Pearson 2021, Erker 2017, Crooks 1999, Jarzynski 1997).
  P3. Narrow empirical base — broad literature scan to expand from 1 →
      ≥5 directly supporting experiments.

Steps:
  1. Apply ALL v4 review edits + structural overhaul → v5 manuscript
  2. Citation verification (broadened, with Crossref-style scan)
  3. Meta-analysis with EXPANDED empirical pillars
  4. New super-strict peer review for IF≥18 (Russian)

Outputs to Desktop and to docs/.
"""
from __future__ import annotations

import os
import re
import sys
import time
import logging
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
log = logging.getLogger("ze.v5")

ROOT = Path.home() / "Desktop"
DOCS = ROOT / "LongevityCommon" / "Ze" / "docs"
DOCS.mkdir(parents=True, exist_ok=True)

V3_PATH = ROOT / "Ze_Theory_v3_2026-04-29.md"
REV_V4_PATH = DOCS / "peer_review_v4_2026-04-29.md"

V5_PATH = ROOT / "Ze_Theory_v5_2026-04-29.md"
CIT_REPORT = DOCS / "citation_verification_v5_2026-04-29.md"
META_REPORT = DOCS / "meta_analysis_v5_2026-04-29.md"
REV_V5_PATH = DOCS / "peer_review_v5_2026-04-29.md"


def _read(p: Path) -> str:
    return p.read_text(encoding="utf-8")


def _write(p: Path, body: str) -> None:
    p.write_text(body, encoding="utf-8")
    log.info("wrote %s (%d bytes)", p, len(body))


# ── Step 1: produce v5 with three problems solved ──────────────────────────


SYSTEM_AUTHOR = (
    "Ты — главный научный редактор и теоретический физик уровня "
    "Reviews of Modern Physics. Твоя задача — переработать рукопись "
    "Ze Theory v3 так, чтобы устранить ТРИ фундаментальные претензии "
    "v4-рецензента: (P1) циклическая логика, (P2) отсутствие вывода "
    "ключевых уравнений из первых принципов, (P3) узкая эмпирическая "
    "база. Сохрани научный смысл теории, но добавь математическую "
    "строгость и расширь эмпирическое подтверждение через цитирование "
    "реальных экспериментов. Возвращай ТОЛЬКО полный текст v5 в markdown "
    "без преамбул."
)


def step1_v5() -> None:
    if V5_PATH.exists() and V5_PATH.stat().st_size > 5_000:
        log.info("step1: skipping — %s exists", V5_PATH)
        return
    log.info("step1: producing v5 (solve P1, P2, P3)")
    v3 = _read(V3_PATH)
    rv4 = _read(REV_V4_PATH)
    prompt = (
        "## Рукопись v3 (исходник)\n\n"
        f"{v3}\n\n"
        "## Сверхстрогая рецензия v4\n\n"
        f"{rv4}\n\n"
        "## Задача — создать v5 с РЕШЕНИЕМ трёх фундаментальных проблем\n\n"
        "**P1. Циклическая логика.** Добавь новую главу 'Mathematical "
        "Foundations: Self-Consistent System' где:\n"
        "  • Сформулируй систему уравнений (1)–(3) как fixed-point "
        "проблему в банаховом пространстве траекторий.\n"
        "  • Докажи существование и единственность решения по теореме "
        "Banach fixed-point с явным условием Lipschitz: |α·dI/dt| < L.\n"
        "  • Приведи численный пример сходимости итераций "
        "I_{n+1} = F[I_n] для модельного случая (одиночный нано-"
        "осциллятор).\n"
        "  • Чётко разграничь: что — определение, что — постулат, что — "
        "следствие.\n\n"
        "**P2. Вывод ключевых уравнений из первых принципов.**\n"
        "  • Выведи Постулат P1 (dτ_Ze/dt = -αI) из теоремы Кроокса "
        "(Crooks 1999) + неравенства Ландауэра (Landauer 1961, "
        "verified by Bérut et al. Nature 2012, doi:10.1038/nature10872).\n"
        "  • Покажи, что α = k_B T·ln 2 / ⟨ΔS⟩ совпадает с измеренным "
        "значением Pearson 2021.\n"
        "  • Постулат Борна-подобного P4 выведи как large-deviation "
        "limit (Touchette 2009, Phys. Rep. 478) дискретной марковской "
        "цепи на пространстве состояний с относительной энтропией как "
        "rate function.\n"
        "  • CHSH-деформацию выведи через информационно-геометрическую "
        "поправку Bogoliubov-Kubo-Mori (Petz 1996; Naudts 2004).\n\n"
        "**P3. Расширение эмпирической базы — ДОБАВЬ как минимум 5 "
        "новых независимых экспериментов** (помимо Pearson 2021), "
        "каждый — с реальной публикацией:\n"
        "  • Bérut et al. (2012) Nature 483, 187 — Landauer principle "
        "experimental verification.\n"
        "  • Jun, Gavrilov, Bechhoefer (2014) PRL 113, 190601 — high-"
        "precision Landauer test.\n"
        "  • Koski et al. (2014) PNAS 111, 13786 — Maxwell demon, "
        "info-to-energy conversion.\n"
        "  • Toyabe et al. (2010) Nature Physics 6, 988 — Sagawa-Ueda "
        "second law.\n"
        "  • Pekola group, e.g. Pekola (2015) Nature Physics 11, 118 — "
        "thermodynamics of info in single-electron circuits.\n"
        "  • Camati et al. (2016) PRL 117, 240502 — quantum Maxwell "
        "demon NMR.\n"
        "  • Cottet et al. (2017) PNAS 114, 7561 — quantum Maxwell "
        "demon в circuit QED.\n"
        "  • Если знаешь ещё реальные подтверждающие эксперименты — "
        "ДОБАВЬ. Главное: каждая ссылка ДОЛЖНА быть реальной "
        "публикацией с корректным DOI/PMID.\n"
        "  • Для каждого эксперимента покажи КАК Ze Theory делает "
        "конкретное предсказание, согласующееся с измерением. Не "
        "интерпретация — а количественное согласие.\n\n"
        "**Также применить ВСЕ оставшиеся правки v4-рецензии:**\n"
        "  • Abboud et al.: 'lower bound', НЕ 'upper bound'.\n"
        "  • Удалить любые fabricated DOI (10.65649/*, hn78-7xx3, "
        "обрезанные).\n"
        "  • Размерности к каждому уравнению.\n"
        "  • γ_Ze — отметить как теоретическое предсказание, "
        "ожидающее MCMC-проверки.\n"
        "  • Cheating rate — операциональное определение в нат/с.\n\n"
        "**ФОРМАТ ВЫВОДА:** Полный markdown v5. Структура: title, "
        "abstract (250 слов), введение, новая глава 'Self-Consistent "
        "Foundations', главы с выводами P1/P4 из первых принципов, "
        "расширенная Глава 25 с ≥6 экспериментами, остальные главы "
        "сохранить, исправленный список литературы в конце. ТОЛЬКО "
        "markdown, без преамбул.\n"
    )
    out = ask_long(prompt, system=SYSTEM_AUTHOR, lang="ru")
    _write(V5_PATH, out)


# ── Step 2: citation verification ──────────────────────────────────────────


def _extract_citations(text: str) -> list[str]:
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


def step2_cit() -> None:
    if CIT_REPORT.exists() and CIT_REPORT.stat().st_size > 5_000:
        log.info("step2: skipping"); return
    log.info("step2: citation verification")
    v5 = _read(V5_PATH)
    cites = _extract_citations(v5)
    log.info("extracted %d citation tokens", len(cites))
    bib = re.search(
        r"(?:## *(?:Литература|References|Библиография|Bibliography).*?)(?=\n## |\Z)",
        v5, flags=re.I | re.S)
    bib_text = bib.group(0) if bib else ""
    prompt = (
        "Верифицируй каждую ссылку из v5 рукописи Ze Theory. Источники "
        "проверки: arXiv, PubMed, CrossRef, Semantic Scholar, "
        "INSPIRE-HEP, NASA ADS, journal direct.\n\n"
        "Особое внимание к новым ссылкам, добавленным для расширения "
        "эмпирической базы (Bérut 2012 Nature, Jun-Gavrilov-Bechhoefer "
        "2014 PRL, Koski 2014 PNAS, Toyabe 2010 Nat Phys, Pekola 2015 "
        "Nat Phys, Camati 2016 PRL, Cottet 2017 PNAS).\n\n"
        "Для каждой:\n"
        "(a) реальна ли публикация — VERIFIED / PROBABLE / UNVERIFIED / "
        "INCONSISTENT / FABRICATED;\n"
        "(b) совпадает ли заявленный сюжет;\n"
        "(c) если статья реальна — даёт ли она количественное "
        "подтверждение Ze Theory или это переинтерпретация.\n\n"
        "Формат: markdown-таблица + резюме + список конкретных правок.\n\n"
        "## Извлечённые токены\n\n"
        + "\n".join(f"- {c}" for c in cites) + "\n\n## Bibliography\n\n"
        + (bib_text or "(не выделена явно)\n")
    )
    out = ask_deep(prompt, system="Ты — научный библиограф для журналов "
                   "класса Nature/PRX. Не делаешь скидок, не доверяешь "
                   "автору на слово.", lang="ru")
    _write(CIT_REPORT, out)


# ── Step 3: expanded meta-analysis ─────────────────────────────────────────


def step3_meta() -> None:
    if META_REPORT.exists() and META_REPORT.stat().st_size > 5_000:
        log.info("step3: skipping"); return
    log.info("step3: expanded meta-analysis")
    v5 = _read(V5_PATH)
    prompt = (
        "Проведи МЕТА-АНАЛИЗ всех экспериментов, цитируемых в v5 как "
        "эмпирическое подтверждение Ze Theory.\n\n"
        "Для каждого:\n"
        "(1) первичная публикация: дизайн, выборка, эффект, p/CI;\n"
        "(2) независимые воспроизведения с DOI;\n"
        "(3) согласованность multi-lab;\n"
        "(4) систематические ошибки и критика;\n"
        "(5) РЕАЛЬНО ли подтверждает Ze Theory или общую "
        "стохастическую термодинамику Crooks/Jarzynski/Landauer;\n"
        "(6) уровень: A (multi-lab replicated, direct prediction Ze) / "
        "B (single-lab, direct prediction) / C (multi-lab, "
        "consistent-but-not-unique) / D (interpretation only).\n\n"
        "В заключении дай Cochrane-style summary: forest-plot-style "
        "таблица с эффект-сайзами и confidence intervals, общий "
        "вывод об эмпирической надёжности теории, и ОТДЕЛЬНО — какие "
        "из экспериментов могут считаться 'unique signature of Ze' vs "
        "'consistent with standard stochastic thermo'.\n\n"
        "## v5\n\n"
        + v5[:80_000]
    )
    out = ask_deep(prompt, system="Ты — методолог meta-analysis Cochrane/"
                   "PRISMA для квантовой информации и стохастической "
                   "термодинамики.", lang="ru")
    _write(META_REPORT, out)


# ── Step 4: new super-strict review ────────────────────────────────────────


def step4_review() -> None:
    if REV_V5_PATH.exists() and REV_V5_PATH.stat().st_size > 5_000:
        log.info("step4: skipping"); return
    log.info("step4: super-strict v5 peer review")
    v5 = _read(V5_PATH)
    cit = _read(CIT_REPORT) if CIT_REPORT.exists() else ""
    meta = _read(META_REPORT) if META_REPORT.exists() else ""
    prompt = (
        "Ты — экспертная группа из 3 ведущих рецензентов журналов "
        "класса Nature, Physical Review X, Reviews of Modern Physics. "
        "IF целевого журнала ≥18. Тройное слепое рецензирование v5 — "
        "после того как автор ЯКОБЫ решил три фундаментальные "
        "проблемы (P1 циклическая логика, P2 вывод уравнений из "
        "первых принципов, P3 расширенная эмпирическая база).\n\n"
        "Твоя задача — проверить, действительно ли v5 решает эти "
        "проблемы, или это косметическое улучшение.\n\n"
        "Структура отзыва (русский):\n"
        "1. Executive summary с явной оценкой Accept/Minor/Major/Reject.\n"
        "2. Проверка решения P1: реально ли доказано существование/"
        "единственность? Корректна ли теорема Banach fixed-point в "
        "данном контексте? Есть ли численный пример сходимости?\n"
        "3. Проверка решения P2: вывод P1 из Crooks/Landauer — это "
        "настоящий вывод или ремэппинг символов? Сходится ли "
        "α = k_B T ln 2 / ⟨ΔS⟩ с Pearson 2021 численно?\n"
        "4. Проверка решения P3: использовать citation report и "
        "meta-analysis. Сколько новых экспериментов реально верифи-"
        "цированы? Сколько из них дают unique signature Ze vs стан-"
        "дартной стох. термодинамики?\n"
        "5. Новизна сверх FEP/IIT/GNW/Stoch.Thermo/Info Geometry.\n"
        "6. Математическая строгость каждого нового вывода.\n"
        "7. Falsifiability score 0–10 для каждого предсказания + "
        "конкретный эксперимент.\n"
        "8. Сравнение с альтернативами (Wheeler-DeWitt, Verlinde, "
        "Penrose OR, ER=EPR).\n"
        "9. Этика, прозрачность, COI.\n"
        "10. MUST-FIX и SHOULD-FIX.\n"
        "11. Альтернативные журналы если IF≥18 не достижим.\n"
        "12. Заключительный вердикт.\n\n"
        "Стиль: жёсткий, без эпитетов, технический. Никакой воды. "
        "Никакого подхалимства автору.\n\n"
        "## v5 рукопись\n\n"
        f"{v5}\n\n"
        "## Citation verification report\n\n"
        f"{cit}\n\n"
        "## Meta-analysis report\n\n"
        f"{meta}\n"
    )
    out = ask_deep(prompt, system="Старший рецензент Nature/PRX. "
                   "Не делаешь скидок.", lang="ru")
    _write(REV_V5_PATH, out)


def main() -> None:
    t0 = time.time()
    step1_v5()
    step2_cit()
    step3_meta()
    step4_review()
    log.info("done in %.1fs", time.time() - t0)
    for p in (V5_PATH, CIT_REPORT, META_REPORT, REV_V5_PATH):
        sz = p.stat().st_size if p.exists() else 0
        log.info("  %s (%d bytes)", p, sz)


if __name__ == "__main__":
    main()
