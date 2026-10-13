defmodule SolutionTest do
  use ExUnit.Case

  test "1" do
    actual = Solution.solve([1, 2, 3])
    expected = 3
    assert expected == actual
  end

end
