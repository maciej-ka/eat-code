defmodule SolutionTest do
  use ExUnit.Case

  test "case 1" do
    assert Solution.solution_one("", 1, ?0) == :true
  end

  test "case 2" do
    assert Solution.solution_one("", -1, ?0) == :false
  end

  test "case 3" do
    assert Solution.solution_one("0101", 0, ?0) == :true
  end

  test "case 4" do
    assert Solution.solution_one("0111", 1, ?0) == :true
  end

  test "case 5" do
    assert Solution.solution_one("01111", 1, ?0) == :false
  end

  test "case 6" do
    assert Solution.min_length("01111", 2) == 1
  end

  test "case 7" do
    assert Solution.min_length("10101", 0) == 1
  end

  test "case 8" do
    assert Solution.lengths("1") == %{1 => 1}
  end

  test "case 9" do
    assert Solution.lengths("100") == %{1 => 1, 2 => 1}
  end

  test "case 10" do
    assert Solution.lengths("000001") == %{1 => 1, 5 => 1}
  end

  test "case 11" do
    assert Solution.lengths("0000") == %{4 => 1}
  end

  test "case 12" do
    assert Solution.lengths("0101") == %{1 => 4}
  end
end
