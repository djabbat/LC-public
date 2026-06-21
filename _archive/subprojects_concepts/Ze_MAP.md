# Ze — MAP

**Дата:** 2026-06-11

---

## Структура проекта

```
Ze/
├── _pi.md            — Правила pi
├── CONCEPT.md        — Концепт (Ze Vectors Theory)
├── PARAMETERS.md     — Ключевые параметры (v, τ, Z, χ)
├── MAP.md            — Эта карта
├── MEMORY.md         — История решений
├── STATE.md          — Текущее состояние
├── TODO.md           — Задачи
├── README.md         — Введение
│
├── Ze_Model/         — Теоретическая модель (Found. of Physics)
├── Ze_CHSH/          — CHSH-неравенство (QSMF)
├── Ze_D/             — Многослойный возраст (Physica A)
├── Ze-Hierarchy/     — Возрастная иерархия в bristlebot-роях (NLnet)
│
├── simulator/        — Rust-симулятор (ze-core + ze-runner)
├── website/          — Интерактивный digital twin (Phoenix)
│   └── ze_sim/       — Симуляция на сайте
│
├── Articles/         — Опубликованные статьи
├── docs/             — Документация
├── grants/           — Грантовые заявки
├── refs/             — Ссылки и источники
└── audits/           — Аудиты
```

## Зависимости

```
Ze_Model ──→ Ze_CHSH ──→ Ze_D ──→ Ze-Hierarchy
    │            │
    └────────────┴──→ simulator ──→ website
```

## Ключевые выходы

- 42 статьи опубликовано
- 4 статьи поданы в журналы (июнь 2026)
- 1 грант NLNet принят (Ze-Hierarchy)
- Симулятор на Rust (open source)
