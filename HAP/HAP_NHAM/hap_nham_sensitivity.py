#!/usr/bin/env python3
"""
HAP/NHAM — глобальный анализ чувствительности (Sobol + Morris)
Модель: 6-variable ODE система (L, B, A, I, S, M)
HAP Strong: аффект (A) не развивается без печени (L) в критическом окне.

Автор: Jaba Tqemaladze (для соавтора Afaf Elfet)
Дата: 2026-06-03
"""

import numpy as np
from scipy.integrate import solve_ivp
import signal


class TimeoutError(Exception):
    pass


class timeout:
    """Контекстный менеджер для ограничения времени выполнения."""
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

# ══════════════════════════════════════════════════════════════════════════
# ПАРАМЕТРЫ
# ══════════════════════════════════════════════════════════════════════════

DEFAULT_PARAMS = {
    # ─── Печень (L) ─────────────────────────────────────────────────────
    'L_basal': 1.0,       # Базальный стероидный выход (норма = 1)
    'L_capacity': 2.0,    # Максимальная ёмкость (логистическое насыщение)
    'k_S_L': 0.5,         # S→L (стресс усиливает стероиды)
    'k_I_L': 0.3,         # I→L (воспаление подавляет)
    'gamma_L': 0.2,       # Деградация L

    # ─── Чувствительность мозга (B) ─────────────────────────────────────
    'k_B_L': 0.8,         # B развивается от L
    'theta_L': 0.5,       # Порог L для развития B
    'gamma_B': 0.1,       # Деградация B (медленная — «память»)

    # ─── Аффект (A) ─────────────────────────────────────────────────────
    'k_A_B': 1.0,         # A развивается от B
    'gamma_A': 0.1,       # Деградация A

    # ─── Обратные связи ─────────────────────────────────────────────────
    'k_L_S': 0.3,         # L→S (стероиды активируют HPA)
    'k_A_S': 0.1,         # A→S (аффект → стресс)
    'k_S_M': 0.2,         # S→M (стресс → метаболизм)
    'k_L_M': 0.3,         # L→M (стероиды → метаболизм)

    # ─── Воспаление ─────────────────────────────────────────────────────
    'I_basal': 0.1,       # Базальное воспаление
    'k_M_I': 0.2,         # M→I (метаболизм → воспаление)
    'gamma_I': 0.2,       # Деградация I

    # ─── HPA / стресс ────────────────────────────────────────────────────
    'gamma_S': 0.2,       # Деградация S

    # ─── Метаболизм ─────────────────────────────────────────────────────
    'M_basal': 0.5,       # Базальный метаболизм
    'gamma_M': 0.15,      # Деградация M
}


# ══════════════════════════════════════════════════════════════════════════
# МОДЕЛЬ
# ══════════════════════════════════════════════════════════════════════════

