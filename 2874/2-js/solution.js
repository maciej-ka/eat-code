/**
 * @param {number[]} nums
 * @return {number}
 */

// (nums[i] - nums[j]) * nums[k]

// just as we often track running max
// track maximum of difference (nums[i] - nums[j])

// ensure order of keys i < j < k is satisfied
// by tracking three values dependent from top to bottom
// and applying updates in specific order

var maximumTripletValue = function(nums) {
  let max = 0, maxDiff = 0, result = 0;
  for (let num of nums) {
    // first, multiply by nums[k]
    // for order it's important that num
    // cannot be a part of maxDiff at this moment
    result = Math.max(result, maxDiff * num);

    // second, apply subtraction - nums[j]
    // num cannot be a part of max at this moment
    maxDiff = Math.max(maxDiff, max - num);

    // third, update max
    max = Math.max(max, num)
  }

  return result
};

export default maximumTripletValue
