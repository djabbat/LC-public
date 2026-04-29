defmodule ZeWebWeb.AboutLive do
  use ZeWebWeb, :live_view

  @impl true
  def mount(_params, _session, socket) do
    {:ok, assign(socket, :page_title, "About")}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="card">
      <h2>About Ze Theory</h2>
      <p>
        <strong>Ze Theory</strong> derives time as a thermodynamic resource consumed by prediction error.
        The fundamental quantity is <em>impedance</em> <code>I(Z) = S(Z_real ‖ Z_model)</code> — the
        Kullback–Leibler divergence between actual and modelled state.
      </p>
      <p>
        Stochastic thermodynamics (Burgholzer 2015) gives the equality
        <code>I = ⟨ΔS⟩_gen</code>, leading to the central law
        <code>dτ_Ze/dt = −α · I(Z)</code>. From this and information geometry
        (Gassner et al. 2021), the Leggett–Garg / QFI bound (Abboud et al. 2026), and the
        asymmetric CHSH protocol (Woodhead et al. 2021) follow:
      </p>
      <ul>
        <li><code>E_Ze(a, b) = −a·b + δ · [(a·b)² − 1/3]</code> — quadratic CHSH deformation.</li>
        <li><code>S_Ze = 2√2 + δ · 1.7478</code> — optimal CHSH parameter under deformation.</li>
        <li><code>C(τ) = C₀ · exp(−β·I·τ)</code> — correlation decay.</li>
        <li><code>F_Q ≥ 8·C₀·(β·I·τ)²·(1 − β·I·τ)</code> — QFI lower bound (Lemma E).</li>
        <li><code>F_Q,max ∝ |dτ_Ze/dt|</code> — Theorem 1 (leading-order, optimal-τ).</li>
      </ul>
      <p>
        Citation: Tkemaladze J. (2026). <em>Ze Theory</em>. Longevity Horizon, 2(5).
        DOI: <a href="https://doi.org/10.65649/xf5vp867">10.65649/xf5vp867</a>.
      </p>
      <p>
        This simulator is part of the LongevityCommon ecosystem. Source code, derivations,
        and tests are in <code>~/Desktop/LongevityCommon/Ze/</code>.
      </p>
    </div>
    """
  end
end
