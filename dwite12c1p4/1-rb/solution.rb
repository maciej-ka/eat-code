require 'pp'

def solve(source)
  parser = Parser.new(source)
  tree = parser.run()
  sum = 0
  path = 0
  tree.inorder do |value|
    sum += value
    path += 2
  end
  # don't count root
  path -= 2
  # assume kids ended furthest
  # so that return path saved is maxed out
  path -= parser.max_depth

  "#{path} #{sum}"
end

class Parser
  attr_reader :max_depth

  def initialize(source)
    @source = source
  end

  def run()
    @position = 0
    stack = []
    depth = 0
    @max_depth = 0

    while @position < @source.length
      if (parse('('))
        depth += 1
      elsif (parse(')'))
        depth -= 1
        stack << Node.new(left: stack.pop(), right: stack.pop())
      elsif (number = parse_number())
        stack << Node.new(value: number)
      else
        @position += 1
      end
      @max_depth = [depth, @max_depth].max
    end

    stack.pop()
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

  def inorder(&block)
    @left.inorder(&block) if (@left)
    block.call(@value || 0)
    @right.inorder(&block) if (@right)
  end

  def initialize(left: nil, right: nil, value: nil)
    @left = left
    @right = right
    @value = value
  end
end
