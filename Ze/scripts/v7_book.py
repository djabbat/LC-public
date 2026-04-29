#!/usr/bin/env python3
"""
Apply 7 MUST-FIX items from v6 review to Ze_v6.md → produce
Ze_Book_v7.md on Desktop. Russian text. NO new citations
beyond the verified-real set explicitly listed below.
"""
from __future__ import annotations
import os, sys, time, logging
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
from llm import ask_long  # noqa: E402

logging.basicConfig(level=logging.INFO,
                    format="%(asctime)s %(levelname)s %(message)s",
                    datefmt="%H:%M:%S")
log = logging.getLogger("ze.book")

ROOT = Path.home() / "Desktop"
V6 = ROOT / "Ze_v6.md"
OUT = ROOT / "Ze_Book_v7_2026-04-29.md"

SYSTEM = (
    "Ты — научный редактор и теоретический физик уровня Foundations "
    "of Physics. Получив рукопись v6 и список 7 обязательных правок, "
    "ты создаёшь v7 в формате научной монографии (книга, ≥10000 слов). "
    "ВАЖНОЕ ОГРАНИЧЕНИЕ: ты НЕ имеешь права добавлять новые цитаты, "
    "которых нет в разрешённом ниже whitelist. Если для аргумента "
    "нужна цитата, которой нет в whitelist, переформулируй "
    "утверждение как 'теоретическое предложение' БЕЗ ссылки. "
    "Возвращай ТОЛЬКО полный markdown книги, без преамбул."
)

ALLOWED_CITATIONS = """
WHITELIST разрешённых цитат (только эти 14, никакие другие):

1. Pearson, A. N., et al. (2021). Measuring the thermodynamic cost of
   timekeeping. Phys. Rev. X 11, 021029. DOI:10.1103/PhysRevX.11.021029
2. Proietti, M., et al. (2019). Experimental test of local observer
   independence. Sci. Adv. 5, eaaw9832. DOI:10.1126/sciadv.aaw9832
3. Woodhead, E., Acín, A., & Pironio, S. (2021). Device-independent
   QKD with asymmetric CHSH inequalities. Quantum 5, 443.
   DOI:10.22331/q-2021-04-26-443
4. Barato, A. C., & Seifert, U. (2015). Thermodynamic uncertainty
   relation for biomolecular processes. Phys. Rev. Lett. 114, 158101.
   DOI:10.1103/PhysRevLett.114.158101
5. Gingrich, T. R., Horowitz, J. M., Perunov, N., & England, J. L.
   (2016). Dissipation bounds all steady-state current fluctuations.
   Phys. Rev. Lett. 116, 120601. DOI:10.1103/PhysRevLett.116.120601
6. Erker, P., Mitchison, M. T., et al. (2017). Autonomous quantum
   clocks: does thermodynamics limit our ability to measure time?
   Phys. Rev. X 7, 031022. DOI:10.1103/PhysRevX.7.031022
7. Crooks, G. E. (1999). Entropy production fluctuation theorem and
   the nonequilibrium work relation. Phys. Rev. E 60, 2721.
   DOI:10.1103/PhysRevE.60.2721
8. Jarzynski, C. (1997). Nonequilibrium equality for free energy
   differences. Phys. Rev. Lett. 78, 2690.
   DOI:10.1103/PhysRevLett.78.2690
9. Bérut, A., et al. (2012). Experimental verification of Landauer's
   principle. Nature 483, 187. DOI:10.1038/nature10872
10. Verlinde, E. (2011). On the origin of gravity and the laws of
    Newton. JHEP 04, 029. DOI:10.1007/JHEP04(2011)029
11. Jacobson, T. (1995). Thermodynamics of spacetime: the Einstein
    equation of state. Phys. Rev. Lett. 75, 1260.
    DOI:10.1103/PhysRevLett.75.1260
12. Friston, K. (2010). The free-energy principle: a unified brain
    theory? Nat. Rev. Neurosci. 11, 127. DOI:10.1038/nrn2787
13. Tononi, G. (2015). Integrated information theory. Scholarpedia
    10(1):4164. DOI:10.4249/scholarpedia.4164
14. Touchette, H. (2009). The large deviation approach to statistical
    mechanics. Phys. Rep. 478, 1. DOI:10.1016/j.physrep.2009.05.002
"""

