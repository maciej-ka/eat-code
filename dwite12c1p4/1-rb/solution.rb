require 'pp'

def solve(source)
  p = Parser.new(source)
  p.run()
end

class Parser
  def initialize(source)
    @source = source
    @position = 0
    @stack = []
  end

  def run()
    @position = 0
    @stack = []

    while @position < @source.length
      if (parse('('))
        # no-op
      elsif (parse(')'))
        @stack << Node.new(left: @stack.pop(), right: @stack.pop())
      elsif (value = parse_number())
        @stack << Node.new(value: value)
      else
        @position += 1
      end
    end

    @stack.pop()
  end

  def parse(string)
    tail = @source.slice(@position)
    return false unless tail.start_with?(string)
    @position += string.length
    string
  end

  def parse_number()
    result = @source.slice(@position)[/\A\d+/]
    return unless result
    @position += result.length
    result.to_i
  end
end

class Node
  attr_accessor :left, :right, :value

  def initialize(left: nil, right: nil, value: nil)
    @left = left
    @right = right
    @value = value
  end
end
