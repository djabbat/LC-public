defmodule BiosenseWeb.BackendClient do
  @moduledoc """
  Thin HTTP client for the Rust biosense-backend on :4101.
  Returns `{:ok, map}` or `{:error, reason}`.
  """

  require Logger

  defp base_url, do: Application.get_env(:biosense_web, :backend_url) || "http://127.0.0.1:4101"

  defp post(path, body) do
    case Req.post(base_url() <> path, json: body, receive_timeout: 5_000) do
      {:ok, %Req.Response{status: 200, body: payload}} -> {:ok, payload}
      {:ok, %Req.Response{status: s, body: payload}} ->
        msg = case payload do
          %{"error" => %{"message" => m}} -> m
          other -> "HTTP #{s}: #{inspect(other)}"
        end
        {:error, msg}
      {:error, e} ->
        Logger.warning("biosense-backend POST #{path} failed: #{inspect(e)}")
        {:error, "backend unreachable"}
    end
  end

  def chi_ze(eeg, hrv, resp, sleep), do: post("/api/chi_ze", %{eeg: eeg, hrv: hrv, resp: resp, sleep: sleep})
  def bridge(d), do: post("/api/bridge", %{d: d})
  def exacerbation(age, sex, chi_now, chi_7d), do:
    post("/api/exacerbation", %{age: age, sex: sex, chi_now: chi_now, chi_7d_ago: chi_7d})
  def pred_info(p), do: post("/api/pred_info", %{p: p})

  def healthz do
    case Req.get(base_url() <> "/healthz", receive_timeout: 1_500) do
      {:ok, %Req.Response{status: 200, body: payload}} -> {:ok, payload}
      _ -> {:error, "down"}
    end
  end
end
