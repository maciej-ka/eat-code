/**
 * @param {number[]} nums
 * @return {number}
 */

var subsetXORSum = function(nums) {
  // recursive "build" all subsets,
  // don't create their representation,
  // just count how they affect the score
  function buildSubset(i, xorSum) {
    // no more elements
    if (i === nums.length) { return xorSum }

    // sum results of two subset families:
    return (
      // a) with current element, impact xorSum
      buildSubset(i + 1, nums[i] ^ xorSum)
      // b) without current element
      + buildSubset(i + 1, xorSum)
    )
  }

  return buildSubset(0, 0)
};

export default subsetXORSum
