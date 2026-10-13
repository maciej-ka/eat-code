defmodule SolutionTest do
  use ExUnit.Case

  test "1" do
    actual = Solution.largest_perimeter([2, 1, 2])
    expected = 5
    assert expected == actual
  end

  test "2" do
    actual = Solution.largest_perimeter([1, 2, 1, 10])
    expected = 0
    assert expected == actual
  end

  test "3" do
    actual = Solution.largest_perimeter([3, 2, 3, 4])
    expected = 10
    assert expected == actual
  end

end
