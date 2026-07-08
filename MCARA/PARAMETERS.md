# MCARA — Parameters

**Обновлено:** 2026-07-06

## Теоретическая модель

| Параметр | Значение | Описание |
|----------|----------|----------|
| N_counters | 9 | #1 центриольный, #2 теломерный, #3 митохондриальный, #4 эпигенетический, #5 протеостатический, #6 piRNA, M1-M9 механизмы |
| f_i(x) | Сигмоида (логистическая) | Функция накопления повреждений для counter i |
| D_critical | Ткане-специфичный | Порог повреждений, после которого СК теряет функцию |
| L_tissue | FI/0.7 | Tissue burden = Frailty Index / 0.7 |
| Γ_ij | Метод парных perturbation | Матрица coupling между counters (экспериментально не определена) |
| BHCA-causal | 9 критериев Bradford-Hill | Нарративная оценка present/partial/absent/contradictory |

## Экспериментальные параметры (Phase III)

| Параметр | Значение | Источник |
|----------|----------|----------|
| HSC differentiation rate (α) | 10⁻³ – 0.02 /день | Pan et al. 2023 |
| Progenitor proliferative potential (L*) | 22 деления | Pan et al. 2023 |
| HSC niche capacity (K) | 10⁴ – 10⁵ клеток | Pan et al. 2023 |
| n на группу | ≥18 клонов | Power analysis (α=0.0042 Holm-Bonferroni, q<0.05 FDR) |
| Аттриция | +50% | Abortive clones + technical failures |
| Статистический метод | FDR (Benjamini-Hochberg) primary; Holm-Bonferroni conservative alt |
| OSF pre-registration | До сбора данных | DOI будет присвоен |

## Бюджет

| Фаза | Бюджет | Длительность | Источник |
|------|------:|-------------|----------|
| Phase A — ARGUS | €90,000 | 6 мес | EIC Pathfinder |
| Phase B — Counter #1 | €200,400 | 12 мес | EIC Pathfinder |
| Phase III — Counters #2–6 | €420,000 | 12 мес | Отдельный грант |
| Phase IV — CELTRA-MAP | €3,240,000 | 36 мес | ERC Advanced |
| **Всего** | **€3,950,400** | | |

## Оценка теории

| Компонент | Оценка |
|-----------|:---:|
| C1 (polyGlu ↔ divisions) | 7/10 |
| C2 (asymmetric inheritance) | 9/10 |
| M1 (chromosome segregation) | 8/10 |
| M2 (ciliary signaling) | 8/10 |
| M3 (CAASM) | 7/10 |
| M4 (MT arrays → senescence) | 6/10 |
| M5 (centrosome amplification) | 7/10 |
| M6 (loss of polarity) | 7.5/10 |
| M7 (centrosome proteostasis) | 5/10 |
| M8 (neurogenesis) | 6/10 |
| M9 (oocyte/meiotic) | 6/10 |
| **Общая** | **7.8/10** |

## Ключевые references

| Статья | PMID/DOI |
|--------|----------|
| Hallmarks of stem cell aging (Rando/Brunet/Goodell 2025) | 40562035 |
| Mother centrosome → T cell memory (Barandun/Oxenius 2025) | 10.1016/j.celrep.2024.115127 |
| Senescence ≠ telomere length (Passanisi/Spencer 2026) | 10.1016/j.isci.2026.114801 |
| UBE2G1 in HSC aging (2026) | 10.3324/haematol.2026.300724 |
| Mitochondrial drivers of SC aging (2026) | 10.1038/s41514-026-00422-5 |
| Somatic piRNA/PIWI review (2024) | 10.3389/fcell.2024.1495035 |
