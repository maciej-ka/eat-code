def build(nums)
  result = [0, nums[0]]
  nums[1..].each do |x|
    result.push [result[-1], x + result[-2]].max
  end
  result
end

def rob(nums)
  return 0 if !nums
  return nums.max if nums.length <= 3
  solutions0 = build(nums[..-2])
  solutions1 = build(nums[1..-3])
  [solutions0[-1], nums[-1] + solutions1[-1]].max
end
