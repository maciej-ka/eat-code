/**
 * @param {number[]} nums
 * @return {number}
 */
var minimumOperations = function(nums) {
  let lastSeen = 0n

  for (let i = nums.length - 1; i >= 0; i--) {
    const mask = 1n << BigInt(nums[i])
    if (lastSeen & mask) {
      console.log(lastSeen.toString(2));
      return Math.ceil((i + 1) / 3)
    }
    lastSeen |= mask
  }

  console.log(lastSeen.toString(2));
  return 0
};

export default minimumOperations
