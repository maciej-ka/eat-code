/**
 * @param {number[]} nums
 * @return {number}
 */
var minimumOperations = function(nums) {
  let idx = -1
  const lastSeen = new Map()

  for (let i = 0; i < nums.length; i++) {
    if (lastSeen.has(nums[i])) {
      idx = Math.max(idx, lastSeen.get(nums[i]))
    }
    lastSeen.set(nums[i], i)
  }

  // convert idx to result: 0 => 1, 3 => 2
  return Math.ceil((idx + 1) / 3)
};

export default minimumOperations
