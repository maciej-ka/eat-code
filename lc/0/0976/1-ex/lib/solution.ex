# https://leetcode.com/problems/largest-perimeter-triangle/submissions/1785368369/

defmodule Solution do
  @spec largest_perimeter(nums :: [integer]) :: integer
  def largest_perimeter(nums), do: Enum.sort(nums, :desc) |> solve
  def solve([a, b, c | _]) when b + c > a, do: a + b + c
  def solve([_ | [_, _, _ | _] = xs]), do: solve(xs)
  def solve(_), do: 0
end
