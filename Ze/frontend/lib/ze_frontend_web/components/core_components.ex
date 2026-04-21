defmodule ZeFrontendWeb.CoreComponents do
  @moduledoc """
  Provides core UI components.
  """
  use Phoenix.Component

  alias Phoenix.LiveView.JS

  @doc """
  Renders a modal.
  """
  attr :id, :string, required: true
  attr :show, :boolean, default: false
  attr :on_cancel, JS, default: %JS{}
  slot :inner_block, required: true

  def modal(assigns) do
    ~H"""
    <div
      id={@id}
      phx-mounted={@show && show_modal(@id)}
      phx-remove={hide_modal(@id)}
      class="relative z-50 hidden"
    >
      <div
        id={"#{@id}-bg"}
        class="fixed inset-0 bg-zinc-50/90 transition-opacity"
        aria-hidden="true"
      />
      <div
        class="fixed inset-0 overflow-y-auto"
        aria-labelledby={"#{@id}-title"}
        aria-modal="true"
        role="dialog"
      >
        <div class="flex min-h-full items-center justify-center p-4">
          <div class="w-full max-w-3xl">
            <.focus_wrap
              id={"#{@id}-container"}
              phx-window-keydown={JS.exec("phx-remove", to: "##{@id}")}
              phx-key="escape"
              phx-click-away={JS.exec("phx-remove", to: "##{@id}")}
              class="hidden relative rounded-2xl bg-white p-14 shadow-lg shadow-zinc-700/10 ring-1 ring-zinc-700/10 transition"
            >
              <div class="absolute top-6 right-5">
                <button
                  phx-click={JS.exec("phx-remove", to: "##{@id}")}
                  type="button"
                  class="-m-3 flex-none p-3 opacity-20 hover:opacity-40"
                  aria-label={gettext("Close")}
                >
                  <.icon name="hero-x-mark-solid" class="h-5 w-5" />
                </button>
              </div>
              <div id={"#{@id}-content"}>
                <%= render_slot(@inner_block) %>
              </div>
            </.focus_wrap>
          </div>
        </div>
      </div>
    </div>
    """
  end

  @doc """
  Renders a loading spinner.
  """
  attr :class, :string, default: nil

  def spinner(assigns) do
    ~H"""
    <svg
      class={["animate-spin h-5 w-5 text-zinc-400", @class]}
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      />
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      />
    </svg>
    """
  end

  @doc """
  Renders a badge.
  """
  attr :color, :string, default: "gray"
  slot :inner_block

  def badge(assigns) do
    colors = %{
      "gray" => "bg-zinc-100 text-zinc-800",
      "red" => "bg-red-100 text-red-800",
      "yellow" => "bg-yellow-100 text-yellow-800",
      "green" => "bg-green-100 text-green-800",
      "blue" => "bg-blue-100 text-blue-800",
      "indigo" => "bg-indigo-100 text-indigo-800",
      "purple" => "bg-purple-100 text-purple-800",
      "pink" => "bg-pink-100 text-pink-800"
    }

    ~H"""
    <span class={"inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium #{colors[@color]}"}>
      <%= render_slot(@inner_block) %>
    </span>
    """
  end

  @doc """
  Renders a progress bar.
  """
  attr :value, :integer, required: true
  attr :max, :integer, default: 100
  attr :color, :string, default: "blue"
  attr :label, :string, default: nil

  def progress_bar(assigns) do
    colors = %{
      "blue" => "bg-blue-600",
      "green" => "bg-green-600",
      "red" => "bg-red-600",
      "yellow" => "bg-yellow-600"
    }

    percentage = round(@value / @max * 100)

    ~H"""
    <div>
      <div class="flex justify-between mb-1">
        <span class="text-sm font-medium text-zinc-700"><%= @label %></span>
        <span class="text-sm font-medium text-zinc-700"><%= percentage %>%</span>
      </div>
      <div class="w-full bg-zinc-200 rounded-full h-2.5">
        <div
          class={"h-2.5 rounded-full #{colors[@color]}"}
          style={"width: #{percentage}%"}
        />
      </div>
    </div>
    """
  end

  @doc """
  Renders a table.
  """
  attr :rows, :list, required: true
  attr :row_click, :any, default: nil
  attr :columns, :list, required: true

  def table(assigns) do
    ~H"""
    <div class="overflow-x-auto">
      <table class="min-w-full divide-y divide-zinc-200">
        <thead class="bg-zinc-50">
          <tr>
            <th :for={col <- @columns} scope="col" class="px-6 py-3 text-left text-xs font-medium text-zinc-500 uppercase tracking-wider">
              <%= col[:label] %>
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-zinc-200">
          <tr :for={row <- @rows} class={"hover:bg-zinc-50": @row_click} phx-click={@row_click && @row_click.(row)}>
            <td :for={col <- @columns} class="px-6 py-4 whitespace-nowrap text-sm text-zinc-900">
              <%= render_slot(col[:cell], row) %>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    """
  end

  @doc """
  Renders an icon by name.
  """
  attr :name, :string, required: true
  attr :class, :string, default: nil

  def icon(assigns) do
    ~H"""
    <span class={["inline-block", @class]}>
      <Heroicons.LiveView.icon type="outline" name={@name} />
    </span>
    """
  end

  defp show_modal(js \\ %JS{}, id) do
    js
    |> JS.show(to: "##{id}")
    |> JS.show(
      to: "##{id}-bg",
      transition: {"transition-all transform ease-out duration-300", "opacity-0", "opacity-100"}
    )
    |> JS.show(
      to: "##{id}-container",
      transition:
        {"transition-all transform ease-out duration-300", "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95",
         "opacity-100 translate-y-0 sm:scale-100"}
    )
    |> JS.focus_first(to: "##{id}-content")
  end

  defp hide_modal(js \\ %JS{}, id) do
    js
    |> JS.hide(
      to: "##{id}-bg",
      transition: {"transition-all transform ease-in duration-200", "opacity-100", "opacity-0"}
    )
    |> JS.hide(
      to: "##{id}-container",
      transition:
        {"transition-all transform ease-in duration-200", "opacity-100 translate-y-0 sm:scale-100",
         "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"}
    )
    |> JS.hide(to: "##{id}", transition: {"block", "block", "hidden"})
  end
end