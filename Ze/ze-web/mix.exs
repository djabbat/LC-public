defmodule ZeWeb.MixProject do
  use Mix.Project

  def project do
    [
      app: :ze_web,
      version: "0.1.0",
      elixir: "~> 1.17",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: Mix.compilers(),
      aliases: aliases()
    ]
  end

  def application do
    [
      mod: {ZeWeb.Application, []},
      extra_applications: [:logger, :runtime_tools]
    ]
  end

  defp deps do
    [
      {:phoenix, "~> 1.7.14"},
      {:phoenix_html, "~> 4.0"},
      {:phoenix_live_reload, "~> 1.5", only: :dev},
      {:phoenix_live_view, "~> 1.0"},
      {:bandit, "~> 1.5"},
      {:jason, "~> 1.4"},
      {:req, "~> 0.5"},
      {:plug_cowboy, "~> 2.7", only: :test}
    ]
  end

  defp aliases do
    [setup: ["deps.get"]]
  end
end
