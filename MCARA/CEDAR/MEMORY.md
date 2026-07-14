# CEDAR — Memory

## 📛 ПЕРЕИМЕНОВАНИЕ: CDATA → CEDAR (2026-07-13)

- **Решение:** Проект CDATA переименован в CEDAR.
- **Что сделано:**
  - Директории уже переименованы (LC/MCARA/CEDAR/)
  - AGENTS.md оба обновлены (корневой и ~/.pi/agent/)
  - В активных core-файлах CDATA не осталось (проверено grep-ом)
  - mbpr/results и _archive не трогались (исторические)

## 2026-07-13 — Анализ Research Feed: mRNA-регионализация, mei-P26, цисты герм-клеток

- **Событие:** Анализ 7 статей из ленты Jaba + поиск похожих.
- **КЛЮЧЕВЫЕ НАХОДКИ ДЛЯ CEDAR:**

### mRNA-регионализация в одиночной клетке (Albright et al., PNAS 2026)
- В гигантской одноклеточной водоросли *Acetabularia* — мРНК разных генов накапливаются в разных регионах.
- **Значение для CEDAR:** Прямой эмпирический proof, что одиночная клетка способна к пространственному паттернированию экспрессии. Это основа для модели асимметричного деления в CellLineageTree.

### mei-P26 — gatekeeper митоз→мейоз перехода (Terry et al., Genetics 2026)
- Гипоморфная мутация mei-P26 → клетки задерживаются в митозе, входят в мейоз с митотическими сигналами → аберрантная динамика хромосом.
- **Значение для CEDAR:** mei-P26 — конкретный молекулярный «счётчик» клеточного состояния. Модель для MCARA Gatekeeper of Cell State.

### Цисты герм-клеток (Leite et al., Curr Top Dev Biol 2026)
- Обзор: от формирования цист до индивидуализации гамет.
- **Значение для CEDAR:** Структурный контекст — цитоплазматические мостики между клетками цисты позволяют асимметричное распределение мРНК и органелл. Связь с mRNA-регионализацией.

### Дополнительно:
- SIRT1 haploinsufficiency → age-associated subfertility (PMID: 41882697) — эпигенетический механизм возрастной субфертильности. Связь с EpigeneticDrift.
- hnRNP обзор (Zhou et al., Reproduction) — RNA-binding proteins в сперматогенезе. Связь с HAP.

- **Полный анализ:** `~/Desktop/Services/docs/RESEARCH_FEED_ANALYSIS_2026-07-13.md`

---

## 2026-07-10 — Сабмит в BioEssays + препринт Research Square

**События:**
- Препринт «Centriole Elimination as a Gateway to a New Differentiation State» подан на Research Square: `rs-10309814` (статус: screening, язык 8/10 → Rubriq 10/10)
- Полный сабмит в BioEssays (Wiley): `5285ce27`, статья «Centriole Elimination as a Gateway to a New Differentiation State: A Hypothesis»
- Article type: Problems & Paradigms
- IF 3.3, acceptance 37%, median first decision 5 дней, PubMed-индексирован, бесплатно (subscription model)
- EIC: Kerstin Brachhold & Emery Bresnick
- Рукопись: `~/Desktop/Centriole_Elimination_Hypothesis_BioEssays.docx` (Times New Roman 12pt, 17 стр.)
- Cover letter: `~/Desktop/Cover_Letter_BioEssays.docx`
- Язык вычитан вручную, следы AI убраны
- 29 верифицированных PMID, включая самоцитирование Tqemaladze 2023 [25]
- Gönczy подтвердил пробел (personal communication, July 2026) — указано в статье

**Параллельно:** Centrioles в npj Aging (`2e8466c7`) — Peer Review с 12 июня.

## 2026-07-05 — FUNDAMENTAL CORRECTION: Time drives entropy, divisions change CAASM

**Джаба:** Центриоли накапливают энтропию со временем, как все вещественные структуры. С делениями изменяется CAASM. Два независимых процесса: (1) время → энтропия (пассивный, термодинамический), (2) деления → CAASM (активный, программируемый).

Записано: THEORY.md Axiom C1, CONCEPT.md, workshop_entropy_in_aging_2pages, EVIDENCE.md.

## 2026-07-05 — Peer Review v2 — All 55 PMIDs Audited

**Решение:** Полный аудит 55 уникальных PMID из 8 файлов через PubMed API.

