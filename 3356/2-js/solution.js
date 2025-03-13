/**
 * @param {number[]} nums
 * @param {number[][]} queries
 * @return {number}
 */
var minZeroArray = function(nums, queries) {
  // to apply operations faster calculate diff array
  // last element is ignored when checking zero
  const diff = new Int32Array(nums.length + 1)
  diff[0] = nums[0]
  diff[queries.length] = 0
  for (let i = 1; i < nums.length; i++) {
    diff[i] = nums[i] - nums[i - 1]
  }

  // binary search result
  let lo = 0, hi = queries.length, result = -1
  while (lo <= hi) {
    const m = (lo + hi) >> 1
    if (isPassing(new Int32Array(diff), queries, m)) {
      result = m
      hi = m - 1
    } else {
      lo = m + 1
    }
  }
  return result
};

function isPassing(diff, queries, queriesCount) {
  // apply queries
  for (let i = 0; i < queriesCount; i++) {
    diff[queries[i][0]] -= queries[i][2]
    diff[queries[i][1] + 1] += queries[i][2]
  }

  // check is zero
  let i = 0
  let sum = 0
  while (i < diff.length - 1) {
    sum += diff[i++]
    if (sum > 0) { return false }
  }
  return true
}

export default minZeroArray

