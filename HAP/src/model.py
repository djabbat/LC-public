"""
model.py — HAP/NHAM Simulation Prototype

Nonlinear dynamical system of hepato-affective feedback loops.

State variables:
    L(t) — Hepatic steroid output (bile acids / ecdysteroids / analogs)
    B(t) — Brain steroid sensitivity (nuclear receptor density in affective circuits)
    A(t) — Affective circuit integrity
    I(t) — Inflammatory state
    S(t) — HPA axis / stress activity (cortisol / octopamine)
    M(t) — Metabolic state (glucose / energy)

System: dX/dt = f(X, t, params)
"""

import numpy as np
from scipy.integrate import solve_ivp

# ──────────────────────────────────────────────
# Default parameters (will be moved to YAML)
# ──────────────────────────────────────────────
DEFAULT_PARAMS = {
    # Steroid production
    'L_basal': 1.0,          # Basal hepatic steroid output (nM)
    'k_L_prod': 0.5,         # Production rate constant
    'k_L_decay': 0.1,        # Decay rate constant
    'L_max': 10.0,           # Max steroid output (saturation)
    'L_half': 5.0,           # Half-saturation for production

    # Brain sensitivity
    'B_basal': 0.3,          # Basal receptor sensitivity
    'k_B_up': 0.2,           # Upregulation rate by L
    'k_B_down': 0.1,         # Downregulation rate
    'B_max': 1.0,            # Max sensitivity
    'B_half': 2.0,           # Half-saturation for L → B

    # Affective circuits
    'A_growth': 0.05,        # Growth rate of affective circuits
    'k_A_L': 0.3,            # Dependence on L (L permissive)
    'k_A_B': 0.4,            # Dependence on B (sensitivity permissive)
    'A_decay': 0.02,         # Decay without input
    'A_max': 1.0,            # Max integrity

    # Inflammation
    'I_basal': 0.1,          # Basal inflammation
    'k_I_stress': 0.05,      # Stress → inflammation
    'k_I_clear': 0.1,        # Clearance rate
    'I_suppress_L': 0.3,     # Inflammation suppresses L

    # HPA axis
    'S_basal': 0.2,           # Basal cortisol
    'k_S_stress_input': 0.1,  # External stress → S
    'k_S_neg_feedback': 0.05, # Negative feedback via A
    'k_S_decay': 0.15,       # Decay rate
    'S_enhance_L': 0.2,      # Stress → L (allostatic)

    # Metabolism
    'M_input': 1.0,          # Nutrient input
    'M_consumption': 0.1,    # Consumption rate
    'k_M_L': 0.1,            # Metabolic effect on L
    'M_min': 0.3,            # Minimum metabolic level

    # Critical developmental window
    'tau_crit': 72.0,        # Critical window end (hours post-fertilization)
    'irreversible': True,    # If L=0 before tau_crit, A is permanently lost
}