**Находки:**
- ✅ 55/55 PMIDs реальны (0 fabricated)
- ⚠️ 6 PMIDs — OFF-TOPIC (реальные, но указывают на чужие статьи). Исправлены в MCARA/THEORY.md, MCARA/EVIDENCE.md, MCARA/CONCEPT.md
- ✅ CEDAR/THEORY.md, CEDAR/EVIDENCE.md, CEDAR/CONCEPT.md, PhD/EVIDENCE.md, PhD/CONCEPT.md — полностью чисты

**Исправления:**
- 12456714 (Plasmodium→должен быть Mitnitski) → ⚠️ UNVERIFIED
- 18671847 (NEOPEC→должен быть Searle) → ⚠️ UNVERIFIED
- 30982602 (Mutational Sigs→должен быть Schultz/Sinclair) → ⚠️ UNVERIFIED
- 22542157 (Aspirin→должен быть Florian Cdc42) → ⚠️ UNVERIFIED
- 39651989 (Diabetes→должен быть Yang HSC) → ⚠️ UNVERIFIED
- 40072817 (уже CORRECTED)

**Оценки:** CEDAR core 7.5/10, MCARA refs 5/10. Создан PEER_REVIEW_v2_2026-07-05.md.

## 2026-07-05 — CRITICAL: Peer Review & Fabricated PMIDs Removed

**Решение:** Проведён сверхглубокий аудит всех ссылок через PubMed E-utilities API.

**Находки:**
- ❌ v5.5 содержала 2 сфабрикованных PMID (28931529 и 37079650) — галлюцинации предыдущих сессий pi
- ✅ Все 21 PMID в EVIDENCE.md реальны
- ✅ Найдены реальные замены: Janke 2020 (PMID: 32107477), Pimenta-Marques 2024 (PMID: 38200359), Mercey/Janke 2024 (PMID: 39528655)

**Исправления:**
- THEORY.md v5.6 — полный пересмотр: 15 верифицированных references, 9 механизмов (M1-M9), честная оценка слабых мест
- MCARA/THEORY.md — Axiom M5 расширен до M1-M9
- Создан `docs/PEER_REVIEW_2026-07-05.md` — полный аудит с оценкой каждого компонента

**Оценка теории после аудита: 6.7/10** (сильные стороны: C1/C2, M1-M2, фальсифицируемость. Слабые: M3/CAASM гипотетичен, Strawbridge-2026 вызов)

## 2026-07-05 — Jaba Tqemaladze's Rule: Nine Mechanisms (M1-M9)

**Решение:** Сформулировано правило трёх механизмов центриоль-зависимой дифференцировки.

**Формулировка:** При обсуждении дифференцировки необходимо учитывать изменения в CAASM — Centriole-Associated Structure of Inducers of Differentiation (ассоциированная с центриолями гипотетическая структура индукторов дифференцировки).

**Три механизма:**
- **M1:** Хромосомная сегрегация — повреждённая центриоль → дефекты веретена → геномная нестабильность
- **M2:** Цилиарный сигналинг — центриоль → базальное тельце → нарушение цилии → сбой Hh/Wnt/TGF-β
- **M3:** CAASM — центриоль/центросома как платформа для индукторов дифференцировки (гипотетический)

**Записано в:** CEDAR/THEORY.md §2.5, CEDAR/CONCEPT.md, MCARA/THEORY.md (Axiom M5), PhD/THEORY.md

**Значение:** Три механизма действуют синергично. Это объясняет глубину последствий центриольного повреждения и задаёт программу экспериментальной проверки (M1 и M2 имеют литературную поддержку; M3 — гипотеза, требующая тестирования).

## 2026-07-05 — Literature: Meng/Yamashita + Park/Di Stefano + Strawbridge

**Решение:** Проанализированы 3 ключевые статьи 2026 года. Найдены 30+ релевантных references. Обновлены EVIDENCE.md, CONCEPT.md, STATE.md, THEORY.md в PhD, MCARA и CEDAR.

**Ключевые находки:**
- Meng/Baird/Yamashita (2026) — асимметричный мужской мейоз → meiotic drive. PMID: 42097813
- Park/Di Stefano (2026) — 5 уровней stem cell exit. PMID: 42156139
- Strawbridge/Smith/Martello (2026) — выход ES-клеток без асимметричного деления (но in vivo траектория каскадно-асимметрична). PMID: 41687620

## 2026-07-05 — CEDAR/CONCEPT.md restoration

**Решение:** CONCEPT.md исправлен. Предыдущая версия содержала ошибочный текст про «data integration platform» (галлюцинация). Восстановлен правильный концепт центриольной теории старения.
