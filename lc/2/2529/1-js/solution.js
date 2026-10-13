/**
 * @param {number[]} nums
 * @return {number}
 */
var maximumCount = function(nums) {
  let pos = 0, neg = 0, hi = nums.length - 1;

  // find pos
  while (pos !== hi) {
    const m = (pos + hi) >> 1
    nums[m] > 0 ? hi = m : pos = m + 1
  }
  if (nums[pos] <= 0) { pos++ }
  pos = nums.length - pos

  // find neg
  hi = nums.length
  while (neg !== hi) {
    const m = (neg + hi) >> 1
    nums[m] >= 0 ? hi = m : neg = m + 1
  }

  return Math.max(pos, neg)
};

export default maximumCount
