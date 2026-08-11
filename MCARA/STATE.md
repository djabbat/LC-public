# STATE — MCARA

**Date: 2026-07-24
**Status:** 🟢 Active. Pierre Gönczy's response received — centrioles are preserved for a reason (signaling or centrosomal). 🆕 Iron-Positive Centriole Remnant hypothesis added to CONCEPT.md §0.

## 2026-07-23: ERR repo created
- GitHub: https://github.com/Georgia-Longevity-Alliance/ERR
- Marketing folder: ~/Desktop/Marketing/ERR/ (renamed from MCARA_EIC_Pathfinder)
- README contains Cologne 2026 handout
## Counters
- C1: CEDAR — centriolar
- C2: Telomere
- C3: MitoROS
- C4: EpigeneticDrift
- C5: Proteostasis

## Current focus
- Consortium: Gönczy (EPFL) responded — field is open, mechanism is unknown, recommended Pimenta-Marques 2023
- Article v8: in Biogerontology (ID 7cc6de62), status 🟡 With Editor
- EIC Pathfinder grant: deadline October 28, 2026
- CIRCBIO-07: deadline September 17, 2026

## Latest changes (2026-07-19 — major update)

### Exchange with Pierre Gönczy
- Three questions — three answers. Field confirmed as open
- Recommended: Pimenta-Marques 2023 (ANA1/CEP295) + Kalbfuss & Gönczy review
- Follow-up letter ready

### In-depth audit (41 PMID)
- 93% verified (38/41)
- 🔴 Lindhout 2021 — corrected (contradicted CEDAR)
- ⚠️ Bodnar/Parrinello overstatements corrected
- Bradford Hill: 5/9 → 6/9 (Temporality 🟢, Specificity 🟡)

### New articles integrated
- **Centriculum** (Maheshwari/Cohen-Fix 2023 Curr Biol, 2026 J Cell Sci) — ER-membrane reticulum around centrosome
- **Spermatogenesis** (Ishida & Shibuya 2026) — gamete asymmetry: oogenesis (elimination) vs spermatogenesis (preservation)
- **Tweedell** — not found on PubMed (old article without indexing)

### File versions
- CONCEPT.md: v4.7 (+Centriculum, +Spermatogenesis, +Three-stage model)
- THEORY.md: v4.7 (+Centriculum, +Gamete asymmetry)
- EVIDENCE.md: v4.7 (§11 Gönczy, §12 Centriculum, §13 Spermatogenesis)
- MEMORY.md: updated
- Audit: `audits/MCARA_Evidence_Audit_2026-07-19.md`

## Next steps
- [x] Follow-up to Pierre sent (July 19)
- [x] Pierre's response received (July 21) — centrioles are preserved for a functional reason
- [ ] Integrate Pierre's response into EVIDENCE.md §11
- [ ] CCP1-KO experiment as the next critical step
- [ ] Update article v8 with new data
## Simulator v0.5 — 2026-08-08
- Эпигенетический счётчик калиброван: τ=100 лет, β=1.0, d_critical=0.75 (EpigeneticDrift).
- Геометрический счётчик центриоли: модель Орнштейна–Уленбека (α≈0.97), наследование геометрии мать→дочь (Wang 2014; Panda 2024).
- Асимметричное деление в стволовых тканях: старейшая центриоль остаётся в СК → retention_boost ×1.6, α_eff=0.985.
- Функции цилии (порог damage 0.30) и центросомы (порог 0.60) деградируют с накоплением.
- Тесты: 18/18 проходят (mcara_core 6, mcara_simulation 9, mcara_compare 3).

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
