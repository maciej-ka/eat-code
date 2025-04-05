/**
 * @param {number[]} nums
 * @return {number}
 */

var subsetXORSum = function(nums) {
  let result = 0

  // avoid duplicates
  let visited = {}

  // build n subarrays of ids
  // by omiting each id from current array
  function visit(ids, sum) {
    // check is it duplicate
    if (visited[ids]) { return }

    // adjust result
    result += sum
    // record for a later duplicate detection
    visited[ids] = true

    // visit n subarrays
    if (ids.length > 1) {
      for (let i = 0; i < ids.length; i++) {
        visit(
          [...ids.slice(0, i), ...ids.slice(i + 1)],
          sum ^ nums[ids[i]]
        )
      }
    }
  }

  // visit all
  visit(
    Array.from(nums.keys()),
    nums.reduce((a, b) => a ^ b, 0)
  )

  return result
};

export default subsetXORSum
