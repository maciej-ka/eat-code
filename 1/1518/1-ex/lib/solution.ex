# https://leetcode.com/problems/water-bottles/submissions/1788326130/?envType=daily-question&envId=2025-10-01

defmodule Solution do
  @spec num_water_bottles(num_bottles :: integer, num_exchange :: integer) :: integer
  def num_water_bottles(num_bottles, num_exchange) do
    solve(num_bottles, num_exchange, num_bottles)
  end

  def solve(n, price, acc) do
    d = div(n, price)
    r = rem(n, price)

    if d == 0 do
      acc
    else
      solve(d + r, price, acc + d)
    end
  end
end
