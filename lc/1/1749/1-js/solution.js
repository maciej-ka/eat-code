/**
 * @param {number[]} nums
 * @return {number}
 */
var maxAbsoluteSum = function(nums) {
  let sum = 0;
  let max = 0, imax = 0;
  let min = 0, imin = 0;

  for (let i = 0; i < nums.length; i++) {
    sum += nums[i]
    if (sum > max) {
      max = sum
      imax = i
    }
    if (sum < min) {
      min = sum
      imin = i
    }
  }

  sum = 0;
  for (let i = imax; i >= 0; i--) {
    sum += nums[i]
    if (sum > max) max = sum
  }

  sum = 0;
  for (let i = imin; i >= 0; i--) {
    sum += nums[i]
    if (sum < min) min = sum
  }

  return Math.max(Math.abs(min), Math.abs(max))
}

export default maxAbsoluteSum
