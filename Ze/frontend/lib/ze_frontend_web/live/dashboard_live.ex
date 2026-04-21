defmodule ZeFrontendWeb.DashboardLive do
  use ZeFrontendWeb, :live_view

  alias ZeFrontendWeb.BackendClient
  alias ZeFrontendWeb.CoreComponents

  @impl true
  def mount(_params, _session, socket) do
    if connected?(socket), do: Process.send_after(self(), :refresh_data, 100)

    {:ok,
     socket
     |> assign(:loading, true)
     |> assign(:dashboard_data, nil)
     |> assign(:mcoa_counters, [])
     |> assign(:cdata_sensitivity, nil)
     |> assign(:error, nil)}
  end

  @impl true
  def handle_params(_params, _uri, socket) do
    {:noreply, assign(socket, page_title: "Dashboard - Ze Theory")}
  end

  @impl true
  def handle_event("refresh", _params, socket) do
    {:noreply,
     socket
     |> assign(:loading, true)
     |> push_event("refresh_start", %{})}
  end

  @impl true
  def handle_info(:refresh_data, socket) do
    with {:ok, dashboard} <- BackendClient.get_dashboard(),
         {:ok, counters} <- BackendClient.get_mcoa_counters(),
         {:ok, sensitivity} <- BackendClient.get_cdata_sensitivity() do
      {:noreply,
       socket
       |> assign(:loading, false)
       |> assign(:dashboard_data, dashboard)
       |> assign(:mcoa_counters, counters)
       |> assign(:cdata_sensitivity, sensitivity)
       |> assign(:error, nil)}
    else
      {:error, reason} ->
        {:noreply,
         socket
         |> assign(:loading, false)
         |> assign(:error, "Failed to load data: #{inspect(reason)}")}
    end
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-6">
      <.header>
        Ze Theory Dashboard
        <:subtitle>
          Counter S (Synchronization) — MCOA Framework
        </:subtitle>
      </.header>

      <.error_banner :if={@error}>
        <%= @error %>
      </.error_banner>

      <.loading_spinner :if={@loading} />

      <div :if={@dashboard_data && !@loading} class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Ze Parameters Card -->
        <div class="lg:col-span-2 space-y-6">
          <.card>
            <:title>Ze Parameters</:title>
            <:content>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <.parameter_card
                  title="v (Observed)"
                  value={format_number(@dashboard_data.v)}
                  subtitle="N_S/(N-1)"
                  color="blue"
                />
                <.parameter_card
                  title="v* passive"
                  value="0.3069"
                  subtitle="1 - ln 2 (theorem)"
                  color="gray"
                />
                <.parameter_card
                  title="v* active"
                  value="≈ 0.456"
                  subtitle="Empirical estimate (per-dataset pending)"
                  color="yellow"
                />
                <.parameter_card
                  title="θ_Z"
                  value="≈ 0.30"
                  subtitle="Prediction threshold"
                  color="green"
                />
              </div>

              <div class="mt-6">
                <h3 class="text-lg font-medium text-zinc-900 mb-2">Ze Formalism</h3>
                <div class="bg-zinc-50 p-4 rounded-lg text-sm font-mono">
                  <div>Ze = (H, ρ_Z, {M_i}, τ_Z, θ_Z)</div>
                  <div class="mt-2 text-zinc-600">
                    H: ℂ² (d=2) | ρ_Z: diagonal limit P_Z | τ_Z: <%= @dashboard_data.tau_z || 200 %> | θ_Z: 0.30
                  </div>
                </div>
              </div>
            </:content>
          </.card>

          <!-- MCOA Counter Registry -->
          <.card>
            <:title>MCOA Counter Registry</:title>
            <:content>
              <.table
                rows={@mcoa_counters}
                columns={[
                  %{label: "Counter", cell: & &1.name},
                  %{label: "Type", cell: & &1.type},
                  %{label: "Value", cell: &format_number(&1.value)},
                  %{label: "Status", cell: &(&1.status |> String.upcase() |> CoreComponents.badge(color: status_color(&1.status)))}
                ]}
              />
            </:content>
          </.card>
        </div>

        <!-- Right Column -->
        <div class="space-y-6">
          <!-- CDATA Sobol Sensitivity -->
          <.card>
            <:title>CDATA Sensitivity Analysis</:title>
            <:content>
              <div :if={@cdata_sensitivity} class="space-y-4">
                <div :for={param <- @cdata_sensitivity.parameters} class="space-y-2">
                  <div class="flex justify-between">
                    <span class="text-sm font-medium text-zinc-700"><%= param.name %></span>
                    <span class="text-sm text-zinc-500"><%= format_number(param.sobol_index) %></span>
                  </div>
                  <.progress_bar
                    value={round(param.sobol_index * 100)}
                    max={100}
                    color="indigo"
                    label={param.description}
                  />
                </div>
              </div>
              <div :if={!@cdata_sensitivity} class="text-center py-4 text-zinc-500">
                No sensitivity data available
              </div>
            </:content>
          </.card>

          <!-- BioSense Stream -->
          <.card>
            <:title>BioSense Raw Stream</:title>
            <:content>
              <div class="text-sm text-zinc-600 mb-4">
                Raw sensor data display. No biomarker claims per CORRECTIONS_2026-04-22.
              </div>
              <div class="space-y-3">
                <div class="flex justify-between">
                  <span class="text-zinc-700">EEG Gamma (25-35 Hz)</span>
                  <span class="font-mono">-<%= :rand.uniform(50) %> µV</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-zinc-700">HRV RMSSD</span>
                  <span class="font-mono"><%= :rand.uniform(100) %> ms</span>
                </div>
              </div>
            </:content>
          </.card>

          <!-- Theorem Status -->
          <.card>
            <:title>Theorem Status</:title>
            <:content>
              <ul class="space-y-3 text-sm">
                <li class="flex items-start">
                  <.icon name="check-circle" class="h-5 w-5 text-green-500 mr-2 flex-shrink-0" />
                  <span>v*_passive = 1 - ln 2 (analytically derived)</span>
                </li>
                <li class="flex items-start">
                  <.icon name="exclamation-triangle" class="h-5 w-5 text-yellow-500 mr-2 flex-shrink-0" />
                  <span>Theorem 5.1: conditional (θ_Q < log₂(d))</span>
                </li>
                <li class="flex items-start">
                  <.icon name="check-circle" class="h-5 w-5 text-green-500 mr-2 flex-shrink-0" />
                  <span>Minkowski metric from Ze dynamics (published)</span>
                </li>
              </ul>
            </:content>
          </.card>
        </div>
      </div>
    </div>
    """
  end

  defp format_number(nil), do: "—"
  defp format_number(num) when is_number(num), do: :io_lib.format("~.4f", [num]) |> List.to_string()
  defp format_number(other), do: inspect(other)

  defp status_color("active"), do: "green"
  defp status_color("inactive"), do: "gray"
  defp status_color("error"), do: "red"
  defp status_color(_), do: "gray"

  defp header(assigns) do
    ~H"""
    <div class="md:flex md:items-center md:justify-between">
      <div class="min-w-0 flex-1">
        <h2 class="text-2xl font-bold leading-7 text-zinc-900 sm:truncate sm:text-3xl sm:tracking-tight">
          <%= render_slot(@inner_block) %>
        </h2>
        <p :if={@subtitle} class="mt-1 text-sm text-zinc-500">
          <%= render_slot(@subtitle) %>
        </p>
      </div>
    </div>
    """
  end

  defp card(assigns) do
    ~H"""
    <div class="bg-white shadow rounded-lg">
      <div class="px-4 py-5 sm:p-6">
        <h3 class="text-lg font-medium leading-6 text-zinc-900 mb-4">
          <%= render_slot(@title) %>
        </h3>
        <div class="mt-2">
          <%= render_slot(@content) %>
        </div>
      </div>
    </div>
    """
  end

  defp parameter_card(assigns) do
    ~H"""
    <div class="bg-white p-4 rounded-lg border border-zinc-200">
      <div class="flex items-center">
        <div class="flex-1">
          <p class="text-sm font-medium text-zinc-500"><%= @title %></p>
          <p class="mt-1 text-2xl font-semibold text-zinc-900"><%= @value %></p>
          <p class="mt-1 text-sm text-zinc-500"><%= @subtitle %></p>
        </div>
        <div class={["h-10 w-1 rounded-full", "bg-#{@color}-500"]} />
      </div>
    </div>
    """
  end

  defp error_banner(assigns) do
    ~H"""
    <div class="rounded-md bg-red-50 p-4">
      <div class="flex">
        <div class="flex-shrink-0">
          <.icon name="exclamation-circle" class="h-5 w-5 text-red-400" />
        </div>
        <div class="ml-3">
          <p class="text-sm font-medium text-red-800">
            <%= render_slot(@inner_block) %>
          </p>
        </div>
      </div>
    </div>
    """
  end

  defp loading_spinner(assigns) do
    ~H"""
    <div class="flex justify-center py-12">
      <.spinner class="h-8 w-8" />
      <span class="ml-3 text-zinc-600">Loading dashboard data...</span>
    </div>
    """
  end
end