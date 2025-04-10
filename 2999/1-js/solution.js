/**
 * @param {number} start
 * @param {number} finish
 * @param {number} dmax
 * @param {string} s
 * @return {number}
 */

// solution is determined by:
//
// - how many free digits are left
//   (number of multipliers)
//
// - what is the limit
//   (value of each multiplier)
//
// - what is the top number in s
//   (value of the first multiplier)

// calculate twice first
// 1. first ignore start, only care about finish
// 2. then treat start as if it would be finish
// final result = x1 - x2

function count(cap, dmax, suffix) {
  // number of not constrained prefix digits
  const free = cap.length - suffix.length

  // all are constrained: check is cap high enough
  if (free <= 0) { return Number(cap) >= Number(suffix) ? 1 : 0 }

  // first digit is above dmax: all options are possible
  if (~~cap[0] > dmax) { return (dmax + 1) ** free }

  // end: "22359", limit: 5, s: "20"
  // (2)    (2)    (0...3) +
  // (2)    (0...1)(0...5) +
  // (0...1)(0...5)(0...5)

  const top = count(cap.substr(1), dmax, suffix)

  // first digit not having top value
  // (0...1)(0...5)(0...5)
  const rest = (~~cap[0]) * (dmax + 1) ** (free - 1)
  console.log('cap:', cap, 'top', top, 'rest', rest);
  return top + rest
}

var numberOfPowerfulInt = function(start, finish, limit, s) {
  console.log('****', count(finish.toString(), limit, s), count(start.toString(), limit, s))
  return count(finish.toString(), limit, s) - count(start.toString(), limit, s)
};

export default numberOfPowerfulInt
