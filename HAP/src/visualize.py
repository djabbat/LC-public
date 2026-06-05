"""
visualize.py — Plotting for HAP/NHAM simulation results.
"""

import numpy as np
import matplotlib.pyplot as plt
from model import (simulate, run_ablation_experiment, DEFAULT_PARAMS,
                      bifurcation_analysis, bifurcation_2d)

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

# ──────────────────────────────────────────────
# Bifurcation plots
# ──────────────────────────────────────────────

def plot_bifurcation(param_name='L_basal', param_range=None, n_points=50,
                      t_end=500.0, ax=None, isolate=False):
    """
    Plot 1D bifurcation: A_final vs parameter.
    Shows the threshold (bifurcation point) where A jumps to 0.

    If isolate=True, disables other L sources (allostatic, inflammatory)
    to show pure effect of the parameter on A.
    """
    if ax is None:
        fig, ax = plt.subplots(figsize=(9, 6))
    else:
        fig = ax.figure

    p_vals, A_final = bifurcation_analysis(param_name, param_range, n_points, t_end,
                                            isolate=isolate)

    ax.plot(p_vals, A_final, 'o-', color='tab:green', markersize=3, linewidth=1.5)
    ax.axhline(y=0.05, color='gray', linestyle=':', alpha=0.5,
               label='A ≈ 0 threshold')

    # Find bifurcation point (where A crosses threshold)
    threshold = 0.05
    cross_idx = np.where(A_final < threshold)[0]
    if len(cross_idx) > 0:
        bif_idx = cross_idx[0]
        bif_val = p_vals[bif_idx]
        ax.axvline(x=bif_val, color='red', linestyle='--', linewidth=2,
                   label=f'Bifurcation ≈ {bif_val:.3f}')
        ax.annotate(f'No affect\nA < {threshold}',
                     xy=(p_vals[-1]*0.7, 0.01), fontsize=10, color='red')
        ax.annotate(f'Affect possible\nA > {threshold}',
                     xy=(p_vals[-1]*0.1, A_final[-1]*0.6), fontsize=10, color='green')

    param_labels = {
        'L_basal': 'Basal hepatic steroid output L_basal',
        'tau_crit': 'Critical window duration τ_crit (hpf)',
        'k_A_L': 'Steroid → affect coupling k_A_L',
        'A_decay': 'Affective circuit decay rate A_decay',
        'k_B_up': 'Brain sensitivity upregulation k_B_up',
        'I_suppress_L': 'Inflammation suppression of L I_suppress_L',
        'S_enhance_L': 'Allostatic enhancement S_enhance_L',
    }
    xlabel = param_labels.get(param_name, f'Parameter {param_name}')

    ax.set_xlabel(xlabel, fontsize=12)
    ax.set_ylabel('Final affective circuit integrity A', fontsize=12)
    ax.set_title(f'Bifurcation Diagram: A vs {param_name}', fontsize=14)
    ax.legend(fontsize=9)
    ax.grid(alpha=0.3)
    return fig, ax


def plot_bifurcation_2d(param_x='L_basal', param_y='k_A_L',
                         x_range=None, y_range=None,
                         n_x=40, n_y=40, ax=None):
    """
    Plot 2D bifurcation heatmap: A_final(x_param, y_param).
    Shows the viable region (A > 0) in parameter space.
    """
    if ax is None:
        fig, ax = plt.subplots(figsize=(9, 7))
    else:
        fig = ax.figure

    x_vals, y_vals, Z = bifurcation_2d(param_x, param_y,
                                         x_range, y_range,
                                         n_x, n_y)

    im = ax.contourf(x_vals, y_vals, Z, levels=20, cmap='viridis')
    cbar = plt.colorbar(im, ax=ax)
    cbar.set_label('Final A (affective integrity)', fontsize=11)

    # Add contour for A = 0.05 threshold
    CS = ax.contour(x_vals, y_vals, Z, levels=[0.05], colors='red', linewidths=2)
    ax.clabel(CS, fmt='A=0.05', fontsize=9)

    param_labels = {
        'L_basal': 'L_basal',
        'k_A_L': 'k_A_L',
        'tau_crit': 'τ_crit',
        'A_decay': 'A_decay',
    }
    ax.set_xlabel(param_labels.get(param_x, param_x), fontsize=12)
    ax.set_ylabel(param_labels.get(param_y, param_y), fontsize=12)
    ax.set_title(f'2D Bifurcation: A({param_x}, {param_y})', fontsize=14)
    return fig, ax


def plot_bifurcation_all(ax=None):
    """Plot all 1D bifurcations for key parameters in a 2x3 grid."""
    key_params = ['L_basal', 'tau_crit', 'k_A_L', 'A_decay', 'k_B_up', 'I_suppress_L']

    if ax is None:
        fig, axes = plt.subplots(2, 3, figsize=(15, 10))
    else:
        fig = ax.figure
        axes = ax  # assume 2x3 array

    for i, param in enumerate(key_params):
        row, col = divmod(i, 3)
        iso = (param == 'L_basal')  # isolate L_basal from other L sources
        plot_bifurcation(param, ax=axes[row, col], isolate=iso)

    plt.tight_layout()
    return fig, axes


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

    # Figure 5: 1D bifurcation L_basal (isolated)
    fig5, _ = plot_bifurcation('L_basal', n_points=60, t_end=500, isolate=True)
    fig5.savefig('../results/bifurcation_L_basal.png', dpi=150, bbox_inches='tight')
    print("  Saved: results/bifurcation_L_basal.png")

    # Figure 6: 1D bifurcation L_basal (full system with all loops)
    fig6, _ = plot_bifurcation('L_basal', n_points=60, t_end=500, isolate=False,
                                param_range=(0, 5))
    fig6.savefig('../results/bifurcation_L_basal_full.png', dpi=150, bbox_inches='tight')
    print("  Saved: results/bifurcation_L_basal_full.png")

    # Figure 7: 1D bifurcation tau_crit
    fig7, _ = plot_bifurcation('tau_crit', n_points=60, t_end=500)
    fig7.savefig('../results/bifurcation_tau_crit.png', dpi=150, bbox_inches='tight')
    print("  Saved: results/bifurcation_tau_crit.png")

    # Figure 8: 1D bifurcation k_A_L
    fig8, _ = plot_bifurcation('k_A_L', n_points=60, t_end=500)
    fig8.savefig('../results/bifurcation_k_A_L.png', dpi=150, bbox_inches='tight')
    print("  Saved: results/bifurcation_k_A_L.png")

    # Figure 9: 1D bifurcation — all key params in grid
    fig9, _ = plot_bifurcation_all()
    fig9.savefig('../results/bifurcation_all_params.png', dpi=150, bbox_inches='tight')
    print("  Saved: results/bifurcation_all_params.png")

    plt.close('all')
    print("\n✅ All plots saved to results/")

if __name__ == '__main__':
    plot_all_experiments()
