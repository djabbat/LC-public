# MCARA — Parameters

**Обновлено:** 2026-07-08 (v4.5 — Stress Integrator + Hardware/Software)

## Модель центриолярного счётчика (v4.5)

### Линейное приближение
| Параметр | Значение | Описание | Статус |
|----------|----------|----------|:------:|
| β | TBD | Время-зависимая энтропия (GT335 IF в non-dividing cells) | ❌ Не откалиброван |
| λ | TBD | Ремоделирование CAASM за асимметричное деление (BioID + quantitative proteomics) | ❌ Не откалиброван |
| η(t) | TBD | Окислительные повреждения (oxidative damage markers на изолированных центросомах) | ❌ Не откалиброван |
| δ | TBD | Эффективность деглутамилаз (CCP1 enzymatic activity assay через пассажи) | ❌ Не откалиброван |
| S₀ | TBD | Базовый уровень polyE при рождении / в молодом состоянии | ❌ Не откалиброван |

### Сигмоидальная модель
| Параметр | Значение | Описание |
|----------|----------|----------|
| S_max | TBD | Максимальный уровень polyE |
| k | TBD | Крутизна сигмоиды |
| t½ | TBD | Критический порог — точка ускорения polyE (onset сенесценса?) |

### Динамика polyE баланса
| Параметр | Значение | Описание |
|----------|----------|----------|
| k_E | TBD | Константа скорости глутамилирования (TTLL activity) |
| k_CCP | TBD | Константа скорости деглутамилирования (CCP efficacy) |

## Теоретическая модель (MCARA v1.0)

| Параметр | Значение | Описание |
|----------|----------|----------|
| N_counters | 5 | C1 центриолярный, C2 эпигенетический, C3 митохондриальный, C4 структурный, C5 теломерный |
| f_i(x) | Сигмоида (логистическая) | Функция накопления повреждений |
| D_critical | Ткане-специфичный | Порог повреждений для потери функции СК |
| L_tissue | FI/0.7 | Tissue burden = Frailty Index / 0.7 |
| Γ_ij | 0 (default) | Матрица coupling (экспериментально не определена) |
| BHCA-causal | ~5/9 критериев | Bradford Hill: класс II–III |

## Экспериментальные параметры

| Параметр | Значение | Источник |
|----------|----------|----------|
| HSC differentiation rate (α) | 10⁻³ – 0.02 /день | Pan et al. 2023 |
| Progenitor proliferative potential (L*) | 22 деления | Pan et al. 2023 |
| HSC niche capacity (K) | 10⁴ – 10⁵ клеток | Pan et al. 2023 |
| n на группу (центральный эксперимент) | ≥6 biological replicates | Power analysis (α=0.05, power=0.8, SD≈30% mean) |
| Аттриция | +50% | Abortive clones + technical failures |

## Бюджет

| Фаза | Бюджет | Длительность |
|------|------:|-------------|
| Phase 0 — Critical experiment | €50,000 | 6 мес |
| Phase 1 — ARGUS | €90,000 | 12 мес |
| Phase 2 — Race | €860,000 | 18 мес |
| Phase 3 — Rejuvenation | €800,000 | 12 мес |
| Phase 4 — Integrated | €650,000 | 9 мес |
| Phase 5 — Transplantation | €580,000 | 6 мес |
| PM + этика + регуляторика | €150,000 | |
| **Всего** | **~€3.2M** | 36 мес |

## Оценка теории (v4.5)

| Компонент | Оценка | Изменение |
|-----------|:---:|:---:|
| C1 (polyGlu ↔ divisions) | 7/10 | — |
| C2 (asymmetric inheritance) | 9.5/10 | ↑ (Barandun 2025 — млекопитающие) |
| M1 (chromosome segregation) | 8/10 | — |
| M2 (ciliary signaling) | 8/10 | — |
| M3 (CAASM) | 8/10 | ↑ (ATF5 + FERMT3 + ALMS1) |
| sinc-MT/KIFC3 pathway | 9/10 | 🆕 Почти полный механизм |
| M4 (MT arrays → senescence) | 8/10 | ↑ (Robichaud 2024) |
| M5 (centrosome amplification) | 7/10 | — |
| M6 (loss of polarity) | 7.5/10 | — |
| M7 (centrosome proteostasis) | 5/10 | — |
| M8 (neurogenesis) | 6/10 | — |
| M9 (oocyte/meiotic) | 6/10 | — |
| Bradford Hill causal | 5/9 | 🆕 Класс II–III |
| **Общая** | **7.9/10** | ↑ |

## Ключевые references (v4.5)

| Статья | PMID/DOI |
|--------|----------|
| CEDAR original | 36583780 |
| Mother centrosome → T cell memory | 39764850 |
| Senescence ≠ telomere length | 41816297 |
| sinc-MT/KIFC3 → senescence | 39266565 |
| ARL13B-ARL3 → FBF1 release | 37019904 |
| ATF5 — PCM↔centriole bridge | 26213385 |
| FERMT3 → centriole senescence | 42343301 |
| ALMS1 IDP → centriole memory | 42380124 |
| CCP1–CCP6 deglutamylases | 21074048 |
| CCP1 KO → impaired osteogenesis | 40349688 |
| Centriole disassembly — polyE specific | 9852152 |
| Irreversible damage beyond epigenome | 41230623 |
| Transcriptional entropy ↑ with age | 41571679 |
| Entropy-based aging framework | 41299832 |
| ECG entropy → fractures + mortality | 40931490 |
| Centriole = maturity sensor | 33835529 |
| Cilium resorption failure → senescence | 30596512 |
| Detyrosination ↑ with age | 39412222 |
| CENP-A decline → p53 senescence | 39809271 |
| hTERT + hypoxia → epigenetic clock ticks | 30332397, 31113906 |
| 3% O₂ immortalizes mouse, not human | 12855956 |
