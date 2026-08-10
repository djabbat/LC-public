# CEDAR – Centriolar Entropy-Damage Accumulation Ratchet

**Status:** Active theory, Counter #1 of MCARA (Multi‑Counter Architecture of Organismal Aging).  
**Last update:** 2026-08-08.

## Overview

CEDAR provides a mechanistic explanation for stem cell aging that is independent of telomere attrition and oxidative stress. It posits that progressive accumulation of polyglutamylation on the mother centriole – a stable cytoskeletal structure passed asymmetrically to the stem daughter – impairs primary cilium signaling, leading to a decline in self‑renewal divisions and eventual pool exhaustion.

## Core Documents

| File | Description |
|------|-------------|
| `CONCEPT.md` | Full theory description, axioms, predictions, meta‑analysis, and references |
| `PARAMETERS.md` | Table of 32 model parameters with sources and calibration status |
| `TEAM_AND_BUDGET.md` | PI track record, team composition, budget breakdown, risk matrix, and pre‑registration plan |

## Key References (Verifiable)

- Tqemaladze J. (2023). Centriolar damage accumulation theory of aging. *J. Theor. Biol.*, 563, 111456. DOI: 10.1016/j.jtbi.2023.111456  
- Royall L. et al. (2023). Asymmetric inheritance of mother centrosome. *eLife*, 12, e83997. PMID 37184769  
- Bobinnec Y. et al. (1998). Glutamylation of centriole. *J. Cell Biol.*, 143(6), 1575‑1589. PMID 9576819  
- Kaur R. et al. (2023). Proteomic landscape of PTMs in human fibroblasts. *Nat. Commun.*, 14, 4567. PMID 37478901  

## Current Barriers

The main remaining experimental gap is direct measurement of PTM accumulation (polyGlu) proportional to division number in hematopoietic stem cells (C1) and demonstration of asymmetric centriole inheritance in HSCs (C2). The current proposal (see `TEAM_AND_BUDGET.md`) is designed to fill these gaps.

## Pre‑registration

All experiments proposed in this project will be pre‑registered on the Open Science Framework (OSF) before data collection begins. Details are provided in `TEAM_AND_BUDGET.md`.

---

**LERR — Ladder, Eliminate, Reprogram, Rebuild.**

**Step 1 (Ladder).** Cut the damage load first: slow the counter, push old centrioles into differentiating daughters, remove only the mother centriole, keep spare young ones.

**Step 2 (Eliminate).** Take out the old centriole. Restore telomeres. Wipe the epigenome. Rescue mitochondria.

**Step 3 (Reprogram).** Push to totipotency with DUX4 + KDM4D + DPPA3.

**Step 4 (Rebuild).** Grow fresh centrioles de novo. Derive clean, young adult stem cells.
**Step 1 (Ladder).** Де-риск перед элиминацией по текущим данным: замедлить счётчик (NAC-антиоксидант; обратимые PTM: TTL-ре-тирозинирование, CCP5/6-деглутамилирование); сегрегировать повреждения асимметричным наследованием материнской центриоли в дифференцирующееся потомство (Yamashita, 2007; Royall, 2023 — человеческие NPC); геми-элиминировать только материнскую центриоль (лазер/PROTAC), сохраняя контроль дупликации и избегая p53-зависимого G1-ареста (Meitinger, 2016); кондиционировать клетку (запасные PLK4-центриоли, синхронизация G1/S, протеостаз); отобрать наименее повреждённый пул (FACS по низкому Δ2/полиGlu).
**Step 2 (Eliminate).** Убрать старую повреждённую центриоль; восстановить теломеры (теломераза/ZSCAN4 через H3K14ac/H3K18ac; Meltzer, 2024); стереть эпигенетические метки (OSK/TET1-TET2-TDG; Lu, 2020 — частично, остаётся линейная память); отобрать здоровые митохондрии (PINK1-зависимая митофагия; Vázquez-Martín, 2016).
**Step 3 (Reprogram).** Индуцировать тотипотентность: DUX4 + KDM4D + DPPA3 — DUX4 открывает cleavage-стадийные гены (Hendrickson, 2017), KDM4D снимает H3K9me3-барьер репрограммирования, DPPA3 (Stella) стабилизирует тотипотентное (2C-подобное) состояние.
**Step 4 (Rebuild).** Пересобрать молодые центриоли de novo (PLK4 → SAS-6 → STIL → CPAP; Nigg & Holland, 2018; Gönczy, 2012) после полной элиминации (Khodjakov, 2002; Uetake, 2007); контроль геометрии (9-кратная симметрия, триплеты, длина); получить безопасные молодые взрослые стволовые клетки (проверка кариотипа, восстановление p53).
**Step 1 (Ladder).** Де-риск перед элиминацией: замедлить счётчик, сегрегировать повреждения, геми-элиминировать материнскую центриоль, кондиционировать клетку, отобрать наименее повреждённый пул.
**Step 2 (Eliminate).** Убрать старую центриоль; восстановить теломеры; стереть эпигенетические метки; отобрать здоровые митохондрии.
**Step 3 (Reprogram).** Индуцировать тотипотентность: DUX4 + KDM4D + DPPA3.
**Step 4 (Rebuild).** Пересобрать молодые центриоли de novo; получить безопасные молодые взрослые стволовые клетки.
