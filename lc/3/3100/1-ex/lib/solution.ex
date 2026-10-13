# https://leetcode.com/problems/water-bottles-ii/submissions/1789618489/?envType=daily-question&envId=2025-10-02

defmodule Solution do
  @spec max_bottles_drunk(num_bottles :: integer, num_exchange :: integer) :: integer
  def max_bottles_drunk(num_bottles, num_exchange) do
    solve(num_bottles, 0, num_exchange, 0)
  end

  def solve(full, empty, ex, res) when empty >= ex do
    solve(full + 1, empty - ex, ex + 1, res)
  end

  def solve(0, _, _, res), do: res

  def solve(full, empty, ex, res) do
    solve(0, empty + full, ex, res + full)
  end

end
