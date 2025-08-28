require 'minitest/autorun'
require_relative 'solution'

class Test < Minitest::Test

  def test_1
    actual = solve([1, 2, 3])
    expected = 3
    assert_equal expected, actual
  end

end
