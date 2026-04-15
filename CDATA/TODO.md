# TODO — CDATA
Последнее обновление: 2026-04-15

---

## 🔴 P0 — КРИТИЧНО (до 25 апреля 2026)

### Co-PI outreach (статус 2026-04-15)
- [ ] **Geiger follow-up** — отправить сегодня (письмо от 14.04, ответа нет)
  - `docs/COPI_LETTERS_2026-04-15/LETTER_GEIGER_FOLLOWUP.md`
- [ ] **Jacobsen** — отправить сегодня (приоритет #2)
  - Email: sten.eirik.jacobsen@ki.se
  - `docs/COPI_LETTERS_2026-04-15/LETTER_JACOBSEN.md`
- [ ] **Trumpp** — отправить сегодня (приоритет #3)
  - Email: a.trumpp@dkfz-heidelberg.de
  - `docs/COPI_LETTERS_2026-04-15/LETTER_TRUMPP.md`
- [ ] Дедлайн решения: **20 апреля** — кто первый согласился, тот в LOI
  - Если к 20.04 никто → Nerlov (Oxford) или Bonnet (Crick)

### Aubrey de Grey (LEVF)
- [ ] Встреча: **17 апреля (пятница), 09:00 PT / 20:00 Тбилиси** — Zoom
  - ID: 799 8169 2609 | Passcode: J7aeJ8
  - Link: https://us04web.zoom.us/j/79981692609?pwd=VkhzmKU0Y9Yd4dShEJTbaNjdPsdEgS.1
  - Шпаргалка: `docs/AUBREY_MEETING_2026-04-17/SHPARGALKA_AUBREY_v3.md`
- [ ] Написать подтверждение до встречи — сегодня вечером (короткое письмо)
- [ ] Получить letter of support → deadline **21 апреля**

### LOI финализация
- [ ] После получения co-PI письма → обновить LOI v22 → финальный peer review
- [ ] LOI v22 готов: `docs/AUBREY_MEETING_2026-04-18/LOI_Impetus_v22.docx`
- [ ] Встреча с Ketevan Shashviashvili (TSU) — **сегодня 16:00–17:00**, Google Meet
  - Link: https://meet.google.com/wbx-bpfi-kfu
  - Шпаргалка: `docs/SHASHVIASHVILI_2026-04-15/SHPARGALKA_v2.docx`
  - Запросить: co-investigator статус + письмо поддержки + CV + equipment list

---

## 🟡 P1 — ВАЖНО (май 2026)

- [ ] EIC Pathfinder CommonHealth (дедлайн **12 мая 2026**) — CDATA как experimental subtrack
  - TSU Biology Faculty → LEAR регистрация для EIC (если Shashviashvili согласится)
- [ ] Трёхсторонний call: Jaba + Liz Parrish + co-PI (до 22 апреля)
- [ ] Cell-DT v4.0: D(t)→ep_age интеграция (ABL-2 парадокс fix):
  ```
  ep_age(t) = ep_rate_base × t + k_ep × ∫D(τ)dτ
  ```
- [ ] Добавить Arm 0-CAUSAL бюджет (iCTTLL6 конструкт) в Phase 0 смету

---

## 🟢 P2 — Планово (Q2 2026)

- [ ] **Phase 0** (после финансирования):
  - [ ] Arm 0-CAUSAL: iCTTLL6-GFP (PACT-domain) в молодых LSK → BMT
  - [ ] Уровень 1: GT335 + Ninein → polyGlu asymmetry index
  - [ ] Уровень 2: ARL13B → частота первичных ресничек
  - [ ] Уровень 3: Ki67/EdU + Arm RELAPSE (co-culture P11)
- [ ] **Aging Cell** preparation:
  - [ ] Dataset расширить 28 → 80+ точек
  - [ ] ROS-уравнение исправить (R²(ROS)=-0.512)
  - [ ] Cell-DT v4.0 Sobol на полной Rust-ODE
  - [ ] meiotic_reset PMID (STED GT335 на ооцитах)
  - [ ] BHCA C1+C2 у HSC → Phase 0 данные

---

## ✅ ЗАВЕРШЕНО

- [x] Три аксиомы зафиксированы в CONCEPT.md v5.0 (locked, 2026-04-15)
- [x] LOI v22 создан с Arm 0-CAUSAL + PACT-CCP1 + multi-organism evidence (2026-04-15)
- [x] Ultra-strict peer review LOI v21 (28/100, DO NOT FUND → fatal flaws зафиксированы)
- [x] Peer review v19/v20/v21 история задокументирована
- [x] Phase 0 Arm RELAPSE (P11) добавлен в LOI и CONCEPT.md
- [x] Lavasani/Kovina/Leins/CD150low → CONCEPT.md §Multi-Organism Evidence
- [x] BHCA: 17/27 (Prop 1), ~10/27 (Prop 2) — BHCA убран из LOI (только внутри)
- [x] R²=0.84 изъято (синтетика, 2026-04-13)
- [x] Sobol N=16384, bootstrap CI — S4 закрыт
- [x] Liz Parrish — Industry Co-PI CONFIRMED (2026-04-14)
- [x] Письмо Geiger отправлено (2026-04-14)
- [x] Письма Jacobsen + Trumpp написаны (2026-04-15)
- [x] Шпаргалка Aubrey v3 (встреча 17.04, Zoom) создана
- [x] Шпаргалка Shashviashvili v2 (встреча 15.04, Google Meet) создана
- [x] Все ядерные .md файлы созданы (2026-04-15)
- [x] Git: private + public push выполнен (2026-04-15)
- [x] Три статьи написаны и сохранены на Desktop (2026-04-15)
