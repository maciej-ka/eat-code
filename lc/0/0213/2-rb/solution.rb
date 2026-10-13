# not using arrays
# in hope of faster result

# result: it's quite faster
# and good on memory

def rob(nums)
  return 0 if !nums
  return nums.max if nums.length <= 3

  temp = 0
  solution_prev = 0
  solution = nums[0]

  i = 1
  while i < nums.length - 1
    temp = solution
    solution = [solution, nums[i] + solution_prev].max
    solution_prev = temp
    i += 1
  end

  skipped_prev = 0
  skipped = nums[1]

  i = 2
  while i < nums.length - 2
    temp = skipped
    skipped = [skipped, nums[i] + skipped_prev].max
    skipped_prev = temp
    i += 1
  end

  [solution, nums[-1] + skipped].max
end
