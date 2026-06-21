# Organismal Aging — DESIGN

**Дата:** 2026-06-21
**Версия:** 1.0

---

## §1. Общая архитектура

```
┌─────────────────────────────────────────────────────────┐
│                   sim_gui (Phoenix LiveView)              │
│                   4D-визуализация (3D + время)            │
├─────────────────────────────────────────────────────────┤
│                   sim_api (Rust, actix-web)               │
│                   REST API / WebSocket                    │
├─────────────────────────────────────────────────────────┤
│                   sim_cli (Rust, clap)                    │
│                   Пакетные симуляции                      │
├─────────────────────────────────────────────────────────┤
│                    sim_core (Rust lib)                    │
│  ┌──────────┬──────────┬──────────┬──────────────────┐   │
│  │ centriole│ counters │  tissue  │    organism      │   │
│  │ (Ур.#1)  │ (Ур.#2)  │ (Ур.#3)  │  (интеграция)    │   │
│  ├──────────┴──────────┴──────────┴──────────────────┤   │
│  │ species │ microbiome │ macrobiome │ learning      │   │
│  └───────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────┤
│              argus_bridge / infogest_bridge               │
│              (физические симуляторы)                      │
└─────────────────────────────────────────────────────────┘
```

---

## §2. Ключевые абстракции (Rust traits)

```rust
/// Счётчик старения (Уровень #2)
trait AgingCounter {
    fn name(&self) -> &str;
    fn damage(&self, divisions: f64, time: f64, ros: f64) -> f64;
    fn critical_threshold(&self, tissue: &Tissue) -> f64;
    fn default_params() -> Self;
}

/// Ткань (Уровень #3)
trait TissueLike {
    fn name(&self) -> &str;
    fn renewal_period(&self, species: &Species) -> f64;
    fn counter_weights(&self) -> [f64; 5];
    fn critical_burden(&self) -> f64;
    fn spatial_position(&self) -> [f64; 3];
}

/// Вид
trait SpeciesLike {
    fn name(&self) -> &str;
    fn max_lifespan(&self) -> f64;
    fn tissues(&self) -> Vec<Box<dyn TissueLike>>;
    fn has_centrioles(&self) -> bool;
    fn counters_active(&self) -> Vec<usize>;
}
```

---

## §3. Поток данных

```
Конфигурация (YAML/JSON)
  │
  ▼
SpeciesConfig → Organism::new(species)
  │
  ▼
Цикл симуляции (t = 0 → T_max, шаг Δt):
  │
  ├─→ centriole::update(dt)         ← S_centriole(t)
  ├─→ counters::update(dt)          ← Dᵢ(t) для всех счётчиков
  ├─→ tissue::update(dt)            ← L_tissue(t), Z_conflict(t)
  ├─→ organism::check_events(dt)    ← Болезни, травмы, смерть
  └─→ learning::maybe_update(dt)    ← Байесовское обновление
  │
  ▼
Результат: AgingCurve { t, S_centriole, L_tissues, Z_conflicts, FI, events }
```

---

## §4. Хранение

- **Симуляции:** SQLite (dev), PostgreSQL (prod)
- **Конфигурации видов:** YAML в `species/configs/`
- **Результаты калибровки:** JSON + графики
- **Логи ARGUS-LP:** отдельная БД робота

---

## §5. Тестирование

| Уровень | Тип тестов | Инструмент |
|---|---|---|
| Модульный | Unit-тесты каждого счётчика | `#[cfg(test)]` |
| Интеграционный | Совместная работа уровней | `tests/integration_tests.rs` |
| Регрессионный | Сравнение с известными данными (Hayflick, FI) | `tests/regression_tests.rs` |
| Бенчмарки | Производительность (120 лет за < 10 мин) | `benches/` (criterion) |
| Property-based | Инварианты (L ∈ [0,1], S монотонна) | `proptest` |

---

## §6. Безопасность

- **FCLC** — конфиденциальные данные пациентов
- **Симулятор** — открытый код (Apache 2.0), данные CC-BY 4.0
- **ARGUS-LP** — BSL-2, лазерная безопасность (Class 3B)

---

*Organismal Aging DESIGN v1.0 — 2026-06-21.*