# ──────────────────────────────────────────────
# ODE System
# ──────────────────────────────────────────────
def hepato_affective_system(t, y, params, stress_input=0.0):
    """
    ODE right-hand side for HAP/NHAM model.

    Parameters:
        t : float — current time
        y : ndarray — [L, B, A, I, S, M]
        params : dict — system parameters
        stress_input : float — external stressor (0.0 = no stress)

    Returns:
        dydt : ndarray — time derivatives
    """
    L, B, A, I, S, M = y
    p = params

    # ── Hepatic steroid output ──
    # Production: basal + S enhancement (allostatic) - I suppression
    prod_L = p['L_basal'] + p['S_enhance_L'] * S - p['I_suppress_L'] * I
    prod_L = max(0.0, prod_L)
    # Logistic saturation
    sat_L = 1.0 - L / p['L_max']
    dL = p['k_L_prod'] * prod_L * sat_L - p['k_L_decay'] * L

    # ── Brain steroid sensitivity ──
    # Upregulation by L, downregulation by decay
    up_B = p['k_B_up'] * (L / (L + p['B_half'])) * (1.0 - B / p['B_max'])
    down_B = p['k_B_down'] * B
    dB = up_B - down_B

    # ── Affective circuit integrity ──
    # Growth requires L and B (permissive), decays otherwise
    permissive = (L / (L + 0.1)) * (B / (B + 0.1))
    growth_A = p['k_A_L'] * permissive * (1.0 - A / p['A_max'])
    decay_A = p['A_decay'] * A

    # Critical developmental window
    if t < p['tau_crit'] and p['irreversible'] and L < 0.01:
        # If no steroid during critical window, circuits cannot form
        growth_A = 0.0
        decay_A = p['A_decay'] * A  # Accelerated decay

    dA = growth_A - decay_A

    # ── Inflammation ──
    # Basal + stress-induced - clearance
    dI = p['k_I_stress'] * S + p['I_basal'] - p['k_I_clear'] * I

    # ── HPA axis ──
    # Stress input + negative feedback via A
    dS = (p['S_basal'] + p['k_S_stress_input'] * stress_input -
          p['k_S_neg_feedback'] * A - p['k_S_decay'] * S)

    # ── Metabolism ──
    # Nutrient input - consumption, modulated by L
    dM = (p['M_input'] - p['M_consumption'] * M -
          p['k_M_L'] * (1.0 - L / (L + 1.0)))
    # Clamp to minimum
    if M < p['M_min']:
        dM = max(0.0, dM)

    return np.array([dL, dB, dA, dI, dS, dM])

# ──────────────────────────────────────────────
# Simulation runner
# ──────────────────────────────────────────────
def simulate(params=None, y0=None, t_span=(0, 200), t_eval=None,
             stress_input=0.0, method='RK45'):
    """
    Run HAP/NHAM simulation.

    Parameters:
        params : dict — system parameters (default: DEFAULT_PARAMS)
        y0 : ndarray — initial state [L0, B0, A0, I0, S0, M0]
        t_span : tuple — (t_start, t_end)
        t_eval : ndarray — time points for output
        stress_input : float — external stressor
        method : str — solver method (RK45, LSODA, Radau, etc.)

    Returns:
        sol : OdeSolution object (sol.t, sol.y)
    """
    if params is None:
        params = DEFAULT_PARAMS.copy()
    if y0 is None:
        y0 = np.array([0.1, 0.1, 0.0, 0.1, 0.2, 1.0])
    if t_eval is None:
        t_eval = np.linspace(t_span[0], t_span[1], 1000)

    sol = solve_ivp(
        hepato_affective_system,
        t_span, y0,
        args=(params, stress_input),
        t_eval=t_eval,
        method=method,
        max_step=1.0,
        rtol=1e-6, atol=1e-9
    )
    return sol

# ──────────────────────────────────────────────
# In silico experiments
# ──────────────────────────────────────────────
# ──────────────────────────────────────────────
# Bifurcation analysis
# ──────────────────────────────────────────────

def bifurcation_analysis(param_name='L_basal', param_range=None, n_points=50,
                          t_end=500.0, keep_all=False, isolate=False):
    """
    Bifurcation analysis: vary one parameter and measure final A.

    Parameters:
        param_name : str — parameter to vary (must be in DEFAULT_PARAMS)
        param_range : tuple/list — (min, max) range, auto if None
        n_points : int — number of parameter values to test
        t_end : float — simulation time (longer for steady state)
        keep_all : bool — if True, return all trajectories
        isolate : bool — if True, disable other L sources (S_enhance_L, I_suppress_L)
                        to isolate pure effect of param on A

    Returns:
        param_values : ndarray — parameter values tested
        A_final : ndarray — final A values
        (optionally) trajectories : list of sol objects
    """
    if param_range is None:
        # Auto-range based on parameter
        default_val = DEFAULT_PARAMS[param_name]
        if param_name in ('L_basal', 'L_half', 'k_A_L', 'A_decay', 'k_B_up'):
            param_range = (0.0, default_val * 4)
        elif param_name in ('tau_crit',):
            param_range = (0.0, default_val * 2)
        elif param_name in ('k_A_B', 'k_B_down', 'I_suppress_L', 'S_enhance_L'):
            param_range = (0.0, 1.0)
        else:
            param_range = (0.0, default_val * 3)

    param_values = np.linspace(param_range[0], param_range[1], n_points)
    A_final = np.zeros(n_points)
    trajectories = []

    for i, val in enumerate(param_values):
        params = DEFAULT_PARAMS.copy()
        params[param_name] = val
        if isolate:
            # Disable all other L sources to isolate pure param effect
            params['S_enhance_L'] = 0.0
            params['I_suppress_L'] = 0.0
        sol = simulate(params=params, t_span=(0, t_end), 
                       t_eval=np.linspace(0, t_end, 2000))
        A_final[i] = sol.y[2, -1]
        if keep_all:
            trajectories.append(sol)

    if keep_all:
        return param_values, A_final, trajectories
    return param_values, A_final


