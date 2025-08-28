/**
 * @param {number} left
 * @param {number} right
 * @return {number[]}
 */
var closestPrimes = function(left, right) {
  // value 1 means it's not a prime
  const primes = new Uint8Array(right + 1)
  for (let i = 2; i < right; i++) {
    if (primes[i]) continue
    for (let k = 2; i * k <= right; k++) {
      primes[i * k] = 1
    }
  }

  let num1 = -1, num2 = -1, best = Infinity
  let num
  for (let i = Math.max(2, left); i <= right; i++) {
    if (primes[i]) continue
    if (num && i - num < best) {
      num1 = num
      num2 = i
      best = i - num
      if (best === 2) break
    }
    num = i
  }
  return [num1, num2]
};

export default closestPrimes

// 1 2 3 5 7 11 13 
