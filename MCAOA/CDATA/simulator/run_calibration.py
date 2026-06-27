"""
Финальная калибровка CDATA-v2 — поиск реалистичных параметров
=============================================================
Цель: максимальный Хейфлик с ненулевой амплификацией.
"""
import sys
sys.path.insert(0, '.')

from cdata_sim import CDATAModel, CDATAParams
import numpy as np
import time

print("╔══════════════════════════════════════════╗")
print("║  CDATA-v2: ПОИСК ОПТИМАЛЬНЫХ ПАРАМЕТРОВ║")
print("╚══════════════════════════════════════════╝")
print()

rng = np.random.default_rng(42)

# Целевые ориентиры
TARGET_HAYFLICK = 50.0
TARGET_AMPLIFICATION = 0.20

def sample_params():
    return CDATAParams(
        mu_P=rng.uniform(0.005, 0.05),
        r_0=rng.uniform(0.04, 0.15),
        r_age=rng.uniform(0.0005, 0.005),
        lambda_age=rng.uniform(0.003, 0.03),
        k_cat=rng.uniform(0.04, 0.15),
        mu_s=rng.uniform(0.005, 0.05),
        mu_f=rng.uniform(0.005, 0.05),
        eta_s=rng.uniform(0.005, 0.10),
        eta_f=rng.uniform(0.005, 0.10),
        omega=rng.uniform(0.005, 0.10),
        alpha_CEP=rng.uniform(0.02, 0.15),
        beta_0=rng.uniform(0.02, 0.10),
        alpha_AurA_215=rng.uniform(0.5, 10.0),
        alpha_AurA_315=rng.uniform(0.5, 10.0),
        kappa=rng.uniform(0.05, 0.40),
        sigma_N=rng.uniform(0.3, 1.0),
    )

def evaluate(params, n_cells=50):
    model = CDATAModel(params=params, seed=rng.integers(0, 2**31))
    trees = model.simulate_tree(max_generations=80, n_cells=n_cells)
    stats = model.compute_statistics(trees)
    return stats

# Две фазы
t0 = time.time()
best_params = None
best_score = -1
best_stats = None

for i in range(4000):
    if i < 2000:
        nc = 20
    else:
        nc = 60
    
    params = sample_params()
    stats = evaluate(params, n_cells=nc)
    
    h = stats['hayflick_median']
    amp = stats['amplification_freq']
    
    # Score: hayflick + бонус за амплификацию
    score = h + 20.0 * min(amp, 0.25)
    
    if score > best_score:
        best_score = score
        best_params = params
        best_stats = stats
        if i % 200 == 0 or score > 30:
            print(f"  [{i:4d}] score={score:.1f} hayflick={h:.1f} amp={amp:.3f}")

elapsed = time.time() - t0
print(f"\n✅ {4000} попыток за {elapsed:.1f} сек")

# Финальная валидация
print("\n▸ Финальная валидация (800 клеток)...")
model = CDATAModel(params=best_params, seed=99999)
trees = model.simulate_tree(max_generations=80, n_cells=800)
stats = model.compute_statistics(trees)

print(f"\n╔══════════════════════════════════════════╗")
print(f"║   ОПТИМАЛЬНЫЕ ПАРАМЕТРЫ CDATA-v2      ║")
print(f"╚══════════════════════════════════════════╝")
print(f"  Score:    {best_score:.1f}")
print(f"  Хейфлик:  {stats['hayflick_median']:.1f} (цель: {TARGET_HAYFLICK})")
print(f"  IQR:      {stats['hayflick_iqr']:.1f}")
print(f"  Амплиф:   {stats['amplification_freq']:.3f} (цель: {TARGET_AMPLIFICATION})")

d = best_params.__dict__
print(f"\n# Откалиброванные параметры для CDATAParams:")
for k in sorted(d):
    print(f"    {k}: float = {d[k]:.6f}")
