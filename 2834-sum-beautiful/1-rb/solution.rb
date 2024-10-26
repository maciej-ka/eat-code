# @param {Integer} n
# @param {Integer} target
# @return {Integer}
def minimum_possible_sum(n, target)
  return 1 if n == 1

  half_target = (target / 2).floor
  if (n <= half_target)
    return (n * (n + 1)) / 2
  end

  rest = n - half_target

  low = (half_target * (half_target + 1)) / 2
  podest = rest * target
  high = (rest * (rest - 1)) / 2

  return low + podest + high
end

