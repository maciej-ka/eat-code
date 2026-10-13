/**
 * @param {number[]} nums
 * @return {number[]}
 */
var applyOperations = function(nums) {
  // modify in place for performance
  for (let i = 0; i < nums.length - 1; i++) {
    if (nums[i] === nums[i + 1]) {
      nums[i] = nums[i] << 1
      nums[i + 1] = 0
    }
  }

  return [...nums.filter(n => n > 0), ...nums.filter(n => n === 0)]
};

export default applyOperations
