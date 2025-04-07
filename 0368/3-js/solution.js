/**
 * @param {number[]} nums
 * @return {number[]}
 */
const END = 1000

var largestDivisibleSubset = function(nums) {
  // sort descending
  nums = new Uint32Array(nums.sort((a, b) => b - a))

  // subsets lengths and next
  const lengths = new Int8Array(nums.length)
  const next = new Uint16Array(nums.length).fill(END)

  // best so far
  let ibest = 0;

  // it's not possible for values above half of max
  // to have a continuation
  const half = nums[0] >> 1
  let ihalf
  let lo = 0, hi = nums.length
  while (lo <= hi) {
    const m = lo + hi >> 1
    if (nums[m] >= half) {
      ihalf = m
      // array is reversed
      lo = m + 1
    }
    else { hi = m - 1 }
  }

  // fill values higher than half max
  for (let i = 0; i < ihalf; i++) {
    lengths[i] = 1;
    // don't look for continuation
  }

  // fill rest with subset lengths
  for (let i = ihalf; i < nums.length; i++) {
    // look for continuation
    for (let k = 0; k < i; k++) {
      if (nums[k] % nums[i] === 0 && lengths[k] > lengths[i]) {
        lengths[i] = lengths[k]
        next[i] = k
      }
    }
    // include current element and check is best
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
