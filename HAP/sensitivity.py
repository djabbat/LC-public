#!/usr/bin/env python3
"""
sensitivity.py — Глобальный анализ чувствительности HAP-модели (Morris + Sobol).
Без NHAM.
"""

import numpy as np
import os, json, sys, signal
from scipy.integrate import solve_ivp

# ── Timeout для защиты от зависаний ──
class TimeoutError(Exception):
    pass

class timeout:
    def __init__(self, seconds):
        self.seconds = seconds
    def __enter__(self):
        signal.signal(signal.SIGALRM, self._handler)
        signal.alarm(self.seconds)
        return self
    def __exit__(self, *args):
        signal.alarm(0)
    def _handler(self, signum, frame):
        raise TimeoutError('Timeout')

# ── Параметры модели ──
PARAMS = {
    'L_basal': 1.0,       # Базальный стероидный выход
    'L_capacity': 2.0,    # Максимальная ёмкость
    'k_S_L': 0.5,         # S→L (стресс усиливает стероиды)
    'k_I_L': 0.3,         # I→L (воспаление подавляет)
    'gamma_L': 0.2,       # Деградация L
    'k_B_L': 0.8,         # B развивается от L
    'theta_L': 0.5,       # Порог L для развития B
    'gamma_B': 0.1,       # Деградация B
    'k_A_B': 1.0,         # A развивается от B
    'gamma_A': 0.1,       # Деградация A
    'k_L_S': 0.3,         # L→S (стероиды активируют HPA)
    'k_A_S': 0.1,         # A→S (аффект → стресс)
    'k_S_M': 0.2,         # S→M (стресс → метаболизм)
    'k_L_M': 0.3,         # L→M (стероиды → метаболизм)
    'I_basal': 0.1,       # Базальное воспаление
    'k_M_I': 0.2,         # M→I (метаболизм → воспаление)
    'gamma_I': 0.2,       # Деградация I
    'gamma_S': 0.2,       # Деградация S
    'M_basal': 0.5,       # Базальный метаболизм
    'gamma_M': 0.15,      # Деградация M
}

# ── ODE модель ──
def hap_model(t, y, params, noise_std=0.0):
    """6-variable ODE модель HAP. noise_std — амплитуда стохастического шума."""
    L, B, A, I, S, M = np.clip(y, 0, 1e6)

    dev_signal = 1.0 / (1.0 + np.exp(np.clip(-10.0 * (L - params['theta_L']), -100, 100)))
    S_norm = S / (1.0 + S)
    I_norm = I / (1.0 + I)

    L_sat = np.clip(1.0 - L / params['L_capacity'], 0, 1)
    dL = (params['L_basal'] * L_sat
          + params['k_S_L'] * S_norm * L_sat
          - params['k_I_L'] * I_norm * L
          - params['gamma_L'] * L)

    B_sat = np.clip(1.0 - B, 0, 1)
    dB = (params['k_B_L'] * dev_signal * B_sat - params['gamma_B'] * B)

    A_sat = np.clip(1.0 - A, 0, 1)
    dA = (params['k_A_B'] * A_sat * B * dev_signal - params['gamma_A'] * A)

    dI = (params['I_basal'] + params['k_M_I'] * M - params['gamma_I'] * I)
    dS = (params['k_L_S'] * L + params['k_A_S'] * A - params['gamma_S'] * S)
    dM = (params['M_basal'] + params['k_L_M'] * L + params['k_S_M'] * S_norm - params['gamma_M'] * M)

    derivs = np.array([dL, dB, dA, dI, dS, dM])

    # Стохастический шум (мультипликативный — пропорционален значению)
    if noise_std > 0:
        y_arr = np.array([L, B, A, I, S, M])
        noise = np.random.normal(0, noise_std, 6) * np.abs(y_arr)
        derivs += noise

    return derivs

# ── Симуляция ──
def simulate(params, T=200, dt=0.1, t_ablate=None, noise_std=0.0):
    y0 = [0.1, 0.01, 0.01, 0.1, 0.1, 0.5]

    def ode_func(t, y):
        if t_ablate is not None and t >= t_ablate:
            y_local = list(y)
            y_local[0] = 0.0
            return hap_model(t, y_local, params, noise_std)
        return hap_model(t, y, params, noise_std)

    sol = solve_ivp(ode_func, (0, T), y0, method='LSODA',
                    max_step=dt, rtol=1e-3, atol=1e-6)
    return {
        'time': sol.t, 'L': sol.y[0], 'B': sol.y[1], 'A': sol.y[2],
        'I': sol.y[3], 'S': sol.y[4], 'M': sol.y[5],
        'A_final': sol.y[2][-1],
    }

