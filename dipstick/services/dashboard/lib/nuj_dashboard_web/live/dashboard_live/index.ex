# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
defmodule NujDashboardWeb.DashboardLive.Index do
  use NujDashboardWeb, :live_view

  alias NujDashboard.Monitor

  @impl true
  def mount(_params, _session, socket) do
    if connected?(socket) do
      :timer.send_interval(5000, self(), :update_stats)
    end

    {:ok, assign(socket, :stats, fetch_stats())}
  end

  @impl true
  def handle_info(:update_stats, socket) do
    {:noreply, assign(socket, :stats, fetch_stats())}
  end

  defp fetch_stats do
    %{
      platforms_monitored: Monitor.count_active_platforms(),
      changes_detected_today: Monitor.count_changes_today(),
      pending_reviews: Monitor.count_pending_reviews(),
      active_publications: Monitor.count_active_publications(),
      recent_changes: Monitor.list_recent_changes(10)
    }
  end
end
