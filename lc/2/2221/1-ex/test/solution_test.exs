defmodule SolutionTest do
  use ExUnit.Case

  test "1" do
    actual = Solution.triangular_sum([1,2,3,4,5])
    expected = 8
    assert expected == actual
  end

  test "2" do
    actual = Solution.triangular_sum([5])
    expected = 5
    assert expected == actual
  end

end
