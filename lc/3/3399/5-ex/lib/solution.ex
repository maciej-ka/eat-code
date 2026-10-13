defmodule Solution do
  # @spec min_length(s :: String.t, num_ops :: integer) :: integer
  def min_length(s, num_ops) do
    cond do
      solution_one(s, num_ops, ?0) -> 1
      solution_one(s, num_ops, ?1) -> 1
      :true ->
        counts = count_lengths(s)
        lengths = Enum.sort(Map.keys(counts), &>=/2)
        score_possible? = fn score -> score_possible?(lengths, counts, num_ops, score) end
        find_solution(score_possible?, 2, hd(lengths))
    end
  end

  def solution_one(_, num_ops, _) when num_ops < 0, do: :false
  def solution_one("", _, _), do: :true
  def solution_one(<<a, rest::binary>>, num_ops, a), do: solution_one(rest, num_ops, flip(a))
  def solution_one(<<_, rest::binary>>, num_ops, a), do: solution_one(rest, num_ops - 1, flip(a))

  def flip(?0), do: ?1
  def flip(?1), do: ?0

  def count_lengths(<<a, rest::binary>>), do: count_lengths(rest, a, 1, %{})
  def count_lengths(<<a, rest::binary>>, a, length, acc), do: count_lengths(rest, a, length + 1, acc)
  def count_lengths(<<a, rest::binary>>, _, length, acc) do
    count = Map.get(acc, length, 0)
    acc = Map.put(acc, length, count + 1)
    count_lengths(rest, a, 1, acc)
  end
  def count_lengths("", _, length, acc) do
    count = Map.get(acc, length, 0)
    Map.put(acc, length, count + 1)
  end

  def score_possible?(lengths, counts, num_ops, score), do: score_possible?(lengths, counts, num_ops, score, 0)
  def score_possible?(_, _, num_ops, _, acc) when acc > num_ops, do: :false
  def score_possible?([], _, _, _, _), do: :true
  def score_possible?([length | rest], counts, num_ops, score, acc) do
    inc = div(length, score + 1) * counts[length]
    score_possible?(rest, counts, num_ops, score, acc + inc)
  end

  # def find_solution(score_possible?, lo, hi)
  def find_solution(_, lo, hi) when lo >= hi, do: lo
  def find_solution(score_possible?, lo, hi) do
    m = div(lo + hi, 2)
    if score_possible?.(m) do
      find_solution(score_possible?, lo, m)
    else
      find_solution(score_possible?, m + 1, hi)
    end
  end
end

