/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var minCapability = function(nums, k) {
  // binary search result
  let lo = 1, hi = 1e9, res = 0
  while (lo <= hi) {
    const m = (lo + hi) >> 1
    if (isOk(m)) {
      res = m
      hi = m - 1
    } else {
      lo = m + 1
    }
  }

  return res

  // solution test done in a greedy way
  function isOk(cap) {
    let count = 0;
    for (let i = 0; i < nums.length; i++) {
      if (nums[i] <= cap) {
        count++
        i++
        if (count >= k) { return true }
      }
    }
  }
};

export default minCapability
