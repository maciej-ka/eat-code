/**
 * @param {number[]} nums
 * @return {boolean}
 */
var divideArray = function(nums) {
  const counts = new Map()
  for (let i = 0; i < nums.length; i++) {
    counts.set(nums[i], (counts.get(nums[i]) ?? 0) + 1)
  }

  for (const value of counts.values()) {
    if (value % 2) { return false }
  }
  return true
};

export default divideArray
