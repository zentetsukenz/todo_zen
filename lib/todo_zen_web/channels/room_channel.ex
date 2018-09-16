defmodule TodoZenWeb.RoomChannel do
  use Phoenix.Channel

  def join(channel_name, _params, socket) do
    {:ok, %{channel_name: channel_name}, socket}
  end
end
