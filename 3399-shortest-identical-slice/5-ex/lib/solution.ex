defmodule Solution do
  # @spec min_length(s :: String.t, num_ops :: integer) :: integer
  def min_length(s, num_ops) do
    cond do
      solution_one(s, num_ops, ?0) -> 1
      solution_one(s, num_ops, ?1) -> 1
      :true -> solution_n(s, num_ops)
    end
  end

  def solution_one(_, num_ops, _) when num_ops < 0, do: :false
  def solution_one("", _, _), do: :true
  def solution_one(<<a, rest::binary>>, num_ops, a), do: solution_one(rest, num_ops, flip(a))
  def solution_one(<<_, rest::binary>>, num_ops, a), do: solution_one(rest, num_ops - 1, flip(a))

  def flip(?0), do: ?1
  def flip(?1), do: ?0

  def solution_n(_, _), do: 2

  def lengths(<<a, rest::binary>>), do: lengths(rest, a, 1, %{})
  def lengths(<<a, rest::binary>>, a, len, acc), do: lengths(rest, a, len + 1, acc)
  def lengths(<<a, rest::binary>>, _, len, acc) do
    count = Map.get(acc, len, 0)
    acc = Map.put(acc, len, count + 1)
    lengths(rest, a, 1, acc)
  end
  def lengths("", _, len, acc) do
    count = Map.get(acc, len, 0)
    Map.put(acc, len, count + 1)
  end
end
