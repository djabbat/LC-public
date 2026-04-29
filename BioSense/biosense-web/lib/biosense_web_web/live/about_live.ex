defmodule BiosenseWebWeb.AboutLive do
  use BiosenseWebWeb, :live_view

  @impl true
  def mount(_params, _session, socket) do
    {:ok, assign(socket, :page_title, "About")}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="card">
      <h2>About BioSense</h2>
      <p>
        <strong>BioSense</strong> is a wearable platform for continuous monitoring of aging as a Total Chronic Disease.
        It estimates the <em>χ_Ze index</em> — derived from EEG, HRV, respiration and sleep — and provides preliminary
        30-day exacerbation-risk estimates, with all processing on-device behind a five-layer privacy stack.
      </p>
      <ul>
        <li>Theoretical fixed point <code>v* = 0.45631</code> from a variational principle <code>F = E − T·S − λ·I_pred</code>.</li>
        <li>Bridge to centriolar damage <code>D(t)</code> via <code>A(D) = a + b·D + c·D²</code> and <code>χ_Ze = g₀ − g₁·A</code>.</li>
        <li>Exacerbation classifier on rolling χ_Ze: P_30d via logistic regression on (age, sex, χ_Ze, Δχ_Ze 7d).</li>
        <li>Privacy stack: minimisation, k-anonymity (k≥7), DP (ε=2.0, Laplace, sensitivity 0.3), secure aggregation.</li>
      </ul>
      <p>
        Citation: Tkemaladze J. (2026). <em>BioSense</em>. Longevity Horizon, 2(5).
        DOI: <a href="https://doi.org/10.65649/23ba5z09">10.65649/23ba5z09</a>.
      </p>
      <p>
        ⚠ Research-grade. No clinical claims. Underpowered pilots; AUCs are exploratory.
      </p>
    </div>
    """
  end
end
