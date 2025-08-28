/**
 * @param {number} n
 * @return {number}
 */

// result is n length series 5 * 4 * 5 * ...

const MOD = 1000000007n

// fast modulo multiplication
function mul(a, b) {
  let inc = a, res = 0n
  while (b) {
    if (b % 2n) { res = (res + inc) % MOD }
    inc = (inc << 1n) % MOD
    b >>= 1n
  }
  return res
}

// fast modulo exponentiaton
function exp(num, e) {
  let inc = num, res = 1n
  while (e) {
    if (e % 2n) { res = (res * inc) % MOD }
    inc = mul(inc, inc)
    e >>= 1n
  }
  return res
}

var countGoodNumbers = function(n) {
  n = BigInt(n)
  return Number(mul(
    exp(5n, n + 1n >> 1n),
    exp(4n, n >> 1n),
  ))
};

export default countGoodNumbers
