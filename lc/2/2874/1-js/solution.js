/**
 * @param {number[]} nums
 * @return {number}
 */

// check value for every position of middle number
// track max before and after middle

var maximumTripletValue = function(nums) {
  // tabelarize post max and min
  const postMax = new Int32Array(nums.length)
  for (let i = nums.length - 2; i > 0; i--) {
    postMax[i] = Math.max(postMax[i + 1], nums[i + 1])
  }

  // initialize preMax
  let preMax = nums[0]

  // initialize result
  let result = 0

  // position of middle number
  let i = 1
  while (true) {
    const score = (preMax - nums[i]) * postMax[i];
    result = Math.max(result, score);

    // adjust preMax
    preMax = Math.max(preMax, nums[i])

    // break on last element
    // as triplet is impossible then
    if (++i == nums.length - 1) { break }
  }

  return result;
};

export default maximumTripletValue
