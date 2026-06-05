"""
main.py — HAP/NHAM Simulation CLI

Usage:
    python3 main.py run                # Run normal simulation
    python3 main.py experiment         # Run all ablation experiments
    python3 main.py plot               # Plot all experiments
    python3 main.py all                # Run + plot everything
    python3 main.py bifurcation        # Run + plot bifurcation analysis
    python3 main.py bifurcation-2d     # Run 2D bifurcation heatmap
"""

import sys
import numpy as np
import matplotlib.pyplot as plt
from model import simulate, run_ablation_experiment, bifurcation_analysis
from visualize import (plot_trajectory, plot_all_experiments,
                        plot_bifurcation, plot_bifurcation_2d)

def print_results(sol, label=''):
    """Print final state values."""
    names = ['L (hepatic)', 'B (brain sens)', 'A (affective)',
             'I (inflamm)', 'S (stress)', 'M (metabolic)']
    print(f"\n{'='*50}")
    print(f"Results: {label}")
    print(f"{'='*50}")
    for i, name in enumerate(names):
        print(f"  {name}: {sol.y[i,0]:.3f} → {sol.y[i,-1]:.3f}")
    print(f"{'='*50}\n")

def cmd_run():
    """Run normal simulation."""
    print("Running normal development simulation...")
    sol = simulate()
    print_results(sol, 'Normal development')

def cmd_experiment():
    """Run ablation experiments."""
    print("\n--- Ablation at t=0 (before critical window) ---")
    sol_abl0 = run_ablation_experiment(t_ablate=0.0)
    print_results(sol_abl0, 'Ablation at t=0')

    print("\n--- Ablation at t=100 (after critical window) ---")
    sol_abl100 = run_ablation_experiment(t_ablate=100.0)
    print_results(sol_abl100, 'Ablation at t=100')

    print("\n--- Comparison ---")
    print(f"  Normal A(t) final:   {simulate().y[2,-1]:.3f}")
    print(f"  Ablation t=0 final:  {sol_abl0.y[2,-1]:.3f}")
    print(f"  Ablation t=100 final:{sol_abl100.y[2,-1]:.3f}")
    print(f"\n  HAP Prediction ablated < crit: A ≈ 0 → {'✅ PASS' if sol_abl0.y[2,-1] < 0.05 else '❌ FAIL'}")
    print(f"  HAP Prediction ablated > crit: A > 0 → {'✅ PASS' if sol_abl100.y[2,-1] > 0.1 else '❌ FAIL'}")

def cmd_bifurcation():
    """Run 1D bifurcation analysis for key parameters."""
    print("\n=== Bifurcation Analysis ===\n")

    for param in ['L_basal', 'tau_crit', 'k_A_L', 'A_decay']:
        print(f"  Scanning {param}...")
        p_vals, A_final = bifurcation_analysis(param, n_points=30, t_end=500)
        # Find bifurcation
        threshold = 0.05
        cross = np.where(A_final < threshold)[0]
        if len(cross) > 0:
            bif = p_vals[cross[0]]
            print(f"    Bifurcation point ≈ {bif:.4f} (A drops below {threshold})")
        else:
            print(f"    No bifurcation in range (A always > {threshold})")

    print("\n  Generating plots...")
    from visualize import plot_bifurcation_all
    fig, _ = plot_bifurcation_all()
    fig.savefig('../results/bifurcation_all_params.png', dpi=150, bbox_inches='tight')
    plt.close(fig)
    print("  Saved: results/bifurcation_all_params.png")

    # Individual plots
    for param in ['L_basal', 'tau_crit', 'k_A_L']:
        fig, _ = plot_bifurcation(param, n_points=60, t_end=500)
        fig.savefig(f'../results/bifurcation_{param}.png', dpi=150, bbox_inches='tight')
        plt.close(fig)
        print(f"  Saved: results/bifurcation_{param}.png")

    print("\n✅ Bifurcation analysis complete")


def cmd_bifurcation_2d():
    """Run 2D bifurcation heatmap."""
    print("\n=== 2D Bifurcation Heatmap ===\n")
    print("  Scanning L_basal × k_A_L (40×40 grid)...")
    fig, _ = plot_bifurcation_2d('L_basal', 'k_A_L', n_x=40, n_y=40)
    fig.savefig('../results/bifurcation_2d_L_vs_kAL.png', dpi=150, bbox_inches='tight')
    plt.close(fig)
    print("  Saved: results/bifurcation_2d_L_vs_kAL.png")
    print("\n✅ 2D bifurcation complete")


def cmd_plot():
    """Generate all plots."""
    plot_all_experiments()

def cmd_all():
    """Run everything."""
    cmd_run()
    cmd_experiment()
    cmd_plot()

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(0)

    cmd = sys.argv[1]
    commands = {
        'run': cmd_run,
        'experiment': cmd_experiment,
        'plot': cmd_plot,
        'bifurcation': cmd_bifurcation,
        'bifurcation-2d': cmd_bifurcation_2d,
        'all': cmd_all,
    }

    if cmd in commands:
        commands[cmd]()
    else:
        print(f"Unknown command: {cmd}")
        print(__doc__)
