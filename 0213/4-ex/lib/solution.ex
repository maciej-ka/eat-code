defmodule Solution do
  defp add({result, memo}, num), do: {result + num, memo}

  # rob houses in line
  def rob_line(arg), do: rob_line(arg, %{}) |> elem(0)
  def rob_line([], memo), do: {0, memo}
  def rob_line([one], memo), do: {one, memo}
  def rob_line(list, memo) do
    [current | [neighbour | rest]] = list
    case Map.get(memo, list) do
      nil ->
        {take_current, memo} = rob_line(rest, memo) |> add(current)
        {skip_current, memo} = rob_line([neighbour | rest], memo)
        result = max(take_current, skip_current)
        memo = Map.put(memo, list, result)
        {result, memo}
      result -> {result, memo}
    end
  end

  # rob cycled houses
  def rob([one]), do: one
  def rob(list) do
    without_first = Enum.slice(list, 1..-1//1)
    without_last = Enum.slice(list, 0..-2//1)
    max(
      rob_line(without_first),
      rob_line(without_last)
    )
  end
end
