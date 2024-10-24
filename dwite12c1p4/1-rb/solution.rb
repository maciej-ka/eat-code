require 'pp'

def solve(source)
  p = Parser.new(source)
  tree = p.run()
  sum = 0
  count = 0
  tree.inorder do |value|
    sum += value
  end
  tree.preorder do |value|
    count += 1
  end
  tree.postorder do |value|
    count += 1
  end
  count -= 1
  "#{count} #{sum}"
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
      elsif (number = parse_number())
        @stack << Node.new(value: number)
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

  def inorder(&block)
    @left.inorder(&block) if (@left)
    block.call(@value) if (@value)
    @right.inorder(&block) if (@right)
  end

  def preorder(&block)
    block.call(@value) if (@value)
    @left.inorder(&block) if (@left)
    @right.inorder(&block) if (@right)
  end

  def postorder(&block)
    @left.inorder(&block) if (@left)
    @right.inorder(&block) if (@right)
    block.call(@value) if (@value)
  end

  def initialize(left: nil, right: nil, value: nil)
    @left = left
    @right = right
    @value = value
  end
end
