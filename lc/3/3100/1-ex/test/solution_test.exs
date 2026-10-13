defmodule SolutionTest do
  use ExUnit.Case

  test "1" do
    actual = Solution.max_bottles_drunk(13, 6)
    expected = 15
    assert expected == actual
  end

  test "2" do
    actual = Solution.max_bottles_drunk(10, 3)
    expected = 13
    assert expected == actual
  end

end
