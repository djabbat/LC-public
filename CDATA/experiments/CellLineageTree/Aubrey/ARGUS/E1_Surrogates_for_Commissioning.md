# ARGUS E1 — ДЕШЁВЫЕ СУРРОГАТЫ КЛЕТОК ДЛЯ COMMISSIONING

**Версия:** E1-Surrogates
**Дата:** 2026-05-13
**Назначение:** Замена дорогих BJ-hTERT + RITE + IF блока ($5,586–7,336) на cheap surrogates для отладки HARDWARE и AI-pipeline до коммита на реальную биологию. Все суррогаты дают **двухканальный сигнал red/green** под лазерами 488 + 561 nm — имитация "старая центриоль = красная, молодая = зелёная".

---

## 1. ПРИНЦИП ЗАМЕНЫ

Commissioning не требует биологической достоверности (peer review уже отверг Elodea как surrogate мammalian центриолей — memory `Aubrey/ARGUS/CLAUDE.md`). Но для **отладки железа + AI-pipeline** нужны только три свойства образца:

| Свойство | Требование |
|---|---|
| Двухканальный сигнал | distinct red (~600+ nm) + green (~520 nm) emission под 488/561 ex |
| Размер | от субдифракционного (~200 nm = центриоль) до клеточного (10–100 µm) |
| Стабильность | минимум часы без среды/инкубатора |
| Дешевизна | ≤10 % от настоящего IF блока |

---

## 2. РЕКОМЕНДОВАННЫЙ STACK (по фазам)

### Phase 1 — Optical alignment (только калибровка железа)

| Образец | Цена | Что валидирует |
|---|---|---|
| **TetraSpeck beads 0.1/0.2/0.5/1.0/4.0 µm** (Thermo T7279) | **$280** | 4-цветные шарики (blue/green/orange/dark red) одновременно в каждой частице. Хроматическая аберрация, PSF, регистрация каналов. Субдифракционные размеры = центриольный масштаб. Long-term reusable. |
| **PS-Speck Microscope Point Source Kit** (Invitrogen P7220) | $200 | Альтернатива/дополнение — субмикронные одноцветные шарики 4 цветов |
| **Цветной маникюрный лак (3 цвета) на стекле** | $5 | Грубая проверка эпи-флуоресценции до покупки beads |

### Phase 2 — Live-cell pipeline (focus, drift, time-lapse)

| Образец | Цена | Что валидирует |
|---|---|---|
| **Elodea / Vallisneria / Anubias листья** (аквариумные) | **FREE** | Хлоропласты: 488 ex → **680 em (RED autofluorescence)**. Клеточные стенки лигнин/целлюлоза: 405/488 ex → **520 em (GREEN autofluorescence)**. Клетки 30–100 µm, хлоропласты 5–10 µm. Живёт неделями в воде. Уже знакомый PI образец. |
| **Лук — эпидермис + SYBR Green I** ($80 за бесконечный сток) | $80 | Зелёные ядра (DNA stain) на фоне auto-red хлоропластов соседних слоёв |
| **Propidium Iodide 1 mg/ml** (Sigma P4170) | $30 | Красное ядерное окрашивание мёртвых клеток — пара к SYBR для live/dead контраста |
| **Срез сосновой иглы / бамбука** | FREE | Lignin (green) + chlorophyll (red) в одном срезе без окраски |

**Phase 2 total: $110** (vs. $1,860 первичных антител = **94 % экономии**)

### Phase 3 — AI two-colour asymmetry pipeline (имитация M/D)

Задача: научить AICoordinator различать «старый красный» vs «молодой зелёный» партнёры в одной картинке, до перехода на CEP164/Centrobin.

| Образец | Цена | Что валидирует |
|---|---|---|
| **Смесь дрожжей: половина окрашена FITC, половина PI** (BacLight kit Thermo L7012) | **$200** | Дрожжи 3–5 µm = малая клетка, бактерия-уровневое разрешение. Half green / half red в одной FOV → AI должен считать ratio. Простая культура (YPD overnight). |
| **Tradescantia stamen hairs + calcein-AM + EthD-1** (LIVE/DEAD kit Invitrogen L3224) | $250 | Живые клетки зелёные (calcein), мёртвые красные (EthD-1). Time-lapse green→red transition на osmotic stress = **прямая имитация RITE Cre-mediated swap**. Tradescantia hairs — одна клетка шириной, отлично для lineage tracking. |
| **Chlamydomonas reinhardtii + GFP-mut штамм** (CC-4533 или CC-5328, $50–100 от Chlamy Center) | $100 | Природные хлоропласты RED + GFP-positive cells GREEN. Имеет 2 центриоли/клетку — биологически релевантно для CDATA контекста. Простая культура TAP medium. |

