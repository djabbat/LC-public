# HAP — Hepato-Affective Primacy Theory

**Эволюционный necessary condition для аффективных состояний у Bilateria.**

HAP утверждает: ни одно билатеральное животное не может обладать аффективными состояниями (эмоции, чувства) без функционального hepatic органа — печени или её гомолога (fat body + nephrocytes у насекомых, hepatopancreas у моллюсков).

## Публикация
Tqemaladze, J. (2026). The Hepato-Affective Primacy (HAP) Theory. *Longevity Horizon*, 2(4). DOI: [10.65649/d76f6c48](https://doi.org/10.65649/d76f6c48)

## Вторая статья (в разработке)
Совместно с Afaf Elfet: nonlinear dynamics модель HAP/NHAM — формализация стероид-пермиссивных feedback loops.

## Симуляция
Python + SciPy, 6-variable ODE system:
- L — Hepatic steroid output
- B — Brain steroid sensitivity
- A — Affective circuit integrity
- I — Inflammatory state
- S — HPA / stress activity
- M — Metabolic state

```bash
cd src/
python3 main.py all   # run simulation + experiments + plots
```

## Статус
- ✅ Первая статья опубликована
- ✅ Прототип симуляции работает (HAP Predictions воспроизводятся)
- ⏳ Ожидание ответа Afaf
- ⏳ Bifurcation analysis
- 📅 Поиск эмпирических данных — после завершения модели
