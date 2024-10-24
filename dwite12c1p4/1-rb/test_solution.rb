require 'minitest/autorun'
require_relative 'solution'

class Test < Minitest::Test
  def test_1
    assert_equal "3 4", solve("(1 3)")
  end

  def test_2
    assert_equal "6 14", solve("((1 5) 8)")
  end
end
