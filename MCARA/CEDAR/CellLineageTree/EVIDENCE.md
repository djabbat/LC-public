# CDLT — EVIDENCE

**Project:** Centriole Damage in Lineage Tree (CDLT) — Counter #1 (Centriolar) of MCARA.
**Last update:** 2026-10-14 — Whole-mammal lineage tracing (DNA Typewriter Science + PEtracer Cell + 2-cell asymmetry Cell 2024).

> Контекст: CDLT — часть CEDAR (см. родительский `LC/MCARA/CEDAR/EVIDENCE.md` для полной эмпирической базы CEDAR/Counter #1). Этот файл собирает доказательства, специфичные для **lineage-реконструкции** — методологии и масштаба платформы.

---

## 1. Whole-Mammal Lineage Tracing — прецеденты осуществимости (2026-10-14)

> Триггер: статья iXBT Live 2026-10-14 «От оплодотворенной клетки до органов». Полный разбор: `../CEDAR/docs/ANALYSIS_2026-10-14_WholeMammal_LineageTracing.md`. Все PMID верифицированы NCBI 2026-10-14.

### 1.1 DNA Typewriter — полная филогения мыши (Science 2026)

**Yu Q et al., Shendure lab (UW)** — *A DNA Typewriter records the cell lineage history of a mouse, from zygote to late organogenesis.* **Science** 2026-10-08:eael0508. PMID **42848909**, DOI 10.1126/science.ael0508.

| Утверждение | PMID/DOI | Проверено | Сила |
|---|---|---|---|
| Реконструирована time-calibrated, parsimony-supported, zygote-rooted филогения **1,28 млн** транскрипционно профилированных клеток из **одного** эмбриона (зигота→поздний органогенез, ~13,5 дн) | 42848909 | ✅ 2026-10-14 | **Strong** |
| Burst редактирования однозначно помечает **дочерей первого деления** (inline replicates) | 42848909 | ✅ | Strong |
| Количественно показана **клональная доминантность при гаструляции** | 42848909 | ✅ | Strong |
| Tree siblings делят клеточный тип выше случайного; гетеротипные сибсы = терминальные дифференцировки | 42848909 | ✅ | Strong |
| Temporal sweeps clade co-occurrence восстанавливают датированную иерархию связей типов клеток | 42848909 | ✅ | Moderate |

### 1.2 PEtracer — 16 эмбрионов, ~75% делений (Cell 2026)

**Colgan WN et al., Weissman lab (Whitehead)** — *Comprehensive lineage tracing maps the landscape of cell fate decisions in mouse embryogenesis.* **Cell** 2026-10-08:S0092-8674(26)01196-7. PMID **42849436**, DOI 10.1016/j.cell.2026.09.050.

| Утверждение | PMID/DOI | Проверено | Сила |
|---|---|---|---|
| PEtracer непрерывно устанавливает наследуемые метки; реконструкция lineage trees, разрешающих **~75% клеточных делений**, по **>1,4 млн клеток из 16 репликатных эмбрионов** (E7.5–E10.0, полу-дневные интервалы) | 42849436 | ✅ | **Strong** |
| Сопряжение lineage trees с глубоким транскрипционным профилированием для разрешения судеб клеток | 42849436 | ✅ | Strong |

### 1.3 Асимметрия 2-клеточной стадии у человека (Cell 2024)

**Junyent S et al., Zernicka-Goetz lab** — *The first two blastomeres contribute unequally to the human embryo.* **Cell** 187(11):2838-2854.e17 (2024). PMID **38744282**, DOI 10.1016/j.cell.2024.04.029.

| Утверждение | PMID/DOI | Проверено | Сила |
|---|---|---|---|
| Большинство клеток эпибласта (тела) человека происходит из **одного** бластомера 2-клеточной стадии | 38744282 | ✅ | **Strong** |
| Только **1–3 клетки интернализуются** на переходе 8→16; чаще из первого делителя 2-клеточной стадии | 38744282 | ✅ | Strong |
| Динамика деления + bottleneck интернализации устанавливают асимметрию клонального состава будущего тела | 38744282 | ✅ | Strong |

---

## 2. Прямые предшественники (уже в refs/)

| PMID | Работа | Значение |
|---|---|---|
| **30093604** | Kalhor R et al. 2018, Science — Developmental barcoding of whole mouse | Предшественник генетического barcoding мыши |
| **31086336** | Chan MM et al. 2019, Nature — Molecular recording of mammalian embryogenesis (Weissman lab) | Предшественник записи в эмбрионе; оценка числа эмбриональных прогениторов и степени асимметричного распределения |
| **29674432** | Plass M et al. 2018, Science — Cell type atlas and lineage tree of planaria | Lineage tree беспозвоночного (планария) |

---

## 3. Выводы для CDLT

1. **Осуществимость доказана:** полная реконструкция lineage млекопитающего — реальность (Science 2026 + Cell 2026, оба 2026-10-08).
2. **Power Analysis:** Weissman n=16 → CDLT должен предусматривать ≥2 эмбриона/условие для оценки межиндивидуальной вариабельности; ~75% покрытие делений достаточно для содержательной реконструкции.
3. **Маркировка первого деления** (DNA Typewriter) — прототип для отслеживания асимметрии мать/дочь in vivo.
4. **Bottleneck интернализации 8→16** (человек) — где редкие дефекты клеток клонально экспандируют в тело → место приложения центриолярного счётчика.
5. **Комплементарность:** оптический (CDLT/Aubrey) vs молекулярный (DNA Typewriter/PEtracer) — два независимых свидетеля lineage.

---

## 4. Слабые места / ограничения

| Ограничение | Влияние |
|---|---|
| Shendure: n=1 эмбрион для полной реконструкции | Нужны повторы для вариабельности |
| Weissman: ~75% делений, E7.5–E10.0 (не от зиготы) | Неполное покрытие ветвей; разные окна |
| Молекулярный, постфактум (parsimony-дерево, не живая динамика) | CDLT-оптический комплементарен; нужна калибровка |
| Нет центриолярной информации в обоих методах | **Пробел, который заполняет CDLT** |
| Человек: малое n, этика, частично ретроспективно | Подтверждающая роль для CEDAR |
| Препринты (42619846, 42146547) не рецензированы | Цитировать журнальные версии |

---

## 5. Ref-файлы (добавлены 2026-10-14)

- `refs/PMID_42848909_Yu_2026_science_dna_typewriter_records_mouse_lineage.md`
- `refs/PMID_42849436_Colgan_2026_cell_comprehensive_lineage_tracing_mouse_embryo.md`
- `refs/PMID_38744282_Junyent_2024_cell_first_two_blastomeres_contribute_unequally.md`