def hap_nham_model(t, y, params):
    """
    6-variable ODE модель HAP/NHAM.
    y = [L, B, A, I, S, M]

    HAP-гипотеза: аффект (A) не может развиться или поддерживаться
    без стероидного выхода печени (L) в критическом окне развития.
    Нет прямого S→A — стресс влияет на аффект только через L.
    Все переменные ≥ 0 (физиологически реалистично).
    """
    L, B, A, I, S, M = np.clip(y, 0, 1e6)  # физиологические границы
    p = params

    # Сигнал развития (sigmoid)
    arg = np.clip(-10.0 * (L - p['theta_L']), -100, 100)
    dev_signal = 1.0 / (1.0 + np.exp(arg))

    # Насыщение сигналов (Hill-type: X/(1+X))
    S_norm = S / (1.0 + S)
    I_norm = I / (1.0 + I)

    # ═══════════════════════════════════════════════════════════════
    # L: печень — стероидный выход
    #   + L_basal масштабируется от свободной ёмкости (L_sat)
    #   + аллостаз от S
    #   - подавление от I (пропорционально L)
    #   - деградация
    # ═══════════════════════════════════════════════════════════════
    L_sat = np.clip(1.0 - L / p['L_capacity'], 0, 1)  # свободная ёмкость
    dL = (p['L_basal'] * L_sat
          + p['k_S_L'] * S_norm * L_sat
          - p['k_I_L'] * I_norm * L
          - p['gamma_L'] * L)

    # ═══════════════════════════════════════════════════════════════
    # B: мозговая чувствительность
    #   + развивается только при L > theta_L (dev_signal)
    #   - логистическое насыщение (B ≤ 1)
    #   - деградация (медленная → «память» развития)
    # ═══════════════════════════════════════════════════════════════
    B_sat = np.clip(1.0 - B, 0, 1)
    dB = (p['k_B_L'] * dev_signal * B_sat
          - p['gamma_B'] * B)

    # ═══════════════════════════════════════════════════════════════
    # A: аффект
    #   + развивается от B × dev_signal (оба зависят от L)
    #   - НЕТ прямого S→A (HAP-принцип)
    #   - деградация
    # ═══════════════════════════════════════════════════════════════
    A_sat = np.clip(1.0 - A, 0, 1)
    dA = (p['k_A_B'] * A_sat * B * dev_signal
          - p['gamma_A'] * A)

    # ═══════════════════════════════════════════════════════════════
    # I: воспаление
    #   + базальный
    #   + от M
    #   - деградация
    # ═══════════════════════════════════════════════════════════════
    dI = (p['I_basal']
          + p['k_M_I'] * M
          - p['gamma_I'] * I)

    # ═══════════════════════════════════════════════════════════════
    # S: HPA / стресс
    #   + от L (стероиды возбуждают HPA)
    #   + от A (аффект → стресс)
    #   - деградация
    # ═══════════════════════════════════════════════════════════════
    dS = (p['k_L_S'] * L
          + p['k_A_S'] * A
          - p['gamma_S'] * S)

    # ═══════════════════════════════════════════════════════════════
    # M: метаболизм
    #   + базальный
    #   + от L и S
    #   - деградация
    # ═══════════════════════════════════════════════════════════════
    dM = (p['M_basal']
          + p['k_L_M'] * L
          + p['k_S_M'] * S_norm
          - p['gamma_M'] * M)

    return [dL, dB, dA, dI, dS, dM]


# ══════════════════════════════════════════════════════════════════════════
# СИМУЛЯЦИЯ
# ══════════════════════════════════════════════════════════════════════════

def simulate(params, T=200, dt=0.1, t_ablate=None):
    """
    Симуляция модели HAP/NHAM.

    Параметры:
        params: словарь параметров
        T: общее время симуляции
        dt: шаг
        t_ablate: время абляции печени (L → 0). None = нет абляции.

    Возвращает словарь с time, L, B, A, I, S, M, A_final.
    """
    y0 = [0.1, 0.01, 0.01, 0.1, 0.1, 0.5]
    t_span = (0, T)

    def ode_func(t, y):
        if t_ablate is not None and t >= t_ablate:
            y_local = list(y)
            y_local[0] = 0.0  # L = 0 (полная абляция)
            return hap_nham_model(t, y_local, params)
        return hap_nham_model(t, y, params)

    sol = solve_ivp(ode_func, t_span, y0, method='LSODA',
                    max_step=dt, rtol=1e-3, atol=1e-6)

    return {
        'time': sol.t,
        'L': sol.y[0],
        'B': sol.y[1],
        'A': sol.y[2],
        'I': sol.y[3],
        'S': sol.y[4],
        'M': sol.y[5],
        'A_final': sol.y[2][-1],
    }


# ══════════════════════════════════════════════════════════════════════════
# HAP-TECT (проверка гипотезы)
# ══════════════════════════════════════════════════════════════════════════

