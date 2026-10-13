/**
 * @param {number[]} candies
 * @param {number} k
 * @return {number}
 */
var maximumCandies = function(candies, k) {
  // calculate total to set search bound
  // and check for early impossible
  let total = 0
  for (let i = 0; i < candies.length; i++) { total += candies[i] }
  if (k > total) { return 0 }

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
  let lo = 1, hi = ~~(total / k), result = 0;
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
