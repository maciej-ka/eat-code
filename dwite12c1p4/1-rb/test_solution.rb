require 'minitest/autorun'
require_relative 'solution'

class Test < Minitest::Test
  def test_1
    assert_equal "3 4", solve("(1 3)")
  end

  def test_2
    assert_equal "6 14", solve("((1 5) 8)")
  end

  def test_3
    assert_equal "10 10", solve("((1 2) (3 4))")
  end

  def test_4
    assert_equal "15 6", solve("(((((1 1) 1) 1) 1) 1)")
  end

  def test_5
    assert_equal "0 3", solve("3")
  end

  def test_6
    assert_equal "30 13", solve("((1 2) (((10 10) (3 4)) ((((1 1) 1) 1) 1)))")
  end

  def test_7
    assert_equal "25 40", solve("(((1 2) (3 4)) ((6 7) (8 9)))")
  end
end
