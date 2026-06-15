# PARAMETERS — HAP Project

## Модель симуляции

### State variables
| Symbol | Variable | Default | Range | Unit |
|--------|----------|---------|-------|------|
| L | Hepatic steroid output | 0.1 | [0, 10] | nM |
| B | Brain steroid sensitivity | 0.1 | [0, 1] | a.u. |
| A | Affective circuit integrity | 0.0 | [0, 1] | a.u. |
| I | Inflammatory state | 0.1 | [0, ∞) | a.u. |
| S | HPA / stress activity | 0.2 | [0, ∞) | nM |
| M | Metabolic state | 1.0 | [0.3, ∞) | mM |

### Ключевые параметры
| Параметр | Значение | Описание |
|----------|---------|----------|
| τ_crit | 72 hpf | Конец критического developmental window |
| L_basal | 1.0 nM | Базальный стероидный выход печени |
| k_A_L | 0.3 | Зависимость аффективных цепей от L |
| k_A_B | 0.4 | Зависимость от B |
| I_suppress_L | 0.3 | Подавление L воспалением |
| S_enhance_L | 0.2 | Усиление L стрессом (аллостаз) |

## Параметры проекта

### Бюджет
- Нет внешнего финансирования (на данный момент)
- Время: свободное (работа с Afaf — коллаборация)

### Контакты
- Jaba Tqemaladze: jaba@longevity.ge
- Afaf Elfet: через email

### Инструменты
- Python 3.10 + SciPy/NumPy/Matplotlib (симуляция)
- Rust (Cargo) — если потребуется производительность
- GitHub: djabbat/HAP (TODO)

### Лицензия
- Публикация: открытый доступ (Longevity Horizon, Gold OA)
- Код: MIT / CC-BY (TODO)
