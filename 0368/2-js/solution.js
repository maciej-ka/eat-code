/**
 * @param {number[]} nums
 * @return {number[]}
 */
const END = 1000

var largestDivisibleSubset = function(nums) {
  // sort descending
  nums = nums.sort((a, b) => b - a)

  // subsets lengths and next
  const lengths = new Int8Array(nums.length)
  const next = new Uint16Array(nums.length)

  // best so far
  let ibest = 0;

  // fill subset lengths
  for (let i = 0; i < nums.length; i++) {
    next[i] = END;
    // look for continuation
    for (let k = 0; k < i; k++) {
      if (nums[k] % nums[i] === 0 && lengths[k] > lengths[i]) {
        lengths[i] = lengths[k]
        next[i] = k
      }
    }
    // include current and check is best
    if (++lengths[i] > lengths[ibest]) { ibest = i }
  }

  // reconstruct the result
  const result = []
  let i = ibest
  while (i !== END) {
    result.push(nums[i])
    i = next[i]
  }

  return result;
};

export default largestDivisibleSubset
