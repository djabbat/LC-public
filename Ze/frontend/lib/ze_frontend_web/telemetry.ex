defmodule ZeFrontendWeb.Telemetry do
  use Supervisor
  import Telemetry.Metrics

  def start_link(arg) do
    Supervisor.start_link(__MODULE__, arg, name: __MODULE__)
  end

  @impl true
  def init(_arg) do
    children = [
      {:telemetry_poller, measurements: periodic_measurements(), period: 10_000}
    ]

    Supervisor.init(children, strategy: :one_for_one)
  end

  def metrics do
    [
      counter("phoenix.endpoint.stop.duration",
        unit: {:native, :millisecond}
      ),
      counter("phoenix.router_dispatch.stop.duration",
        tags: [:route],
        unit: {:native, :millisecond}
      ),
      summary("ze_frontend.backend_client.request.duration",
        tags: [:endpoint, :status],
        unit: {:native, :millisecond}
      ),
      counter("ze_frontend.backend_client.request.count",
        tags: [:endpoint, :status]
      ),
      last_value("ze_frontend.memory.usage", unit: :byte),
      last_value("ze_frontend.live_view.count")
    ]
  end

  defp periodic_measurements do
    [
      {Process, :info, [self(), :memory]},
      {ZeFrontendWeb.Telemetry, :live_view_count, []}
    ]
  end

  def live_view_count do
    count =
      Phoenix.LiveView.Server.count()
      |> Kernel.||(0)

    :telemetry.execute([:ze_frontend, :live_view, :count], %{count: count}, %{})
  end
end