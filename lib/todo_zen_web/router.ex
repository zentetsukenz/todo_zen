defmodule TodoZenWeb.Router do
  use TodoZenWeb, :router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/api", TodoZenWeb do
    pipe_through :api
  end
end