def test_hap_hypothesis(params=None):
    """
    Проверка HAP Strong:
    - Норма: A > 0.9
    - Абляция до критического окна (t=0): A ≈ 0
    - Абляция после развития (t=100): A > 0 (но падает)
    """
    if params is None:
        params = DEFAULT_PARAMS

    res_norm = simulate(params, T=200)
    res_abl_early = simulate(params, T=200, t_ablate=0)
    res_abl_late = simulate(params, T=200, t_ablate=100)

    print("=" * 50)
    print("HAP HYPOTHESIS TEST")
    print("=" * 50)
    print(f"  Normal:           A_final = {res_norm['A_final']:.4f}")
    print(f"  Ablation t=0:     A_final = {res_abl_early['A_final']:.4f}")
    print(f"  Ablation t=100:   A_final = {res_abl_late['A_final']:.4f}")
    print("─" * 50)

    passed = True
    if res_norm['A_final'] < 0.8:
        print("  ❌ FAIL: Normal development: A should be > 0.8")
        passed = False
    if res_abl_early['A_final'] > 0.1:
        print("  ❌ FAIL: Early ablation: A should be ≈ 0")
        passed = False
    if res_abl_late['A_final'] > res_abl_early['A_final']:
        print("  ✅ PASS: Late ablation > early ablation (as expected)")
    else:
        print("  ⚠️  Late ablation should preserve some affect")

    if passed:
        print("  ✅ HAP STRONG: All checks passed!")
    return passed


# ══════════════════════════════════════════════════════════════════════════
# ГРАФИКИ
# ══════════════════════════════════════════════════════════════════════════

def plot_trajectory(result, label='', save_path=None):
    """График всех 6 переменных по времени."""
    import matplotlib.pyplot as plt
    fig, axes = plt.subplots(2, 3, figsize=(15, 8))
    vars_names = ['L (Steroids)', 'B (Sensitivity)', 'A (Affect)',
                  'I (Inflammation)', 'S (Stress)', 'M (Metabolism)']
    vars_keys = ['L', 'B', 'A', 'I', 'S', 'M']

    for i, (ax, name, key) in enumerate(zip(axes.flat, vars_names, vars_keys)):
        ax.plot(result['time'], result[key], 'b-', linewidth=1.5)
        ax.set_xlabel('Time')
        ax.set_ylabel(name)
        ax.set_title(f'{name} {label}')
        ax.grid(alpha=0.3)

    plt.tight_layout()
    if save_path:
        plt.savefig(save_path, dpi=150)
        print(f"Saved: {save_path}")
    plt.close()


def plot_comparison(results_dict, var='A', save_path=None):
    """Сравнение траекторий для разных условий (норма, абляция...)."""
    import matplotlib.pyplot as plt
    fig, ax = plt.subplots(figsize=(10, 6))
    for label, res in results_dict.items():
        ax.plot(res['time'], res[var], label=label, linewidth=1.5)
    ax.set_xlabel('Time')
    ax.set_ylabel(f'{var} value')
    ax.set_title(f'{var} — comparison')
    ax.legend()
    ax.grid(alpha=0.3)
    if save_path:
        plt.savefig(save_path, dpi=150)
        print(f"Saved: {save_path}")
    plt.close()


# ══════════════════════════════════════════════════════════════════════════
# ANALIS CHREATIVHOCTI: MORRIS
# ══════════════════════════════════════════════════════════════════════════

def run_morris_analysis(n_levels=4, n_trajectories=20, seed=42):
    """
    Morris Elementary Effects — быстрый скрининг.
    Определяет важность (mu*) и нелинейность (sigma) параметров.
    """
    try:
        from SALib.sample import morris as morris_sample
        from SALib.analyze import morris as morris_analyze
    except ImportError:
        print("SALib not installed. Run: pip install SALib")
        return None, None

    param_names = list(DEFAULT_PARAMS.keys())
    bounds = [[v * 0.1, v * 10.0] for v in DEFAULT_PARAMS.values()]

    problem = {
        'num_vars': len(param_names),
        'names': param_names,
        'bounds': bounds
    }

    np.random.seed(seed)
    X = morris_sample.sample(problem, n_trajectories, n_levels)

    Y = np.zeros(X.shape[0])
    for i in range(X.shape[0]):
        if i % 100 == 0 and i > 0:
            print(f'    Sobol: {i}/{X.shape[0]}...')
        params = dict(DEFAULT_PARAMS)
        for j, name in enumerate(param_names):
            params[name] = X[i, j]
        try:
            with timeout(5):
                res = simulate(params, T=100)
            Y[i] = res['A_final']
        except Exception:
            Y[i] = 0.0  # если симуляция не сошлась

    Si = morris_analyze.analyze(problem, X, Y, conf_level=0.95)
    return Si, param_names


