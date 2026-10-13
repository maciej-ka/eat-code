/**
 * @param {number[]} candies
 * @param {number} k
 * @return {number}
 */
var maximumCandies = function(candies, k) {
  // check that every kid can get n candies
  function check(n) {
    let count = 0
    for (let i = 0; i < candies.length; i++) {
      count += ~~(candies[i] / n)
      if (count >= k) { return true }
    }
    return false
  }

  // binary search the result
  let lo = 1, hi = 10e12, result = 0;
  while (lo <= hi) {
    const m = (lo + hi) >> 1
    if (check(m)) {
      result = m
      lo = m + 1
    } else {
      hi = m - 1
    }
  }
  return result
};

export default maximumCandies
