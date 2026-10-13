require 'minitest/autorun'
require_relative 'solution'

class Test < Minitest::Test
  def test_1
    assert_equal 3, rob([2, 3, 2])
  end

  def test_2
    assert_equal 4, rob([1, 2, 3, 1])
  end

  def test_3
    assert_equal 3, rob([1, 2, 3])
  end

  def test_4
    assert_equal 6, rob([1, 2, 3, 1, 2, 3])
  end

  def test_5
    assert_equal 27, rob([6,6,4,8,4,3,3,10])
  end

end