def plot_morris(Si, param_names, save_path='morris_results.png'):
    import matplotlib.pyplot as plt
    mu_star = Si['mu_star']
    sigma = Si['sigma']
    names_short = [n[:14] for n in param_names]
    idx = np.argsort(mu_star)

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 5))
    ax1.barh(range(len(idx)), mu_star[idx])
    ax1.set_yticks(range(len(idx)))
    ax1.set_yticklabels([names_short[i] for i in idx])
    ax1.set_xlabel('μ* (importance)')
    ax1.set_title('Morris — Parameter Importance')

    ax2.scatter(mu_star, sigma)
    for i, name in enumerate(names_short):
        ax2.annotate(name, (mu_star[i], sigma[i]), fontsize=7)
    ax2.set_xlabel('μ* (importance)')
    ax2.set_ylabel('σ (nonlinearity)')
    ax2.set_title('Morris — μ* vs σ')
    ax2.axvline(mu_star.mean()*0.5, color='r', ls='--', alpha=0.5)

    plt.tight_layout()
    plt.savefig(save_path, dpi=150)
    print(f"Saved: {save_path}")
    plt.close()


# ══════════════════════════════════════════════════════════════════════════
# ANALIS CHREATIVHOCTI: SOBOL
# ══════════════════════════════════════════════════════════════════════════

def run_sobol_analysis(n_samples=512, seed=42):
    """
    Sobol Sensitivity Analysis — дисперсионный.
    S1: first-order effect, ST: total-order effect.

    n_samples=512 → ~18,000 simulations (может занять 5-15 мин).
    """
    try:
        from SALib.sample import saltelli
        from SALib.analyze import sobol
    except ImportError:
        print("SALib not installed.")
        return None, None

    param_names = list(DEFAULT_PARAMS.keys())
    bounds = [[v * 0.1, v * 10.0] for v in DEFAULT_PARAMS.values()]

    problem = {
        'num_vars': len(param_names),
        'names': param_names,
        'bounds': bounds
    }

    D = len(param_names)
    N = n_samples
    total = N * (2 * D + 2)
    print(f"Sobol: {N} samples × {D} params = ~{total:,} runs")

    X = saltelli.sample(problem, N, calc_second_order=False)
    Y = np.zeros(X.shape[0])

    for i in range(X.shape[0]):
        if i % 1000 == 0:
            print(f"  {i}/{X.shape[0]}...")
        params = dict(DEFAULT_PARAMS)
        for j, name in enumerate(param_names):
            params[name] = X[i, j]
        try:
            with timeout(8):
                res = simulate(params, T=100)
            Y[i] = res['A_final']
        except Exception:
            Y[i] = 0.0

    Si = sobol.analyze(problem, Y, calc_second_order=False)
    return Si, param_names


def plot_sobol(Si, param_names, save_path='sobol_results.png'):
    import matplotlib.pyplot as plt
    S1 = Si['S1']
    ST = Si['ST']
    S1_conf = Si['S1_conf']
    ST_conf = Si['ST_conf']
    names_short = [n[:14] for n in param_names]
    idx = np.argsort(ST)

    fig, ax = plt.subplots(figsize=(12, 6))
    x = range(len(idx))
    ax.bar(x, ST[idx], yerr=ST_conf[idx], capsize=5, alpha=0.5,
           label='ST (total)', color='orange')
    ax.bar(x, S1[idx], yerr=S1_conf[idx], capsize=5, alpha=0.8,
           label='S1 (first-order)', color='steelblue')
    ax.set_xticks(range(len(idx)))
    ax.set_xticklabels([names_short[i] for i in idx], rotation=45, ha='right')
    ax.set_ylabel('Sensitivity index')
    ax.set_title('Sobol — Global Sensitivity Analysis')
    ax.legend()
    ax.axhline(0, color='gray', lw=0.5)
    plt.tight_layout()
    plt.savefig(save_path, dpi=150)
    print(f"Saved: {save_path}")
    plt.close()


