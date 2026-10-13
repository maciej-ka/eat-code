# perhaps faster when avoiding max
# and creating temporary arrays on fly
# also reusing prev variable

# result:
# great on memory, not so good on speed
# turns out, array max is quite fast

def rob(nums)
  return 0 if !nums
  return nums.max if nums.length <= 3

  temp = 0

  prev = 0
  solution = nums[0]
  i = 1
  while i < nums.length - 1
    temp = nums[i] + prev
    prev = solution
    solution = temp if temp > solution
    i += 1
  end

  prev = 0
  skipped = nums[1]
  i = 2
  while i < nums.length - 2
    temp = nums[i] + prev
    prev = skipped
    skipped = temp if temp > skipped
    i += 1
  end

  temp = nums[-1] + skipped
  solution > temp ? solution : temp
end

