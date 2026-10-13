/**
 * @param {number} low
 * @param {number} high
 * @return {number}
 */

// In 321x there may be only one x
// that makes the number symmetric.

var countSymmetricIntegers = function(low, high) {
  let result = 0

  // count of nums to check
  let t = high - low + 2

  // convert to format easy for counting
  // 1200 becomes [0, 0, 2, 1]
  const num = [...Number(low).toString()].map(x => ~~x).reverse()

  // potential for global memo here
  function isSym() {
    if (num.length % 2) { return false }
    let sumA = 0, sumB = 0, i = 0
    for (; i < num.length / 2; i++) { sumA += num[i] }
    for (; i < num.length; i++) { sumB += num[i] }
    return sumA === sumB
  }

  // increase num in array format by one
  function inc() {
    num[0]++
    let i = 0;
    while (num[i] === 10) {
      num[i] = 0
      // increase or convert from undefined to 1
      num[i + 1] = (num[i + 1] || 0) + 1
      i++
    }
  }

  // check result
  while (--t) {
    if (isSym()) { result++ }
    inc()
  }

  return result
};

export default countSymmetricIntegers
