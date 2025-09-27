defmodule SolutionTest do
  use ExUnit.Case

  test "triangle_area" do
    actual = Solution.triangle_area([1, 1], [2, 2], [3, 1])
    expected = 1
    assert expected == actual
  end

  test "1" do
    actual = Solution.largest_triangle_area([[0,0], [0,1], [1,0], [0,2], [2,0]])
    expected = 2
    assert expected == actual
  end

  test "2" do
    actual = Solution.largest_triangle_area([[1,0], [0,0], [0,1]])
    expected = 0.5
    assert expected == actual
  end

end
