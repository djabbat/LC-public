# TODO.md — Tree v0.1

**Дата:** 2026-03-31
**Статус:** 🔴 Начальная фаза

---

## Фаза 0: Литературный обзор и концептуализация

- [ ] Прочитать `Circles That Mutate.pdf` (Desktop) — выписать ключевые идеи
- [ ] Прочитать `Methods to detect pathological shortening or elongation of circular DNA in human cells.pdf` — выписать методы
- [ ] PubMed: mtDNA lineage tracing + asymmetric stem cell division (2020–2026)
- [ ] PubMed: mitochondria transplantation into stem cells (2020–2026)
- [ ] PubMed: clonal lineage tree reconstruction from mtDNA mutations (scRNA-seq)
- [ ] Синтез: определить наиболее перспективный технический подход
- [ ] Формальная связь Tree ↔ Ontogenesis transitions (встреча / email с коллегами?)

## Фаза 1: Экспериментальный дизайн (in vitro)

- [ ] Выбрать модельную систему (HSC / neural SC / iPSC-производные)
- [ ] Выбрать стратегию введения мутантных митохондрий (микроинъекция / нанотранспорт)
- [ ] Выбрать вариант msDNA для трассировки (природная гетероплазмия / CRISPR point mut)
- [ ] Протокол: ACD in vitro (feeder layer / niche reconstitution)
- [ ] scRNA-seq + mtDNA sequencing pipeline

## Фаза 2: Биоинформатика

- [ ] Lineage tree reconstruction алгоритм (LARRY / LINEAGE / SMALT)
- [ ] Интеграция с Ontogenesis CV/Range transition map
- [ ] Визуализация: 3D-дерево (Ontogenesis engine или D3.js)

## Фаза 3: CDATA-интеграция

- [ ] Валидация: распределение мутантных митохондрий = предсказание Системы II CDATA?
- [ ] Статья: "Mitochondrial Lineage Tree as a Validator of Asymmetric QC during ACD"
- [ ] Добавить в NEEDTOWRITE.md

---

## Входящее из экосистемы

- [ ] Ждём: определение формата Ontogenesis transition nodes → Tree node mapping
