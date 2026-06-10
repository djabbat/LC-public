# STATE — BioSense

**Дата:** 2026-06-11
**Версия:** 2.0 (аудит — заменён фиктивный AI-шаблон от 2025-04-01)

---

## Текущий статус: 🟢 В разработке (реальный)

| Компонент | Статус | Детали |
|-----------|:------:|--------|
| Бэкенд (Rust) | ✅ Работает | Порт :4101, healthz: http://127.0.0.1:4101/healthz |
| Phoenix LiveView | ✅ Работает | Порт :4100, http://127.0.0.1:4100 |
| Datasets registry | ✅ Работает | 12 датасетов на :4100/datasets |
| EEG Cuban data | ✅ Загружены | FirstWaveCubanHumanNormativeEEGProject |
| Ze EEG validation | 🔄 В процессе | ze_eeg_validation/ |
| Automated microscopy | 🔄 В процессе | instruments/automated-microscopy/ |
| γ velocity convention | ✅ Применена | Из LC CONCEPT v5.6 |

---

## Сервисы (из LC STATE.md)

| Сервис | Статус | URL |
|--------|:------:|-----|
| BioSense backend | up :4101 | http://127.0.0.1:4101/healthz |
| BioSense Phoenix | up :4100 | http://127.0.0.1:4100 |
| BioSense /datasets | up :4100/datasets | 12 datasets registry |

---

## Архитектура

BioSense — платформа федеративного клинического обучения (federated clinical learning) в составе LongevityCommon (LC). Назначение:
- Сбор и анализ биомедицинских данных (ЭЭГ, микроскопия)
- Валидация Ze-метрик на реальных данных
- EEG-компонент: кубинский нормативный датасет
- Инструментальный компонент: automated microscopy pipeline

---

## Подпроекты

| Подпроект | Статус |
|-----------|:------:|
| `instruments/automated-microscopy/` | 🟡 В разработке (есть _pi.md) |
| `data/cuban/FirstWaveCubanHumanNormativeEEGProject/` | ✅ Данные загружены (есть _pi.md) |
| `ze_eeg_validation/` | 🔄 Валидация Ze на EEG |
| `biosense-web/` | 🟡 Phoenix LiveView |
| `src/` | ✅ Rust-бэкенд |

---

## Код, требующий изменений (из DESIGN.md §5.1 после CONCEPT v5.6)

- [ ] `handlers/biosense.rs` — заголовок `X-LC-Status: hypothesis-stage-exploratory`
- [ ] BioSense live stream channel — метаданные `{disclosure: "exploratory"}`
- [ ] Кросс-проверка CONCEPT.md с зонтичным LC CONCEPT v5.6

---

## Ближайшие шаги

1. Завершить Ze EEG validation
2. Automated microscopy: доработать инструментальный пайплайн
3. Обновить disclosures (гипотезный статус) в коде
4. Синхронизировать CONCEPT.md с актуальным состоянием

---

*Предыдущая версия STATE.md от 2025-04-01 была фиктивным AI-шаблоном с вымышленной командой (Ivanov, Petrov, Sidorova, Sergeev) — полностью удалена 2026-06-11.*
