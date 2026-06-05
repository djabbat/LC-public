"""
visualize.py — Plotting for HAP/NHAM simulation results.
"""

import numpy as np
import matplotlib.pyplot as plt
from model import simulate, run_ablation_experiment, DEFAULT_PARAMS

VARIABLE_NAMES = {
    'L': ('Hepatic steroid output L(t)', 'nM', 'tab:blue'),
    'B': ('Brain steroid sensitivity B(t)', 'a.u.', 'tab:orange'),
    'A': ('Affective circuit integrity A(t)', 'a.u.', 'tab:green'),
    'I': ('Inflammatory state I(t)', 'a.u.', 'tab:red'),
    'S': ('HPA / stress activity S(t)', 'nM', 'tab:purple'),
    'M': ('Metabolic state M(t)', 'mM', 'tab:brown'),
}

def plot_trajectory(sol, title='HAP/NHAM Simulation', ax=None, labels=True,
                    vars_to_plot=None):
    """Plot all 6 state variables over time."""
    if ax is None:
        fig, ax = plt.subplots(figsize=(10, 6))
    else:
        fig = ax.figure

    if vars_to_plot is None:
        vars_to_plot = list(range(6))

    keys = list(VARIABLE_NAMES.keys())
    for i in vars_to_plot:
        name, unit, color = VARIABLE_NAMES[keys[i]]
        if labels:
            label = f'{keys[i]} (${name.split("(")[1].split(")")[0]}$)'
        else:
            label = None
        ax.plot(sol.t, sol.y[i], color=color, label=label, linewidth=2)

    ax.set_xlabel('Time (hours post-fertilization)', fontsize=12)
    ax.set_ylabel('State', fontsize=12)
    ax.set_title(title, fontsize=14)
    ax.legend(fontsize=9, loc='best')
    ax.grid(alpha=0.3)
    return fig, ax

def plot_experiment_comparison(sol_normal, sol_ablate, ablate_time,
                               title='Ablation Experiment'):
    """Compare normal vs ablated trajectories for A(t)."""
    fig, ax = plt.subplots(figsize=(10, 6))

    ax.plot(sol_normal.t, sol_normal.y[2],
            color='tab:green', linewidth=2, label='Normal A(t)')
    ax.plot(sol_ablate.t, sol_ablate.y[2],
            color='tab:red', linewidth=2, linestyle='--',
            label=f'Ablated A(t) (t_ablate={ablate_time})')

    ax.axvline(x=ablate_time, color='gray', linestyle=':',
               label=f'Ablation at t={ablate_time}')
    ax.axvline(x=DEFAULT_PARAMS['tau_crit'], color='gold',
               linestyle=':', linewidth=2,
               label=f'Critical window end τ={DEFAULT_PARAMS["tau_crit"]}')

    ax.set_xlabel('Time (hours post-fertilization)', fontsize=12)
    ax.set_ylabel('Affective circuit integrity A(t)', fontsize=12)
    ax.set_title(title, fontsize=14)
    ax.legend(fontsize=10)
    ax.grid(alpha=0.3)
    return fig, ax

def plot_all_experiments():
    """Run and plot all ablation experiments."""
    print("  Running normal simulation...")
    sol_norm = simulate()

    print("  Running ablation at t=0...")
    sol_abl0 = run_ablation_experiment(t_ablate=0.0)

    print("  Running ablation after critical window (t=100)...")
    sol_abl100 = run_ablation_experiment(t_ablate=100.0)

    # Figure 1: Full trajectory (normal)
    fig1, _ = plot_trajectory(sol_norm, title='HAP/NHAM — Normal Development')
    fig1.savefig('../results/normal_trajectory.png', dpi=150, bbox_inches='tight')
    print("  Saved: results/normal_trajectory.png")

    # Figure 2: Ablation at t=0 comparison
    fig2, _ = plot_experiment_comparison(
        sol_norm, sol_abl0, 0.0,
        title='Ablation at t=0 (before critical window)'
    )
    fig2.savefig('../results/ablation_before_crit.png', dpi=150, bbox_inches='tight')
    print("  Saved: results/ablation_before_crit.png")

    # Figure 3: Ablation at t=100 comparison
    fig3, _ = plot_experiment_comparison(
        sol_norm, sol_abl100, 100.0,
        title='Ablation at t=100 (after critical window)'
    )
    fig3.savefig('../results/ablation_after_crit.png', dpi=150, bbox_inches='tight')
    print("  Saved: results/ablation_after_crit.png")

    # Figure 4: Phase portrait (A vs L)
    fig4, ax4 = plt.subplots(figsize=(8, 6))
    ax4.plot(sol_norm.y[0], sol_norm.y[2], color='tab:green', linewidth=2,
             label='Normal')
    ax4.plot(sol_abl0.y[0], sol_abl0.y[2], color='tab:red', linewidth=2,
             linestyle='--', label='Ablation at t=0')
    ax4.plot(sol_abl100.y[0], sol_abl100.y[2], color='tab:orange', linewidth=2,
             linestyle='--', label='Ablation at t=100')
    ax4.set_xlabel('Hepatic steroid output L(t)', fontsize=12)
    ax4.set_ylabel('Affective circuit integrity A(t)', fontsize=12)
    ax4.set_title('Phase Portrait: A vs L', fontsize=14)
    ax4.legend(fontsize=10)
    ax4.grid(alpha=0.3)
    fig4.savefig('../results/phase_portrait_A_vs_L.png', dpi=150, bbox_inches='tight')
    print("  Saved: results/phase_portrait_A_vs_L.png")

    plt.close('all')
    print("\n✅ All plots saved to results/")

if __name__ == '__main__':
    plot_all_experiments()
