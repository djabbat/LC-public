"""
ABC-SMC Calibration Run for CDATA-v2
====================================
Быстрая калибровка с оптимизированными параметрами.
Цель: получить реалистичные параметры (Хейфлик 45-55, амплификация >0.2).
"""
import sys
sys.path.insert(0, '.')

from cdata_sim import CDATAModel, CDATAParams
from cdata_sim.calibration import ABCSMC, ABCConfig
import numpy as np
import time

print("╔══════════════════════════════════════════╗")
print("║   ABC-SMC КАЛИБРОВКА CDATA-v2          ║")
print("╚══════════════════════════════════════════╝")
print()

# Целевые наблюдаемые значения (человеческие фибробласты)
observed_stats = {
    "hayflick_median": 50.0,    # Медиана Хейфлика
    "hayflick_iqr": 10.0,       # IQR
    "amplification_freq": 0.30, # Частота центросомной амплификации
}

# Оптимизированная конфигурация (быстрее)
config = ABCConfig(
    n_populations=5,
    n_particles=200,           # Уменьшено с 1000 для скорости
    tolerance_schedule=[np.inf, 4.0, 2.5, 1.5, 0.8],
    ess_threshold=100,
)

print(f"Конфигурация: {config.n_populations} популяций × {config.n_particles} частиц")
print(f"Целевые значения: {observed_stats}")
print(f"Деревьев на частицу: 10")
print()

abc = ABCSMC(config, observed_stats, seed=42)

t0 = time.time()
posteriors = abc.run(n_trees_per_particle=10, verbose=True)
elapsed = time.time() - t0

print(f"\n✅ Калибровка завершена за {elapsed:.1f} сек ({elapsed/60:.1f} мин)")
print()

# Получение оценок параметров
estimates = abc.get_parameter_estimates()
print("Постериорные оценки (медиана [95% CI]):")
print("-" * 55)
for name, (med, lo, hi) in sorted(estimates.items()):
    print(f"  {name:20s}: {med:8.4f}  [{lo:.4f}, {hi:.4f}]")

# Лучший набор параметров (минимальное расстояние)
final = posteriors[-1]
best_idx = np.argmin(final["distances"])
best_params = final["particles"][best_idx]

print()
print("Лучшие параметры (минимальная дистанция):")
print("-" * 40)
d = best_params.__dict__
for k in sorted(d):
    if not k.startswith('sigma'):
        print(f"  {k:20s}: {d[k]:.6f}")

# Проверка с лучшими параметрами
print()
print("Валидация лучших параметров (500 клеток):")
model = CDATAModel(params=best_params, seed=12345)
trees = model.simulate_tree(max_generations=80, n_cells=500)
stats = model.compute_statistics(trees)
print(f"  Медиана Хейфлика: {stats['hayflick_median']:.1f} (цель: 50.0)")
print(f"  IQR: {stats['hayflick_iqr']:.1f} (цель: 10.0)")
print(f"  Амплификация: {stats['amplification_freq']:.3f} (цель: 0.30)")

# Сохраняем лучшие параметры
print()
print("Для обновления CDATAParams по умолчанию скопируй:")
print()
for k in sorted(d):
    if not k.startswith('sigma'):
        print(f"    {k}: float = {d[k]:.6f}")
