defmodule ZeWeb.BackendClient do
  @moduledoc """
  Thin HTTP client for the Rust ze-backend on :4001.
  Every public function returns `{:ok, map}` or `{:error, String.t()}`.
  """

  require Logger

  defp base_url, do: Application.get_env(:ze_web, :backend_url) || "http://127.0.0.1:4001"

  defp post(path, body) do
    url = base_url() <> path

    case Req.post(url, json: body, receive_timeout: 5_000) do
      {:ok, %Req.Response{status: 200, body: payload}} ->
        {:ok, payload}

      {:ok, %Req.Response{status: status, body: payload}} ->
        msg =
          case payload do
            %{"error" => %{"message" => m}} -> m
            other -> "HTTP #{status}: #{inspect(other)}"
          end

        {:error, msg}

      {:error, reason} ->
        Logger.warning("ze-backend POST #{path} failed: #{inspect(reason)}")
        {:error, "backend unreachable: #{inspect(reason)}"}
    end
  end

  def chsh(delta, opts \\ []) do
    post("/api/chsh", %{
      delta: delta,
      optimizer: Keyword.get(opts, :optimizer, "planar-grid"),
      n: Keyword.get(opts, :n, 128)
    })
  end

  def correlation(c0, beta, i, tau_grid) do
    post("/api/correlation", %{c0: c0, beta: beta, i: i, tau_grid: tau_grid})
  end

  def qfi(c0, beta, i, tau \\ nil) do
    body = %{c0: c0, beta: beta, i: i}
    body = if tau, do: Map.put(body, :tau, tau), else: body
    post("/api/qfi", body)
  end

  def qfi_sweep(c0, beta, i_grid) do
    post("/api/qfi_sweep", %{c0: c0, beta: beta, i_grid: i_grid})
  end

  def healthz do
    case Req.get(base_url() <> "/healthz", receive_timeout: 1_500) do
      {:ok, %Req.Response{status: 200, body: payload}} -> {:ok, payload}
      {:ok, %Req.Response{status: s}} -> {:error, "HTTP #{s}"}
      {:error, e} -> {:error, inspect(e)}
    end
  end
end
