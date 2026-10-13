/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var minOperations = function(nums, k) {
  const arr = new Uint8Array(101)
  let result = 0
  for (const num of nums) {
    if (num > k && !arr[num]++) { result++ }
    if (num < k) { return -1 }
  }
  return result
};

export default minOperations
