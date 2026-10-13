# https://leetcode.com/problems/find-triangular-sum-of-an-array/submissions/1786915558/

defmodule Solution do
  @spec triangular_sum(nums :: [integer]) :: integer
  def triangular_sum([a]), do: a
  def triangular_sum(ls), do: triangular_sum(ls, [])
  def triangular_sum([a, b | rest], ls) do triangular_sum([b | rest], [Integer.mod(a + b, 10) | ls])
  def triangular_sum([_], ls), do: triangular_sum(ls)
end
