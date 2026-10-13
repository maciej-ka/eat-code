/**
 * @param {number[]} nums
 * @return {boolean}
 */
var divideArray = function(nums) {
  const counts = {}
  for (let i = 0; i < nums.length; i++) {
    counts[nums[i]] = (counts[nums[i]] || 0) + 1
  }

  const keys = Object.keys(counts)
  for (let i = 0; i < keys.length; i++) {
    if (counts[keys[i]] % 2) { return false }
  }
  return true
};

export default divideArray
