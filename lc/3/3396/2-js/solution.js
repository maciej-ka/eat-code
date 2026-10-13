/**
 * @param {number[]} nums
 * @return {number}
 */
var minimumOperations = function(nums) {
  const lastSeen = new Set()

  for (let i = nums.length - 1; i >= 0; i--) {
    if (lastSeen.has(nums[i])) {
      return Math.ceil((i + 1) / 3)
    }
    lastSeen.add(nums[i], i)
  }

  return 0
};

export default minimumOperations
