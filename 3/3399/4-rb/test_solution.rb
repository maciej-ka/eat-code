require 'minitest/autorun'
require_relative 'solution'

class Test < Minitest::Test
  def self.test_order
   :alpha
  end

  def test_1
    assert_equal 2, min_length("000001", 1)
  end

  def test_2
    assert_equal 1, min_length("0000", 2)
  end

  def test_3
    assert_equal 1, min_length("0101", 0)
  end

  def test_4
    assert_equal 2, min_length("011", 0)
  end

  def test_5
    assert_equal 1, min_length("00", 2) 
  end

  def test_6
    assert_equal 3, min_length("000", 0) 
  end

  def test_7
    assert_equal 1, min_length("0", 0) 
  end

  def test_8
    assert_equal 1, min_length("000", 1) 
  end
end
