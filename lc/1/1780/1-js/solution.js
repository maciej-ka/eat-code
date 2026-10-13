/**
 * @param {number} n
 * @return {boolean}
 */
var checkPowersOfThree = function(n) {
  let power = ~~(Math.log(n)/Math.log(3))
  while (power >= 0) {
    const step = 3 ** power
    if (n >= step) {
      n -= step
    }
    if (n >= step) {
      return false
    }
    power--
  }
  return true
};

export default checkPowersOfThree
