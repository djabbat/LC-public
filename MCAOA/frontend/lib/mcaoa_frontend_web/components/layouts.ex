defmodule MCAOAFrontendWeb.Layouts do
  use MCAOAFrontendWeb, :html

  embed_templates "layouts/*"

  def live_socket_path(assigns) do
    ~H"""
    <script>
      window.liveSocketPath = "<%= assigns[:socket_path] || "/live" %>"
    </script>
    """
  end
end