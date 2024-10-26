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

end
