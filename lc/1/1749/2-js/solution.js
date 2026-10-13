/**
 * @param {number[]} nums
 * @return {number}
 */
var maxAbsoluteSum = function(nums) {
  let sum = 0;
  let max = 0, imax = 0;
  let min = 0, imin = 0;
  let i;

  for (i = 0; i < nums.length; i++) {
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

  if (max < 0) {
    imax = 0
    max = 0
  }
  if (min > 0) {
    imin = 0
    min = 0
  }

  sum = 0;
  for (i = imax; i >= 0; i--) {
    sum += nums[i]
    if (sum > max) max = sum
  }

  sum = 0;
  for (i = imin; i >= 0; i--) {
    sum += nums[i]
    if (sum < min) min = sum
  }

  min = -1 * min
  return min > max ? min : max
}

export default maxAbsoluteSum

