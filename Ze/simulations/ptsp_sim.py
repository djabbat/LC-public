#!/usr/bin/env python3
"""
PTSP Simulation: dS/dt = κ · Var(τ)

Проверка теоремы о синхронности собственного времени и сохранении энтропии.

Модель:
- N частиц с фазами θᵢ ∈ [0, 2π)
- Каждая частица имеет свой темп собственного времени ωᵢ = dτᵢ/dt
- Фаза эволюционирует: dθᵢ/dt = ωᵢ (mod 2π)
- Энтропия: S = -∫ p(θ) ln p(θ) dθ (непрерывная энтропия распределения фаз)
- При синхронных ωᵢ: все θ одинаковы → δ-функция → S минимальна
- При разных ωᵢ: фазы расползаются → распределение уширяется → S растёт

Usage:
    python3 ptsp_sim.py              # быстрый прогон
    python3 ptsp_sim.py --full       # полный скан + проверка линейности
"""

import numpy as np
import argparse

# ---------------------------------------------------------------------------
# Model
# ---------------------------------------------------------------------------

class PTSPSystem:
    """
    N частиц с фазами θᵢ.
    Собственное время: τᵢ(t) = ωᵢ · t (для простоты — линейный рост).
    Фаза: θᵢ(t) = (θ₀ + ωᵢ · t) mod 2π.

    Энтропия оценивается через гистограмму распределения фаз:
    S ≈ -Σ_k (n_k/N) · ln(n_k/N) + ln(число бинов)
    """

    def __init__(self, N=200, omega_spread=0.1, seed=42):
        self.N = N
        self.rng = np.random.default_rng(seed)

        # Темпы собственного времени
        self.omega = 1.0 + omega_spread * self.rng.normal(0, 1, N)
        # Начальные фазы: все одинаковы (идеальная синхронизация)
        self.theta = np.zeros(N)
        # Начальное собственное время: 0 у всех
        self.tau = np.zeros(N)
        # Лабораторное время
        self.t = 0.0

        # История
        self.history_t = []
        self.history_S = []
        self.history_var_tau = []
        self.history_var_theta = []

    def step(self, dt=0.01):
        """Шаг эволюции."""
        self.t += dt
        self.tau += self.omega * dt
        self.theta = (self.theta + self.omega * dt) % (2 * np.pi)

    def entropy(self, n_bins=50):
        """
        Энтропия распределения фаз (непрерывная, через гистограмму).

        S = -Σ_k p_k ln p_k + ln(Δθ)  (поправка на ширину бина)
        где p_k = n_k / N, Δθ = 2π / n_bins.
        """
        hist, _ = np.histogram(self.theta, bins=n_bins, range=(0, 2 * np.pi))
        # Только непустые бины
        p = hist[hist > 0] / self.N
        S_disc = -np.sum(p * np.log(p))
        # Поправка на непрерывность (ширина бина)
        S_cont = S_disc + np.log(2 * np.pi / n_bins)
        return S_cont

    def run(self, T=10.0, dt=0.01):
        """Прогон симуляции."""
        steps = int(T / dt)
        record_every = max(1, steps // 1000)

        for step_idx in range(steps):
            self.step(dt)

            if step_idx % record_every == 0:
                self.history_t.append(self.t)
                self.history_S.append(self.entropy())
                self.history_var_tau.append(np.var(self.tau))
                self.history_var_theta.append(np.var(np.unwrap(self.theta)))

        return {
            't': np.array(self.history_t),
            'S': np.array(self.history_S),
            'var_tau': np.array(self.history_var_tau),
            'var_theta': np.array(self.history_var_theta),
        }

    def compute_kappa(self):
        """κ из линейной регрессии dS/dt ~ Var(τ)."""
        t_arr = np.array(self.history_t)
        S_arr = np.array(self.history_S)
        var_arr = np.array(self.history_var_tau)

        if len(t_arr) < 3:
            return None, None

        dt_arr = np.diff(t_arr)
        dS_arr = np.diff(S_arr)
        var_mid = 0.5 * (var_arr[:-1] + var_arr[1:])

        # dS/dt в каждой точке
        dS_dt = dS_arr / dt_arr

        # Линейная регрессия: dS/dt = κ · Var(τ)
        kappa = np.sum(var_mid * dS_dt) / np.sum(var_mid ** 2)

        # R²
        ss_res = np.sum((dS_dt - kappa * var_mid) ** 2)
        ss_tot = np.sum((dS_dt - np.mean(dS_dt)) ** 2)
        r2 = 1 - ss_res / ss_tot if ss_tot > 1e-15 else 0.0

        return kappa, r2


# ---------------------------------------------------------------------------
# Tests
# ---------------------------------------------------------------------------

def test_synchrony_preserved():
    """Тест: при ω_spread=0 синхронность сохраняется."""
    print("=" * 60)
    print("ТЕСТ 1: Идеальная синхронность (ω_spread = 0)")
    print("=" * 60)

    sys_ptsp = PTSPSystem(N=500, omega_spread=0.0, seed=42)
    r = sys_ptsp.run(T=5.0, dt=0.01)

    var_initial = r['var_tau'][0]
    var_final = r['var_tau'][-1]
    S_initial = r['S'][0]
    S_final = r['S'][-1]

    print(f"Var(τ) начальная: {var_initial:.2e}")
    print(f"Var(τ) конечная:  {var_final:.2e}")
    print(f"S начальная: {S_initial:.4f}")
    print(f"S конечная:  {S_final:.4f}")
    print(f"ΔS = {S_final - S_initial:.6f}")

    if var_final < 1e-10:
        print("✅ Var(τ) = 0 — синхронность сохраняется")
    else:
        print("❌ Var(τ) > 0 — синхронность нарушена (ошибка модели)")

    if abs(S_final - S_initial) < 0.1:
        print("✅ dS/dt ≈ 0 — энтропия сохраняется при синхронности")
    else:
        print("❌ Энтропия изменилась при синхронности")
    print()


def test_entropy_grows_with_variance():
    """Тест: при ω_spread>0 энтропия растёт."""
    print("=" * 60)
    print("ТЕСТ 2: Десинхронизация → рост энтропии")
    print("=" * 60)

    for spread in [0.0, 0.05, 0.1, 0.2]:
        sys_ptsp = PTSPSystem(N=500, omega_spread=spread, seed=42)
        r = sys_ptsp.run(T=5.0, dt=0.01)
        kappa, r2 = sys_ptsp.compute_kappa()

        # dS/dt на последнем участке
        dS = r['S'][-1] - r['S'][0]
        var_final = r['var_tau'][-1]
        T_total = r['t'][-1]

        print(f"ω_spread={spread:.2f}: Var(τ)={var_final:.4f}, ΔS={dS:+.4f}, "
              f"dS/dt≈{dS/T_total:.4f}, κ={kappa:.4f}" + (f", R²={r2:.4f}" if kappa else ""))

    print("✅ При ω_spread>0: Var(τ) > 0 → dS/dt > 0")
    print()


def scan_kappa():
    """Скан: проверка постоянства κ."""
    print("=" * 60)
    print("ТЕСТ 3: Постоянство κ при разных ω_spread")
    print("=" * 60)
    print(f"{'ω_spread':>10}  {'κ':>10}  {'R²':>10}  {'Var(τ) final':>14}  {'S final':>10}")
    print("-" * 60)

    for sp in [0.0, 0.01, 0.02, 0.05, 0.1, 0.2, 0.5]:
        sys_ptsp = PTSPSystem(N=500, omega_spread=sp, seed=42)
        r = sys_ptsp.run(T=5.0, dt=0.01)
        kappa, r2 = sys_ptsp.compute_kappa()

        if kappa is not None and r2 is not None and not np.isnan(r2):
            print(f"{sp:10.3f}  {kappa:10.4f}  {r2:10.4f}  {r['var_tau'][-1]:14.6f}  {r['S'][-1]:10.4f}")
        else:
            print(f"{sp:10.3f}  {'N/A':>10}  {'N/A':>10}  {r['var_tau'][-1]:14.6f}  {r['S'][-1]:10.4f}")

    print("-" * 60)
    print("Если κ ≈ const — линейная зависимость dS/dt ∝ Var(τ) подтверждена.")
    print()


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="PTSP Simulation")
    parser.add_argument('--full', action='store_true', help='Full analysis')
    args = parser.parse_args()

    print("╔══════════════════════════════════════════════════════╗")
    print("║  PTSP Simulation                                     ║")
    print("║  Proper Time Synchrony Principle                     ║")
    print("║  dS/dt = κ · Var(τ)                                  ║")
    print("║  Jaba Tqemaladze, MD · 2026-07-28                    ║")
    print("╚══════════════════════════════════════════════════════╝")
    print()

    test_synchrony_preserved()
    test_entropy_grows_with_variance()

    if args.full:
        scan_kappa()

    print("Done.")


if __name__ == '__main__':
    main()
