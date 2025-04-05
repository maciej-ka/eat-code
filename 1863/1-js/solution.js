/**
 * @param {number[]} nums
 * @return {number}
 */

var subsetXORSum = function(nums) {
  let result = 0

  // avoid duplicates
  let visited = {}

  // top-bottom
  function visit(ids, sum) {
    // already visited
    if (visited[ids]) { return }

    // adjust result
    result += sum
    visited[ids] = true

    // visit subsets
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
