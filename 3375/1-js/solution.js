/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var minOperations = function(nums, k) {
  const set = new Set()
  for (const num of nums) {
    if (num > k) { set.add(num) }
    if (num < k) { return -1 }
  }
  return set.size
};

export default minOperations
