"""
main.py — HAP/NHAM Simulation CLI

Usage:
    python3 main.py run                # Run normal simulation
    python3 main.py experiment         # Run all ablation experiments
    python3 main.py plot               # Plot all experiments
    python3 main.py all                # Run + plot everything
"""

import sys
import numpy as np
from model import simulate, run_ablation_experiment
from visualize import plot_trajectory, plot_all_experiments

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
        'all': cmd_all,
    }

    if cmd in commands:
        commands[cmd]()
    else:
        print(f"Unknown command: {cmd}")
        print(__doc__)
