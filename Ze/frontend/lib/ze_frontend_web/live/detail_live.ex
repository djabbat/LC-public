defmodule ZeFrontendWeb.DetailLive do
  use ZeFrontendWeb, :live_view

  alias ZeFrontendWeb.BackendClient
  alias ZeFrontendWeb.CoreComponents

  @impl true
  def mount(%{"id" => id}, _session, socket) do
    if connected?(socket), do: Process.send_after(self(), {:load_entity, id}, 100)

    {:ok,
     socket
     |> assign(:loading, true)
     |> assign(:entity, nil)
     |> assign(:hsc_lineage, [])
     |> assign(:error, nil)
     |> assign(:entity_id, id)}
  end

  def mount(_params, _session, socket) do
    {:ok,
     socket
     |> assign(:loading, false)
     |> assign(:entity, nil)
     |> assign(:error, "No entity ID provided")
     |> push_navigate(to: ~p"/")}
  end

  @impl true
  def handle_params(_params, _uri, socket) do
    {:noreply, assign(socket, page_title: "Entity Detail - Ze Theory")}
  end

  @impl true
  def handle_event("refresh", _params, socket) do
    {:noreply,
     socket
     |> assign(:loading, true)
     |> push_event("refresh_start", %{})}
  end

  @impl true
  def handle_info({:load_entity, id}, socket) do
    with {:ok, entity} <- BackendClient.get_entity(id),
         {:ok, lineage} <- BackendClient.get_hsc_lineage(id) do
      {:noreply,
       socket
       |> assign(:loading, false)
       |> assign(:entity, entity)
       |> assign(:hsc_lineage, lineage)
       |> assign(:error, nil)}
    else
      {:error, reason} ->
        {:noreply,
         socket
         |> assign(:loading, false)
         |> assign(:error, "Failed to load entity: #{inspect(reason)}")}
    end
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="space-y-6">
      <.header>
        Entity Detail
        <:subtitle>
          ID: <%= @entity_id %>
        </:subtitle>
        <:actions>
          <.button phx-click="refresh" disabled={@loading}>
            <.icon name="arrow-path" class="h-4 w-4 mr-2" />
            Refresh
          </.button>
        </:actions>
      </.header>

      <.error_banner :if={@error}>
        <%= @error %>
      </.error_banner>

      <.loading_spinner :if={@loading} />

      <div :if={@entity && !@loading} class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Main Entity Data -->
        <div class="lg:col-span-2 space-y-6">
          <.card>
            <:title>Ze Entity State</:title>
            <:content>
              <dl class="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2">
                <.detail_item label="v (N_S/(N-1))">
                  <%= format_number(@entity.v) %>
                </.detail_item>
                <.detail_item label="τ_Z">
                  <%= @entity.tau_z %>
                </.detail_item>
                <.detail_item label="θ_Z">
                  <%= @entity.theta_z %>
                </.detail_item>
                <.detail_item label="State">
                  <CoreComponents.badge color={state_color(@entity.state)}>
                    <%= String.upcase(@entity.state) %>
                  </CoreComponents.badge>
                </.detail_item>
                <.detail_item label="Created">
                  <%= format_date(@entity.created_at) %>
                </.detail_item>
                <.detail_item label="Last Updated">
                  <%= format_date(@entity.updated_at) %>
                </.detail_item>
              </dl>

              <div class="mt-6">
                <h4 class="text-sm font-medium text-zinc-900 mb-2">P_Z (Probability Vector)</h4>
                <div class="bg-zinc-50 p-3 rounded-lg">
                  <code class="text-sm font-mono">
                    (<%= Enum.join(Enum.map(@entity.p_z, &format_number/1), ", ") %>)
                  </code>
                </div>
              </div>
            </:content>
          </.card>

          <!-- Event Stream -->
          <.card>
            <:title>Event History</:title>
            <:content>
              <div class="flow-root">
                <ul role="list" class="-mb-8">
                  <li :for={event <- @entity.events} class="relative pb-8">
                    <div class="relative flex items-start space-x-3">
                      <div class="relative">
                        <div class={["h-8 w-8 rounded-full flex items-center justify-center ring-8 ring-white", event_color(event.type)]}>
                          <.icon name={event_icon(event.type)} class="h-4 w-4 text-white" />
                        </div>
                      </div>
                      <div class="min-w-0 flex-1">
                        <div>
                          <div class="text-sm">
                            <span class="font-medium text-zinc-900"><%= event.type %> event</span>
                          </div>
                          <p class="mt-0.5 text-sm text-zinc-500">
                            <%= event.description %>
                          </p>
                        </div>
                        <div class="mt-2 text-sm text-zinc-700">
                          <p><%= format_date(event.timestamp) %></p>
                        </div>
                      </div>
                    </div>
                  </li>
                </ul>
              </div>
            </:content>
          </.card>
        </div>

        <!-- Sidebar -->
        <div class="space-y-6">
          <!-- HSC Lineage Tracking -->
          <.card>
            <:title>HSC Lineage</:title>
            <:content>
              <div :if={Enum.empty?(@hsc_lineage)} class="text-center py-4 text-zinc-500">
                No lineage data available
              </div>
              <div :if={!Enum.empty?(@hsc_lineage)} class="space-y-4">
                <div :for={node <- @hsc_lineage} class="flex items-center">
                  <div class={["h-3 w-3 rounded-full mr-3", lineage_color(node.generation)]} />
                  <div class="flex-1">
                    <div class="text-sm font-medium text-zinc-900">Generation <%= node.generation %></div>
                    <div class="text-xs text-zinc-500"><%= node.cell_count %> cells</div>
                  </div>
                </div>
              </div>
            </:content>
          </.card>

          <!-- τ_Z Counter -->
          <.card>
            <:title>τ_Z Prediction Counter</:title>
            <:content>
              <div class="text-center py-4">
                <div class="text-4xl font-bold text-zinc-900"><%= @entity.tau_z %></div>
                <div class="text-sm text-zinc-500 mt-2">remaining prediction units</div>
                <.progress_bar
                  value={@entity.tau_z}
                  max={200}
                  color="blue"
                  label="τ_Z depletion"
                  class="mt-4"
                />
              </div>
            </:content>
          </.card>

          <!-- Quick Actions -->
          <.card>
            <:title>Actions</:title>
            <:content>
              <div class="space-y-3">
                <.button variant="outline" class="w-full justify-center">
                  <.icon name="plus-circle" class="h-4 w-4 mr-2" />
                  Add T-event
                </.button>
                <.button variant="outline" class="w-full justify-center">
                  <.icon name="minus-circle" class="h-4 w-4 mr-2" />
                  Add S-event
                </.button>
                <.button variant="outline" color="red" class="w-full justify-center">
                  <.icon name="trash" class="h-4 w-4 mr-2" />
                  Reset Entity
                </.button>
              </div>
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

  defp format_date(nil), do: "—"
  defp format_date(datetime), do: Calendar.strftime(datetime, "%Y-%m-%d %H:%M:%S")

  defp state_color("active"), do: "green"
  defp state_color("inactive"), do: "gray"
  defp state_color("error"), do: "red"
  defp state_color(_), do: "gray"

  defp event_color("T"), do: "bg-red-500"
  defp event_color("S"), do: "bg-green-500"
  defp event_color(_), do: "bg-zinc-500"

  defp event_icon("T"), do: "arrow-up"
  defp event_icon("S"), do: "arrow-down"
  defp event_icon(_), do: "question-mark-circle"

  defp lineage_color(gen) when rem(gen, 2) == 0, do: "bg-blue-500"
  defp lineage_color(_), do: "bg-indigo-400"

  # Reuse components from DashboardLive
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
      <div :if={@actions} class="mt-4 flex md:ml-4 md:mt-0">
        <%= render_slot(@actions) %>
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

  defp detail_item(assigns) do
    ~H"""
    <div>
      <dt class="text-sm font-medium text-zinc-500"><%= @label %></dt>
      <dd class="mt-1 text-sm text-zinc-900">
        <%= render_slot(@inner_block) %>
      </dd>
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
      <span class="ml-3 text-zinc-600">Loading entity data...</span>
    </div>
    """
  end
end