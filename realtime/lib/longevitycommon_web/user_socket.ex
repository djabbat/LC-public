defmodule LCRealtimeWeb.UserSocket do
  use Phoenix.Socket

  channel "feed:*",   LCRealtimeWeb.FeedChannel
  channel "ze_clock", LCRealtimeWeb.ZeClockChannel
  channel "study:*",  LCRealtimeWeb.StudyChannel

  @impl true
  def connect(%{"token" => token}, socket, _connect_info) do
    case LCRealtime.Auth.verify_token(token) do
      {:ok, user_id} ->
        {:ok, assign(socket, :user_id, user_id)}
      {:error, _} ->
        :error
    end
  end

  def connect(_params, _socket, _connect_info), do: :error

  @impl true
  def id(socket), do: "user_socket:#{socket.assigns.user_id}"
end
