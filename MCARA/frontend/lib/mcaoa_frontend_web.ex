defmodule MCAOAFrontendWeb do
  def controller do
    quote do
      use Phoenix.Controller, namespace: MCAOAFrontendWeb
      import Plug.Conn
      import MCAOAFrontendWeb.Gettext
      alias MCAOAFrontendWeb.Router.Helpers, as: Routes
    end
  end

  def view do
    quote do
      use Phoenix.View,
        root: "lib/mcaoa_frontend_web/templates",
        namespace: MCAOAFrontendWeb

      import Phoenix.Controller,
        only: [get_flash: 1, get_flash: 2, view_module: 1]

      import Phoenix.LiveView.Helpers
      import MCAOAFrontendWeb.ErrorHelpers
      import MCAOAFrontendWeb.Gettext
      import MCAOAFrontendWeb.CoreComponents
      import MCAOAFrontendWeb.Layouts

      alias MCAOAFrontendWeb.Router.Helpers, as: Routes
    end
  end

  def router do
    quote do
      use Phoenix.Router
      import Plug.Conn
      import Phoenix.Controller
      import Phoenix.LiveView.Router
    end
  end

  def channel do
    quote do
      use Phoenix.Channel
      import MCAOAFrontendWeb.Gettext
    end
  end

  def live_view do
    quote do
      use Phoenix.LiveView,
        layout: {MCAOAFrontendWeb.Layouts, :app}

      on_mount MCAOAFrontendWeb.LiveMountHooks

      import MCAOAFrontendWeb.CoreComponents
      import MCAOAFrontendWeb.Gettext
      alias MCAOAFrontendWeb.Router.Helpers, as: Routes

      unquote(html_helpers())
    end
  end

  def live_component do
    quote do
      use Phoenix.LiveComponent

      unquote(html_helpers())
    end
  end

  defp html_helpers do
    quote do
      use Phoenix.HTML
      import Phoenix.LiveView.Helpers
      import MCAOAFrontendWeb.CoreComponents
      import MCAOAFrontendWeb.ErrorHelpers
      alias MCAOAFrontendWeb.Router.Helpers, as: Routes
    end
  end

  @doc """
  When used, dispatch to the appropriate controller/view/etc.
  """
  defmacro __using__(which) when is_atom(which) do
    apply(__MODULE__, which, [])
  end
end