defmodule Solution do
  use Agent

  # rob houses in line
  def rob_line([]), do: 0
  def rob_line([one]), do: one
  def rob_line(list) do
    [current | [neighbour | rest]] = list
    case Agent.get(:memo, &Map.get(&1, list)) do
      nil ->
        take_current = current + rob_line(rest)
        skip_current = rob_line([neighbour | rest])
        result = max(take_current, skip_current)
        Agent.update(:memo, &Map.put(&1, list, result))
        result
      result -> result
    end
  end

  # rob cycled houses
  def rob([one]), do: one
  def rob(list) do
    Agent.start_link(fn -> %{} end, name: :memo)
    without_first = Enum.slice(list, 1..-1//1)
    without_last = Enum.slice(list, 0..-2//1)
    max(
      rob_line(without_first),
      rob_line(without_last)
    )
  end
end
