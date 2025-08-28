/**
 * @param {number[]} nums
 * @return {number}
 */

var subsetXORSum = function(nums) {
  let res = 0;

  // there are 2 ** n subsets
  // binary of their order number
  // will tell which elements are present
  const n = 2 ** nums.length;
  for (let i = 0; i < n; i++) {
    let sum = 0
    // scan bits
    for (let mask = 1, j = 0; mask < n; mask <<= 1, j++) {
      // detect is bit 1
      if (mask & i) { sum ^= nums[j] }
    }
    // update res
    res += sum
  }

  return res
};

export default subsetXORSum
