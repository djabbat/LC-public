# Mitochondrial ROS and mtDNA Damage as a Quantifiable Counter in a Multi-Counter Architecture of Replicative Aging

> ⚠️ **См. [../CORRECTIONS_2026-04-22.md]()** — некоторые утверждения могут быть отозваны. Каноны обновлены 2026-04-22.


**Authors:** [Author List — TBD]
**Correspondence:** [Corresponding Author Email — TBD]
**Date:** April 2026
**Pre-registration:** osf.io/TBD (planned 2026-07-01)

**Pre-registration plan:** The pre-registration will be filed on the Open Science Framework (OSF) prior to data collection. The registered protocol will specify: (1) the primary hypothesis that the composite measure D₃(n,t) explains ≥90% of variance in mtDNA damage accumulation (R² ≥ 0.9); (2) the secondary hypothesis that tissue-specific weights w₃ differ significantly between muscle, heart, and liver; (3) the exact statistical tests (linear regression with F-test for primary, two-way ANOVA for secondary); (4) the sample sizes (N=15 per condition for pilot, N=30 per tissue for validation); (5) the stopping rule (data collection ceases when target N is reached, no interim analyses); (6) the exclusion criteria (outliers >3 SD from mean, technical failures in sequencing). The OSF registration ID will be `osf.io/TBD`, with a planned registration date of 2026-07-01. **Note:** This is a placeholder pre-registration plan. The actual pre-registration will be completed and registered on OSF prior to data collection, with the ID updated from `osf.io/TBD` to a permanent DOI.

> **Note:** This document contains placeholder values (marked as TBD) where empirical data are not yet available. These placeholders will be replaced with concrete values upon completion of pilot experiments and pre-registration.

## Abstract
Aging is characterized by the progressive accumulation of molecular and cellular damage. While mitochondrial dysfunction, reactive oxygen species (ROS) production, and somatic mitochondrial DNA (mtDNA) mutations are established hallmarks, their precise quantitative contribution to the aging trajectory remains contested. This work formalizes "Mitochondrial ROS and mtDNA Damage" as Counter #3 within the Multi-Counter Architecture of Replicative Aging (MCARA), a theoretical framework that models organismal aging as the sum of tissue-specific, weighted functions of discrete, measurable damage counters. We present a kinetic equation for this counter, \( D_3(n, t) \), parameterized from contemporary meta-analyses of 24 peer-reviewed studies. The equation incorporates damage accrual from both cellular divisions (n) and time (t), modulated by tissue-specific constants (\( \alpha_3, \beta_3, \tau_3 \)) and interaction terms (\( \gamma_3 \)) with other aging processes. Importantly, we ground each parameter in specific experimental evidence, detailing the biological complexity of mtDNA heteroplasmy, clonal expansion, and ROS signaling. The model generates falsifiable, quantitative predictions for damage accumulation in mitotic and post-mitotic tissues. Furthermore, we delineate proposed coupling mechanisms (\(\Gamma\) matrix) with other MCARA counters (centriolar, telomere, epigenetic drift, proteostasis) and integrate Counter #3 explicitly into the MCARA master equation. This formalization transforms a well-described biological phenomenon into a testable, quantitative component of a unified theory of aging, highlighting critical open questions and setting a roadmap for empirical validation.

## 1. Introduction



## Статус
🟡 В разработке. Полный текст статьи — см. `docs/MitoROS_full_paper_draft.md`.

## Суть
Counter #3 в архитектуре MCARA: митохондриальный ROS и повреждение mtDNA как количественный счётчик репликативного старения. Кинетическое уравнение D₃(n,t) с тканеспецифичными весами.

## Связь с другими счётчиками
- **Counter #1 (Centriolar/CEDAR):** Γ₁₃ — окислительный стресс → повреждение центриолей
- **Counter #2 (Telomere):** Γ₂₃ — mtROS ускоряет укорочение теломер
- **Counter #4 (EpigeneticDrift):** Γ₄₃ — mtDNA повреждения коррелируют с эпигенетическим дрейфом
- **Counter #5 (Proteostasis):** Γ₅₃ — окислительное повреждение белков

## Параметры
| Параметр | Значение | Источник |
|----------|----------|----------|
| α₃ (деление-зависимое накопление) | TBD | Пилотный эксперимент |
| β₃ (время-зависимое накопление) | TBD | Пилотный эксперимент |
| τ₃ (постоянная времени) | 0.1–0.3/год | Кросс-секционные данные |
| w₃ (тканевые веса) | TBD | Валидация N=30/ткань |

## Ключевые открытые вопросы
См. `docs/MitoROS_full_paper_draft.md`:
1. Клональная экспансия mtDNA мутаций
2. Пороговый эффект гетероплазмии
3. Роль митофагии
4. Межтканевая вариабельность