**Phase 3 total: $200–550** (vs. $770 вторичных + $400 controls = **30–70 % экономии**)

### Phase 4 — Pulse-chase RITE simulation (опционально)

| Образец | Цена | Что валидирует |
|---|---|---|
| **Photoactivatable GFP (PA-GFP) на бусинах ИЛИ Kaede-coupled beads** | $300–500 | 405 nm UV pulse → green→red photoconversion за секунды. **Прямой аналог Cre-ER^T2 / 4-OHT switch**, но без биологии. AI учится трекать converted vs unconverted в одной FOV. |
| **Dronpa-conjugated quantum dots** | $400 | Reversible on/off switching под 405/488 nm — для теста agent-driven timing |

---

## 3. СВОДНЫЙ БЮДЖЕТ E1 SURROGATES vs E2 РЕАЛЬНАЯ БИОЛОГИЯ

| Phase | E1 Surrogate $ | E2 Real biology $ | Экономия |
|---|---|---|---|
| Phase 1 (alignment) | 280–485 | 0 (включено в IF) | — |
| Phase 2 (live-cell pipeline) | 110 | 1,860 (первичные АТ) | **94 %** |
| Phase 3 (AI two-colour) | 200–550 | 770 (вторичные) + 400 controls | **70 %** |
| Phase 4 (pulse-chase RITE опц.) | 300–500 | $760 трансфекция Kaede | **40 %** |
| **ИТОГО E1 stack** | **$890–1,645** | **$3,790–5,990** | **~75 %** |

После прохождения E1 commissioning (~$1K) и валидации, что железо + AI-pipeline работают correctly, переход на E2 (реальная биология, $5,586–7,336) — **обоснованный** капитал-коммит.

---

## 4. КАК ВКРУТИТЬ В ARGUS PHASE 1 BOM

В `Aubrey/ARGUS/PARAMETERS.md` уже есть hardware-бюджет $881–1,687. **E1 Surrogates добавляется как отдельная статья $890–1,645**:

```
Hardware (E0):           $881–1,687
+ E1 Surrogate kit:      $890–1,645
─────────────────────────────────
Total commissioning:     $1,771–3,332
```

vs. сразу с реальной биологией: $881 + $5,586 = $6,467 минимум.

**Phased gate decision (Tsomaia design canon):**
- Gate E0→E1: hardware passes alignment с TetraSpeck → разрешён Elodea/yeast live test
- Gate E1→E2: AI-pipeline различает red/green с >85 % accuracy на surrogates → разрешена закупка BJ-hTERT + RITE + IF
- Gate E2→Phase A: tag-swap efficiency ≥70 % + asymmetry ratio ≥0.6 на c1 → старт 6-мес окна

---

## 5. ПРИОРИТЕТ ЗАКУПКИ (E1 minimum viable)

**Если бюджет на старт ограничен ~$400:**

- [ ] TetraSpeck T7279 — $280 (Phase 1 alignment, обязательно)
- [ ] SYBR Green I 10,000× stock — $80 (Phase 2 nuclei)
- [ ] Propidium Iodide 1 mg/ml — $30 (Phase 2 dead cells)
- [ ] Elodea / Anubias из аквариума — FREE

**Расширение до $1,000:**

- [ ] BacLight L7012 LIVE/DEAD yeast kit — $200 (Phase 3 AI two-colour)
- [ ] LIVE/DEAD L3224 Tradescantia — $250 (Phase 3 RITE simulation)
- [ ] DAPI 10 mg — $30 (universal counterstain)
- [ ] Cover slips + slides — $30

**Опциональные ($1K → $1.5K):**

- [ ] PA-GFP beads или Kaede-conjugated particles — $300 (Phase 4 photoconversion)
- [ ] Chlamydomonas GFP strain — $100 (биологически релевантный 2-centriole organism)

---

*E1-Surrogates приёмочный pre-stage для E2 IF Validation Block. См. также `E0/PARAMETERS.md` (hardware), `E0/E2_IF_Validation_Block.md` (real biology IF).*
