/**
 * @param {number[]} ranks
 * @param {number} cars
 * @return {number}
 */
var repairCars = function(ranks, cars) {
  // binary search for solution

  // Ranks are from 0 to 100,
  // but their count is 1e5,
  // so they are quite repeated.
  // For faster check, aggregate them.
  const counts = new Uint32Array(101)
  for (let i = 0; i < ranks.length; i++) { counts[ranks[i]]++ }

  // ranks with non zero count
  const nonzero = []
  for (let i = 1; i <= 100; i++) {
    if (counts[i]) { nonzero.push(i) }
  }

  // check a candidate solution
  function check(time) {
    let result = 0
    for (let i = 0; i < nonzero.length; i++) {
      const rank = nonzero[i]
      result += ~~((time/rank) ** 0.5) * counts[rank]
      if (result >= cars) { return true }
    }
    return false
  }

  // search for solution
  let lo = 1, result = 0
  // for hi assume the lowest rank will perform all the cars
  let hi = nonzero[0] * Math.ceil(cars / counts[nonzero[0]]) ** 2

  while (lo <= hi) {
    const m = Math.floor((lo + hi) / 2)
    if (check(m)) {
      result = m
      hi = m - 1
    } else {
      lo = m + 1
    }
  }
  return result
};

export default repairCars
