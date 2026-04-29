defmodule BiosenseWebWeb.DatasetsLive do
  use BiosenseWebWeb, :live_view

  @impl true
  def mount(_params, _session, socket) do
    {:ok, socket
     |> assign(:page_title, "Datasets")
     |> assign(:datasets, datasets())}
  end

  defp datasets do
    [
      %{id: "lemon",
        name: "LEMON (MPI Leipzig Mind-Brain-Body)",
        modalities: "EEG (resting), ECG, MRI",
        size: "N=227, age 20–77",
        license: "CC-BY-NC (open, research)",
        relevance: "Direct EEG → χ_Ze(EEG) testbed; baseline aging-correlated cohort",
        status: "Phase 2 — loader pending",
        link: "https://www.openneuro.org/datasets/ds000221"},
      %{id: "cuban",
        name: "Cuban EEG dataset (Olmos-Cabrera et al.)",
        modalities: "EEG (resting eyes-closed/open)",
        size: "N=196",
        license: "Open with DUA",
        relevance: "Original article cohort for χ_Ze pilot (r=−0.61 with age)",
        status: "Phase 2 — loader pending; archived script in _archive/",
        link: "https://www.synapse.org (Olmos-Cabrera 2020)"},
      %{id: "nhats",
        name: "NHATS (National Health and Aging Trends Study)",
        modalities: "Wearable accelerometry (rest-activity rhythm)",
        size: "N≈62k participants (Shim & Onnela 2025 subset)",
        license: "Public with sign-in DUA",
        relevance: "Largest aging-wearable cohort; rest-activity rhythm features",
        status: "Phase 2 — DUA required; loader pending",
        link: "https://www.nhatsdata.org"},
      %{id: "allofus_fitbit",
        name: "All of Us — Fitbit subset",
        modalities: "Fitbit (heart rate, steps, sleep)",
        size: "N=2,222 (article subset) of larger total",
        license: "Researcher Workbench DUA, Registered tier",
        relevance: "External-validation cohort cited in article (PhenoAge r=0.67)",
        status: "Phase 2 — DUA required; loader pending",
        link: "https://www.researchallofus.org"},
      %{id: "ukbb",
        name: "UK Biobank — wearable subset",
        modalities: "AX3 accelerometer (1 week, 100 Hz)",
        size: "N≈103k with valid wearable data",
        license: "Full DUA + cost",
        relevance: "Gold-standard population-scale wearable resource",
        status: "Phase 2+ — placeholder only (cost barrier)",
        link: "https://www.ukbiobank.ac.uk"},
      %{id: "physionet_eeg",
        name: "PhysioNet — resting-state EEG (multiple cohorts)",
        modalities: "EEG resting",
        size: "varies",
        license: "Open with citation requirement",
        relevance: "Multi-cohort EEG benchmarks for χ_Ze stability",
        status: "Phase 2 — survey + loader pending",
        link: "https://physionet.org"},
      %{id: "shhs",
        name: "SHHS — Sleep Heart Health Study",
        modalities: "Polysomnography (EEG+ECG+respiration)",
        size: "N≈6,400 baseline + follow-ups",
        license: "DUA via NSRR",
        relevance: "Multi-modal night-time data — direct fit for sleep-modality χ_Ze",
        status: "Phase 2 — loader pending",
        link: "https://sleepdata.org/datasets/shhs"},
      %{id: "mesa",
        name: "MESA Sleep (Multi-Ethnic Study of Atherosclerosis)",
        modalities: "Polysomnography + actigraphy + HRV",
        size: "N≈2,200 sleep exam subset",
        license: "DUA via NSRR",
        relevance: "Cross-modality test of χ_Ze(sleep) + χ_Ze(HRV)",
        status: "Phase 2 — loader pending",
        link: "https://sleepdata.org/datasets/mesa"},
      %{id: "dreamer",
        name: "DREAMER (multimodal affective DB)",
        modalities: "EEG (Emotiv) + ECG (Empatica)",
        size: "N=23",
        license: "Open with citation",
        relevance: "Small-N feasibility for joint EEG+HRV χ_Ze pipeline",
        status: "Phase 2 — loader pending",
        link: "https://zenodo.org/record/546113"},
      %{id: "stress_predict",
        name: "Stress-Predict (Empatica E4)",
        modalities: "HRV + EDA + temperature",
        size: "N=35 healthy adults",
        license: "Open",
        relevance: "Short-N feasibility for HRV-only χ_Ze",
        status: "Phase 2 — loader pending",
        link: "https://physionet.org/content/stresspredict/1.0/"},
      %{id: "pmdata",
        name: "PMData",
        modalities: "Fitbit + lifelog (sleep, activity, mood)",
        size: "N=16, 5 months",
        license: "CC-BY-4.0",
        relevance: "Long-N=1 dense per-participant for trajectory analyses",
        status: "Phase 2 — loader pending",
        link: "https://datasets.simula.no/pmdata"}
    ]
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="card">
      <h2>Dataset registry</h2>
      <p style="font-size:13px; color:#52525b; margin:0 0 12px 0;">
        Inventory of datasets considered for χ_Ze validation. Status reflects implementation in this subproject's
        <code>datasets/</code> crate (Phase 2 of TODO). Article-cited datasets (Cuban, All-of-Us Fitbit, NHATS, LEMON-style)
        have priority; new candidates (PMData, DREAMER, MESA, SHHS) extend coverage.
      </p>
      <table>
        <thead>
          <tr><th>ID</th><th>Name</th><th>Modalities</th><th>Size</th><th>Licence</th><th>Status</th></tr>
        </thead>
        <tbody>
          <%= for ds <- @datasets do %>
            <tr>
              <td><a href={ds.link} rel="noopener">{ds.id}</a></td>
              <td>{ds.name}</td>
              <td>{ds.modalities}</td>
              <td>{ds.size}</td>
              <td>{ds.license}</td>
              <td>{ds.status}</td>
            </tr>
          <% end %>
        </tbody>
      </table>
    </div>

    <div class="card">
      <h2>Why each dataset</h2>
      <ul>
        <%= for ds <- @datasets do %>
          <li style="margin-bottom: 6px;">
            <strong>{ds.id}</strong> — {ds.relevance}.
          </li>
        <% end %>
      </ul>
    </div>
    """
  end
end