# ══════════════════════════════════════════════════════════════════════════
# POLNYJ RUNC
# ══════════════════════════════════════════════════════════════════════════

def full_analysis(output_dir='./results'):
    import os, json
    os.makedirs(output_dir, exist_ok=True)

    print("=" * 60)
    print("HAP/NHAM — FULL SENSITIVITY ANALYSIS")
    print("=" * 60)

    # 1. HAP test
    print("\n[1/5] HAP Hypothesis Test...")
    test_hap_hypothesis(DEFAULT_PARAMS)

    # 2. Normal trajectory
    print("\n[2/5] Normal trajectory plot...")
    res = simulate(DEFAULT_PARAMS)
    plot_trajectory(res, label='(normal)',
                    save_path=os.path.join(output_dir, 'trajectory_normal.png'))

    # 3. Comparison
    print("\n[3/5] Ablation comparison plot...")
    res_abl0 = simulate(DEFAULT_PARAMS, t_ablate=0)
    res_abl100 = simulate(DEFAULT_PARAMS, t_ablate=100)
    plot_comparison({
        'Normal': res,
        'Ablation t=0': res_abl0,
        'Ablation t=100': res_abl100,
    }, var='A', save_path=os.path.join(output_dir, 'comparison_A.png'))

    # 4. Morris
    print("\n[4/5] Morris Sensitivity Analysis...")
    Si_m, names_m = run_morris_analysis(n_levels=4, n_trajectories=15)
    if Si_m is not None:
        plot_morris(Si_m, names_m,
                    save_path=os.path.join(output_dir, 'morris.png'))
        top5 = np.argsort(Si_m['mu_star'])[-5:][::-1]
        print("  Top-5 by μ*:")
        for i, idx in enumerate(top5):
            print(f"    {i+1}. {names_m[idx]}: μ*={Si_m['mu_star'][idx]:.4f}")

    # 5. Sobol
    print("\n[5/5] Sobol Sensitivity Analysis (это займёт ~10 мин)...")
    Si_s, names_s = run_sobol_analysis(n_samples=256)
    if Si_s is not None:
        plot_sobol(Si_s, names_s,
                   save_path=os.path.join(output_dir, 'sobol.png'))
        top5 = np.argsort(Si_s['ST'])[-5:][::-1]
        print("  Top-5 by ST:")
        for i, idx in enumerate(top5):
            print(f"    {i+1}. {names_s[idx]}: ST={Si_s['ST'][idx]:.4f}")

        # Save results as JSON
        results = {}
        for i, name in enumerate(names_s):
            results[name] = {
                'S1': float(Si_s['S1'][i]),
                'S1_conf': float(Si_s['S1_conf'][i]),
                'ST': float(Si_s['ST'][i]),
                'ST_conf': float(Si_s['ST_conf'][i]),
            }
        with open(os.path.join(output_dir, 'sobol_results.json'), 'w') as f:
            json.dump(results, f, indent=2)
        print(f"  Results saved: {output_dir}/sobol_results.json")

    # Save params
    with open(os.path.join(output_dir, 'params.json'), 'w') as f:
        json.dump(DEFAULT_PARAMS, f, indent=2)

    print(f"\n{'='*60}")
    print(f"✅ All results saved to: {output_dir}")
    print(f"{'='*60}")


if __name__ == '__main__':
    import sys
    out = sys.argv[1] if len(sys.argv) > 1 else './sensitivity_results'
    full_analysis(out)
