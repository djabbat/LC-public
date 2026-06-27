"""
Быстрая калибровка CDATA-v2 методом Монте-Карло
================================================
Случайный поиск по сетке параметров с отбором по расстоянию.
"""
import sys
sys.path.insert(0, '.')

from cdata_sim import CDATAModel, CDATAParams
import numpy as np
import time

print("╔══════════════════════════════════════════╗")
print("║  БЫСТРАЯ КАЛИБРОВКА CDATA-v2 (MC)      ║")
print("╚══════════════════════════════════════════╝")
print()

rng = np.random.default_rng(42)

# Целевые значения
TARGET = {
    "hayflick_median": 50.0,
    "hayflick_iqr": 10.0,
    "amplification_freq": 0.30,
}

# Веса для метрики
WEIGHTS = {"hayflick_median": 1.0, "hayflick_iqr": 0.5, "amplification_freq": 0.5}

def sample_params():
    """Случайная выборка параметров."""
    return CDATAParams(
        mu_P=rng.uniform(0.005, 0.05),
        r_0=rng.uniform(0.04, 0.15),
        r_age=rng.uniform(0.0005, 0.008),
        lambda_age=rng.uniform(0.003, 0.05),
        k_cat=rng.uniform(0.03, 0.15),
        mu_s=rng.uniform(0.005, 0.08),
        mu_f=rng.uniform(0.005, 0.08),
        eta_s=rng.uniform(0.005, 0.15),
        eta_f=rng.uniform(0.005, 0.15),
        omega=rng.uniform(0.005, 0.15),
        alpha_CEP=rng.uniform(0.01, 0.15),
        beta_0=rng.uniform(0.01, 0.10),
        alpha_AurA_215=rng.uniform(0.3, 12.0),
        alpha_AurA_315=rng.uniform(0.3, 12.0),
        kappa=rng.uniform(0.03, 0.40),
    )

def evaluate(params, n_cells=30):
    """Оценка одного набора параметров."""
    model = CDATAModel(params=params, seed=rng.integers(0, 2**31))
    trees = model.simulate_tree(max_generations=80, n_cells=n_cells)
    stats = model.compute_statistics(trees)
    
    # Расстояние до цели
    dist = 0.0
    for k in TARGET:
        w = WEIGHTS.get(k, 1.0)
        dist += w * (TARGET[k] - stats.get(k, 0.0)) ** 2
    return np.sqrt(dist), stats

# Три фазы с увеличивающимся числом клеток
N_TRIALS = 2000
best_params = None
best_dist = np.inf
best_stats = None

t0 = time.time()
for i in range(N_TRIALS):
    # Фаза 1 (первые 500): быстрая оценка (10 клеток)
    # Фаза 2 (500-1000): средняя (30 клеток)
    # Фаза 3 (1000+): точная (80 клеток)
    if i < 500:
        nc = 10
    elif i < 1200:
        nc = 30
    else:
        nc = 80
    
    params = sample_params()
    dist, stats = evaluate(params, n_cells=nc)
    
    if dist < best_dist:
        best_dist = dist
        best_params = params
        best_stats = stats
        if i % 100 == 0 or dist < 3.0:
            print(f"  [{i:4d}] dist={dist:.3f} hayflick={stats['hayflick_median']:.1f} "
                  f"iqr={stats['hayflick_iqr']:.1f} amp={stats['amplification_freq']:.3f}")

elapsed = time.time() - t0
print(f"\n✅ {N_TRIALS} попыток за {elapsed:.1f} сек")

# Финальная валидация на 500 клетках
print("\n▸ Финальная валидация (500 клеток)...")
model = CDATAModel(params=best_params, seed=99999)
trees = model.simulate_tree(max_generations=80, n_cells=500)
stats = model.compute_statistics(trees)

print(f"\n╔══════════════════════════════════════════╗")
print(f"║   РЕЗУЛЬТАТЫ КАЛИБРОВКИ               ║")
print(f"╚══════════════════════════════════════════╝")
print(f"  Расстояние до цели: {best_dist:.3f}")
print(f"  Хейфлик: {stats['hayflick_median']:.1f} (цель: {TARGET['hayflick_median']})")
print(f"  IQR:     {stats['hayflick_iqr']:.1f} (цель: {TARGET['hayflick_iqr']})")
print(f"  Амплиф:  {stats['amplification_freq']:.3f} (цель: {TARGET['amplification_freq']})")

# Параметры для копирования
print(f"\n# Откалиброванные параметры (скопировать в CDATAParams):")
d = best_params.__dict__
for k in sorted(d):
    if not k.startswith('sigma'):
        print(f"    {k}: float = {d[k]:.6f}")
