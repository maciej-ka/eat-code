defmodule SolutionTest do
  use ExUnit.Case

  test "empty" do
    assert Solution.rob([]) == 0
  end

  test "one" do
    assert Solution.rob([2]) == 2
  end

  test "pair" do
    assert Solution.rob([2, 3]) == 3
  end

  test "case 1" do
    assert Solution.rob([2, 3, 2]) == 3
  end

  test "case 2" do
    assert Solution.rob([1, 2, 3, 1]) == 4
  end

  test "case 3" do
    assert Solution.rob([1, 2, 3]) == 3
  end

  test "case 4" do
    assert Solution.rob([1, 2, 3, 1, 2, 3]) == 6
  end

  test "case 5" do
    assert Solution.rob([6, 6, 4, 8, 4, 3, 3, 10]) == 27
  end

  test "case 6" do
    assert Solution.rob([104, 209, 137, 52, 158, 67, 213, 86, 141, 110, 151, 127, 238, 147, 169, 138, 240, 185, 246, 225, 147, 203, 83, 83, 131, 227, 54, 78, 165, 180, 214, 151, 111, 161, 233, 147, 124, 143]) == 3157
  end

  test "case 7" do
    assert Solution.rob([114, 117, 207, 117, 235, 82, 90, 67, 143, 146, 53, 108, 200, 91, 80, 223, 58, 170, 110, 236, 81, 90, 222, 160, 165, 195, 187, 199, 114, 235, 197, 187, 69, 129, 64, 214, 228, 78, 188, 67, 205, 94, 205, 169, 241, 202, 144, 240]) == 4077
  end
end
