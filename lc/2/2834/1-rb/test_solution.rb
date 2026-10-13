require 'minitest/autorun'
require_relative 'solution'

class Test < Minitest::Test
  def test_1
    assert_equal 4, minimum_possible_sum(2, 3)
  end

  def test_2
    assert_equal 8, minimum_possible_sum(3, 3)
  end

  def test_3
    assert_equal 1, minimum_possible_sum(1, 1)
  end

  def test_4
    assert_equal 91, minimum_possible_sum(13, 50)
  end

  def test_5
    assert_equal 156198582, minimum_possible_sum(39636, 49035)
  end

  def test_6
    assert_equal 15000000, minimum_possible_sum(100000000, 1000000000)
  end
end