# ── Morris ──
def run_morris(n_levels=4, n_trajectories=20, seed=42):
    from SALib.sample import morris as morris_sample
    from SALib.analyze import morris as morris_analyze

    names = list(PARAMS.keys())
    bounds = [[v * 0.1, v * 10.0] for v in PARAMS.values()]
    problem = {'num_vars': len(names), 'names': names, 'bounds': bounds}

    np.random.seed(seed)
    X = morris_sample.sample(problem, n_trajectories, n_levels)
    Y = np.zeros(X.shape[0])

    print(f"  Morris: {X.shape[0]} runs...")
    for i in range(X.shape[0]):
        if i % 50 == 0:
            print(f"    {i}/{X.shape[0]}...")
        p = dict(PARAMS)
        for j, n in enumerate(names):
            p[n] = X[i, j]
        try:
            with timeout(5):
                res = simulate(p, T=100)
            Y[i] = res['A_final']
        except Exception:
            Y[i] = 0.0

    return morris_analyze.analyze(problem, X, Y, conf_level=0.95), names

# ── Sobol (быстрый) ──
def run_sobol(n_samples=128, seed=42):
    from SALib.sample import sobol as sobol_sample
    from SALib.analyze import sobol as sobol_analyze

    names = list(PARAMS.keys())
    bounds = [[v * 0.1, v * 10.0] for v in PARAMS.values()]
    problem = {'num_vars': len(names), 'names': names, 'bounds': bounds}

    D = len(names)
    N = n_samples
    total = N * (2 * D + 2)
    print(f"  Sobol: {N}×{D} = ~{total:,} runs...")

    X = sobol_sample.sample(problem, N, calc_second_order=False)
    Y = np.zeros(X.shape[0])

    for i in range(X.shape[0]):
        if i % 1000 == 0:
            print(f"    {i}/{X.shape[0]}...")
        p = dict(PARAMS)
        for j, n in enumerate(names):
            p[n] = X[i, j]
        try:
            with timeout(5):
                res = simulate(p, T=100)
            Y[i] = res['A_final']
        except Exception:
            Y[i] = 0.0

    return sobol_analyze.analyze(problem, Y, calc_second_order=False), names

# ── Визуализация ──
def plot_morris(Si, names, path='morris.png'):
    import matplotlib.pyplot as plt
    mu_star = Si['mu_star']
    sigma = Si['sigma']
    short = [n[:14] for n in names]
    idx = np.argsort(mu_star)

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 5))
    ax1.barh(range(len(idx)), mu_star[idx])
    ax1.set_yticks(range(len(idx)))
    ax1.set_yticklabels([short[i] for i in idx])
    ax1.set_xlabel('μ* (важность)')
    ax1.set_title('Morris — важность параметров')

    ax2.scatter(mu_star, sigma)
    for i, n in enumerate(short):
        ax2.annotate(n, (mu_star[i], sigma[i]), fontsize=7)
    ax2.set_xlabel('μ* (важность)')
    ax2.set_ylabel('σ (нелинейность)')
    ax2.set_title('Morris — μ* vs σ')
    plt.tight_layout()
    plt.savefig(path, dpi=150)
    print(f"  Saved: {path}")
    plt.close()

def plot_sobol(Si, names, path='sobol.png'):
    import matplotlib.pyplot as plt
    S1 = Si['S1']
    ST = Si['ST']
    short = [n[:14] for n in names]
    idx = np.argsort(ST)

    fig, ax = plt.subplots(figsize=(12, 6))
    ax.bar(range(len(idx)), ST[idx], alpha=0.5, label='ST (total)', color='orange')
    ax.bar(range(len(idx)), S1[idx], alpha=0.8, label='S1 (first-order)', color='steelblue')
    ax.set_xticks(range(len(idx)))
    ax.set_xticklabels([short[i] for i in idx], rotation=45, ha='right')
    ax.set_ylabel('Sensitivity index')
    ax.set_title('Sobol — Global Sensitivity')
    ax.legend()
    ax.axhline(0, color='gray', lw=0.5)
    plt.tight_layout()
    plt.savefig(path, dpi=150)
    print(f"  Saved: {path}")
    plt.close()

