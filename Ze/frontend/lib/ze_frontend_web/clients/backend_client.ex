defmodule ZeFrontendWeb.BackendClient do
  @moduledoc """
  HTTP client for communicating with the Rust backend API.
  Uses Req with retries, telemetry, and proper error handling.
  """

  use Telemetry.TelemetryEvents, prefix: [:ze_frontend, :backend_client]

  @base_url Application.compile_env(:ze_frontend, ZeFrontendWeb.BackendClient)[:base_url]
  @timeout Application.compile_env(:ze_frontend, ZeFrontendWeb.BackendClient)[:timeout]
  @max_retries Application.compile_env(:ze_frontend, ZeFrontendWeb.BackendClient)[:max_retries]
  @retry_delay Application.compile_env(:ze_frontend, ZeFrontendWeb.BackendClient)[:retry_delay]

  @spec get_dashboard() ::
          {:ok, map()} | {:error, :timeout | :connection_failed | :invalid_response | term()}
  def get_dashboard do
    request(:get, "/api/ze/dashboard")
  end

  @spec get_mcoa_counters() ::
          {:ok, list(map())} | {:error, :timeout | :connection_failed | :invalid_response | term()}
  def get_mcoa_counters do
    request(:get, "/api/mcoa/counters")
  end

  @spec get_cdata_sensitivity() ::
          {:ok, map()} | {:error, :timeout | :connection_failed | :invalid_response | term()}
  def get_cdata_sensitivity do
    request(:get, "/api/cdata/sensitivity")
  end

  @spec get_entity(String.t()) ::
          {:ok, map()} | {:error, :timeout | :connection_failed | :invalid_response | :not_found | term()}
  def get_entity(id) do
    request(:get, "/api/ze/entities/#{id}")
  end

  @spec get_hsc_lineage(String.t()) ::
          {:ok, list(map())} | {:error, :timeout | :connection_failed | :invalid_response | term()}
  def get_hsc_lineage(entity_id) do
    request(:get, "/api/cdata/lineage/#{entity_id}")
  end

  defp request(method, path) do
    url = @base_url <> path

    telemetry_metadata = %{
      method: method,
      url: url,
      path: path,
      attempt: 1
    }

    :telemetry.execute([:ze_frontend, :backend_client, :request, :start], %{}, telemetry_metadata)

    result =
      with_req do
        Req.request(
          method: method,
          url: url,
          receive_timeout: @timeout,
          retry: :transient,
          max_retries: @max_retries,
          retry_delay: fn attempt -> @retry_delay * :math.pow(2, attempt - 1) end
        )
      end

    case result do
      {:ok, %{status: 200, body: body}} ->
        :telemetry.execute(
          [:ze_frontend, :backend_client, :request, :stop],
          %{duration: System.monotonic_time() - telemetry_metadata.start_time},
          Map.put(telemetry_metadata, :status, :success)
        )

        {:ok, body}

      {:ok, %{status: 404}} ->
        :telemetry.execute(
          [:ze_frontend, :backend_client, :request, :stop],
          %{duration: System.monotonic_time() - telemetry_metadata.start_time},
          Map.put(telemetry_metadata, :status, :not_found)
        )

        {:error, :not_found}

      {:ok, %{status: status}} ->
        :telemetry.execute(
          [:ze_frontend, :backend_client, :request, :stop],
          %{duration: System.monotonic_time() - telemetry_metadata.start_time},
          Map.put(telemetry_metadata, :status, :error)
        )

        {:error, {:http_error, status}}

      {:error, %Mint.TransportError{reason: :timeout}} ->
        :telemetry.execute(
          [:ze_frontend, :backend_client, :request, :stop],
          %{duration: System.monotonic_time() - telemetry_metadata.start_time},
          Map.put(telemetry_metadata, :status, :timeout)
        )

        {:error, :timeout}

      {:error, %Mint.TransportError{reason: reason}} ->
        :telemetry.execute(
          [:ze_frontend, :backend_client, :request, :stop],
          %{duration: System.monotonic_time() - telemetry_metadata.start_time},
          Map.put(telemetry_metadata, :status, :connection_failed)
        )

        {:error, {:connection_failed, reason}}

      {:error, %Jason.DecodeError{} = error} ->
        :telemetry.execute(
          [:ze_frontend, :backend_client, :request, :stop],
          %{duration: System.monotonic_time() - telemetry_metadata.start_time},
          Map.put(telemetry_metadata, :status, :invalid_response)
        )

        {:error, {:invalid_response, error}}

      {:error, reason} ->
        :telemetry.execute(
          [:ze_frontend, :backend_client, :request, :stop],
          %{duration: System.monotonic_time() - telemetry_metadata.start_time},
          Map.put(telemetry_metadata, :status, :error)
        )

        {:error, reason}
    end
  end

  defp with_req(fun) do
    try do
      fun.()
    rescue
      e in RuntimeError ->
        {:error, e}
    end
  end
end