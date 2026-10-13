defmodule SolutionTest do
  use ExUnit.Case

  test "1" do
    actual = Solution.num_water_bottles(9, 3)
    expected = 13
    assert expected == actual
  end

  test "2" do
    actual = Solution.num_water_bottles(15, 4)
    expected = 19
    assert expected == actual
  end

end