def bifurcation_2d(param_x='L_basal', param_y='k_A_L',
                    x_range=None, y_range=None,
                    n_x=30, n_y=30, t_end=500.0):
    """
    2D bifurcation: vary two parameters, measure final A.
    Returns a 2D heatmap for A_final(x_param, y_param).

    Useful for finding the region where A > 0 (affect possible).
    """
    if x_range is None:
        x_range = (0.0, DEFAULT_PARAMS[param_x] * 3)
    if y_range is None:
        y_range = (0.0, DEFAULT_PARAMS[param_y] * 3)

    x_vals = np.linspace(x_range[0], x_range[1], n_x)
    y_vals = np.linspace(y_range[0], y_range[1], n_y)
    Z = np.zeros((n_y, n_x))

    for ix, xv in enumerate(x_vals):
        for iy, yv in enumerate(y_vals):
            params = DEFAULT_PARAMS.copy()
            params[param_x] = xv
            params[param_y] = yv
            sol = simulate(params=params, t_span=(0, t_end))
            Z[iy, ix] = sol.y[2, -1]

    return x_vals, y_vals, Z


def run_ablation_experiment(t_ablate=0.0, t_crit=72.0, t_end=200.0):
    """
    Experiment: ablate hepatic output at t_ablate.
    If t_ablate < tau_crit → A should be permanently lost.
    If t_ablate > tau_crit → A should be partially preserved.
    """
    # We implement this by setting L=0 after t_ablate
    # For simplicity, we'll use a modified system
    # (proper implementation with events-based switching is TODO)

    def ablated_system(t, y):
        params = DEFAULT_PARAMS.copy()
        stress = 0.0
        if t >= t_ablate:
            # Set L to 0 by making production impossible
            params['L_basal'] = 0.0
            params['S_enhance_L'] = 0.0
        return hepato_affective_system(t, y, params, stress)

    y0 = np.array([0.1, 0.1, 0.0, 0.1, 0.2, 1.0])
    t_span = (0, t_end)
    t_eval = np.linspace(0, t_end, 2000)

    sol = solve_ivp(
        ablated_system, t_span, y0,
        t_eval=t_eval, method='RK45',
        max_step=0.5, rtol=1e-6, atol=1e-9
    )
    return sol

# ──────────────────────────────────────────────
# If run directly
# ──────────────────────────────────────────────
if __name__ == '__main__':
    print("Running HAP/NHAM simulation...")

    # Normal development
    sol = simulate(t_span=(0, 200), t_eval=np.linspace(0, 200, 2000))
    print(f"  Final A(t): {sol.y[2,-1]:.3f} (normal)")
    print(f"  Final L(t): {sol.y[0,-1]:.3f} (normal)")

    # Ablation at t=0 (before critical window)
    sol_abl0 = run_ablation_experiment(t_ablate=0.0, t_end=200)
    print(f"  Final A(t) [ablation at t=0]: {sol_abl0.y[2,-1]:.3f} (expected ~0)")

    # Ablation after critical window
    sol_abl_post = run_ablation_experiment(t_ablate=100.0, t_end=200)
    print(f"  Final A(t) [ablation at t=100]: {sol_abl_post.y[2,-1]:.3f} (expected >0)")
