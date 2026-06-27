# MAP.md — CDATA-v2 Simulator

```
CDATA_simulator/
├── _pi.md                # Правила для pi
├── CONCEPT.md            # Концепт
├── TODO.md               # Задачи
├── PARAMETERS.md         # Параметры модели
├── MAP.md                # Этот файл
├── STATE.md              # Текущий статус
├── MEMORY.md             # История решений
├── README.md             # README (PyPI-совместимый)
├── DESIGN.md             # Архитектура
├── THEORY.md             # Теория
├── EVIDENCE.md           # Доказательная база
├── pyproject.toml        # Python-пакет
├── LICENSE               # GPL v3
├── .gitignore
├── cdata_sim/            # Исходный код
│   ├── __init__.py
│   ├── model.py          # CDATAModel
│   ├── abc_smc.py        # ABC-SMC калибровка
│   ├── gsa.py            # Sobol GSA
│   └── utils.py          # Вспомогательные функции
└── tests/                # Тесты
    ├── test_model.py
    ├── test_abc_smc.py
    └── test_gsa.py
```