PROMPT_TEMPLATE = """## Исходная рукопись v6

{v6}

## Семь обязательных правок (MUST-FIX)

1. **Удалить полностью** ссылки Abboud et al. (2026) arXiv:2604.09772
   и Kiely et al. (2026) Phys. Rev. A 113. Они не существуют
   (fabricated, YY=26 на май 2025 невозможен; PRA том 113 не вышел).
   Где они использовались — переформулировать аргумент без ссылки или
   заменить на whitelist-альтернативу.

2. **Исправить дату публикации** с «29 апреля 2026 г.» на реальную:
   «Май 2025 г. (preprint)».

3. **Удалить или исправить Burgholzer (2015)**. Тождество
   «I = ⟨ΔS⟩_gen» в этой работе НЕ содержится. Заменить на корректную
   ссылку: Crooks (1999) PRE 60, 2721 + Jarzynski (1997) PRL 78, 2690
   как первоисточники флуктуационных теорем; либо удалить и
   обосновать тождество как авторский постулат.

4. **Добавить цитирование TUR** в нескольких местах:
   - Barato & Seifert (2015) PRL 114, 158101 — первичный TUR.
   - Gingrich, Horowitz, Perunov, England (2016) PRL 116, 120601 —
     обобщение на стационарные токи.
   - Erker, Mitchison et al. (2017) PRX 7, 031022 — термодинамика
     автономных часов.
   Явно признать, что центральная связь «точность часов ↔ энтропия»
   является следствием TUR, и Pearson et al. (2021) — это
   экспериментальная проверка TUR. Не выдавать TUR-результат за
   уникальное предсказание Ze Theory.

5. **Удалить orphan citations** или интегрировать их в текст с
   конкретным аргументом:
   - Carhart-Harris & Friston (2019) — удалить (нет в whitelist).
   - Dehaene & Naccache (2001) — удалить.
   - Friston (2019) arXiv — заменить на Friston (2010) Nat Rev
     Neurosci 11, 127 (whitelist), интегрировать в раздел сравнения с
     FEP с конкретной цитатой.
   - Braunstein & Caves (1988) — удалить.
   - Brunner et al. (2014) — удалить.
   - Zurek (2003) — удалить (если не используется в тексте).

6. **Коэффициент 1.7478 в CHSH-деформации** S_Ze = 2√2 + δ·1.7478 —
   либо вывести аналитически (показать вычисление), либо честно
   пометить как численная подгонка под желаемый сдвиг и
   переформулировать как «коэффициент порядка единицы, требует вывода
   в будущей работе».

7. **Операционализировать I для ЭЭГ или удалить нейробиологические
   разделы.** Без операционального определения I через измеримые
   величины ЭЭГ-предсказания (анестезия → R_cheat, медитация → I)
   нефальсифицируемы. Рекомендуется удалить разделы про сознание /
   анестезию / медитацию из основной части и оставить только в
   приложении как «спекулятивные расширения, требующие операцион-
   ализации».

## Whitelist цитирования

{whitelist}

## Задача

Создай **Ze Theory: A Theoretical Proposal — Monograph v7** (книга,
≥10 000 слов, на русском). Структура:

- Титульный лист (название, автор, статус «теоретическое предложение»,
  май 2025).
- Аннотация (≤300 слов).
- Глава 1. Введение и мотивация.
- Глава 2. Энтропийный импеданс: формальное определение.
- Глава 3. Постулат P1: время как интеграл импеданса. Включить
  явное признание, что P1 — это переформулировка TUR (Barato &
  Seifert 2015; Gingrich et al. 2016) с подгоночным параметром α,
  определяемым из Pearson et al. (2021).
- Глава 4. Постулат P4: вероятности переходов. Большие отклонения
  (Touchette 2009) как формальное основание. Признать, что переход
  к уравнению Шрёдингера остаётся открытой проблемой.
- Глава 5. CHSH-деформация: ансац и его статус. Честно: коэффициенты
  не выводятся из первых принципов; это эвристика. Ссылка только на
  Woodhead et al. (2021) для асимметричных CHSH.
- Глава 6. Связь с FEP (Friston 2010), IIT (Tononi 2015): что
  переименовано, что добавлено. Без претензий на новизну там, где её нет.
- Глава 7. Эмпирическая база: один эксперимент — Pearson et al. (2021)
  — как проверка TUR, совместимая с Ze Theory как одной из возможных
  интерпретаций. Бérут (2012), Verlinde (2011), Jacobson (1995) —
  как теоретический фон.
- Глава 8. Фальсифицируемые предсказания: только CHSH-зависимость от
  энтропии источника, с честным указанием, что α — подгонка.
- Глава 9. Открытые проблемы и ограничения: циклическая структура
  определения времени; отсутствие вывода КМ из P4; отсутствие
  операционального определения I в нейронауке.
- Приложение A. Спекулятивные расширения (сознание/анестезия/
  медитация) — с явной маркировкой как «требует операционализации».
- Список литературы — ТОЛЬКО whitelist цитаты, реально использованные
  в тексте.

ВАЖНО:
- Никаких ссылок на произведения 2026+ года.
- Никаких префиксов 10.65649 или подобных непроверяемых.
- Каждая цитата в списке литературы должна быть реально использована
  в тексте с указанием места и роли.
- Размерности к каждому ключевому уравнению.
- Если для аргумента нужна цитата, которой нет в whitelist —
  переформулируй без цитаты.

Возвращай только полный markdown книги.
"""


def main():
    if OUT.exists() and OUT.stat().st_size > 30_000:
        log.info("already produced: %s", OUT)
        return
    log.info("reading v6")
    v6 = V6.read_text(encoding="utf-8")
    prompt = PROMPT_TEMPLATE.format(v6=v6, whitelist=ALLOWED_CITATIONS)
    log.info("calling DeepSeek (long context, large output)")
    t0 = time.time()
    out = ask_long(prompt, system=SYSTEM, lang="ru", max_tokens=120_000)
    log.info("response received in %.1fs (%d chars)", time.time() - t0, len(out))
    OUT.write_text(out, encoding="utf-8")
    log.info("wrote %s (%d bytes)", OUT, OUT.stat().st_size)


if __name__ == "__main__":
    main()