# ── Stochastic parameter perturbation ──
def run_stochastic_params(n_runs=30, noise_pct=0.05, path='stochastic_params.png'):
    """
    Стохастичность параметров: каждый run — параметры ±noise_pct%.
    Биологически: реальные системы имеют разброс параметров между индивидами.
    """
    import matplotlib.pyplot as plt

    fig, axes = plt.subplots(1, 2, figsize=(14, 5))

    # Детерминированная базовая линия
    res0 = simulate(PARAMS, T=200)
    axes[0].plot(res0['time'], res0['A'], 'b-', linewidth=2)
    axes[0].set_title('A(t) — детерминированная'); axes[0].set_xlabel('Time'); axes[0].set_ylabel('A')
    axes[0].grid(alpha=0.3)

    # Стохастические параметры
    finals = []
    for i in range(n_runs):
        p = dict(PARAMS)
        for k, v in PARAMS.items():
            p[k] = v * (1 + np.random.normal(0, noise_pct))
            p[k] = max(p[k], 0.001)  # не уходим в 0
        res = simulate(p, T=200)
        axes[1].plot(res['time'], res['A'], alpha=0.25, linewidth=0.5, color='red')
        finals.append(res['A_final'])

    finals = np.array(finals)
    axes[1].set_title(f'A(t) — параметры ±{noise_pct*100:.0f}% ({n_runs} runs)')
    axes[1].set_xlabel('Time'); axes[1].set_ylabel('A')
    axes[1].grid(alpha=0.3)

    plt.tight_layout()
    plt.savefig(path, dpi=150)
    print(f"  Saved: {path}")
    plt.close()

    print(f"\n  Стохастичность параметров ±{noise_pct*100:.0f}%, {n_runs} runs:")
    print(f"    A_final: mean={finals.mean():.4f}, std={finals.std():.4f}, "
          f"min={finals.min():.4f}, max={finals.max():.4f}")
    print(f"    Robustness = 1 - CV = {1 - finals.std()/finals.mean():.4f}")

# ── Главный запуск ──
def main(output_dir='./sensitivity_results'):
    os.makedirs(output_dir, exist_ok=True)

    print("=" * 60)
    print("HAP — SENSITIVITY ANALYSIS")
    print("=" * 60)

    # 1. Morris
    print("\n[1/4] Morris Sensitivity...")
    Si_m, names = run_morris(n_levels=4, n_trajectories=20)
    plot_morris(Si_m, names, os.path.join(output_dir, 'morris.png'))
    top5 = np.argsort(Si_m['mu_star'])[-5:][::-1]
    print("  Top-5 по μ*:")
    for i, idx in enumerate(top5):
        print(f"    {i+1}. {names[idx]}: μ*={Si_m['mu_star'][idx]:.4f}")

    # 2. Sobol (быстрый)
    print("\n[2/4] Sobol Sensitivity (N=128)...")
    Si_s, names = run_sobol(n_samples=128)
    plot_sobol(Si_s, names, os.path.join(output_dir, 'sobol.png'))
    top5 = np.argsort(Si_s['ST'])[-5:][::-1]
    print("  Top-5 по ST:")
    for i, idx in enumerate(top5):
        print(f"    {i+1}. {names[idx]}: ST={Si_s['ST'][idx]:.4f}")

    # Сохраняем JSON
    results = {}
    for i, n in enumerate(names):
        results[n] = {
            'S1': float(Si_s['S1'][i]),
            'ST': float(Si_s['ST'][i]),
            'mu_star': float(Si_m['mu_star'][i]),
            'sigma': float(Si_m['sigma'][i]),
        }
    with open(os.path.join(output_dir, 'sensitivity.json'), 'w') as f:
        json.dump(results, f, indent=2)

    # 3. Stochastic parameter perturbation
    print("\n[3/4] Стохастичность параметров...")
    run_stochastic_params(n_runs=50, noise_pct=0.05,
                          path=os.path.join(output_dir, 'stochastic_params.png'))

    # 4. Сохраняем параметры
    with open(os.path.join(output_dir, 'params.json'), 'w') as f:
        json.dump(PARAMS, f, indent=2)

    print(f"\n{'='*60}")
    print(f"✅ Всё сохранено в: {output_dir}")
    print(f"{'='*60}")

if __name__ == '__main__':
    out = sys.argv[1] if len(sys.argv) > 1 else './sensitivity_results'
    main(out)
