/**
 * @param {number} n
 * @return {number}
 */

// result is n length series 5 * 4 * 5 * ...

const MOD = 1e9 + 7

// fast modulo multiplication
function mul(a, b) {
  let res = 0
  while (b) {
    if (b & 1) { res = (res + a) % MOD }
    a = (a << 1) % MOD
    b >>= 1
  }
  return res
}

// fast modulo exponentiaton
function exp(num, e) {
  let res = 1
  while (e) {
    if (e & 1n) { res = mul(res, num) }
    num = mul(num, num)
    e >>= 1n
  }
  return res
}

var countGoodNumbers = function(n) {
  n = BigInt(n)
  const series = exp(20, n >> 1n)
  return n % 2n ? mul(series, 5) : series
};

export default countGoodNumbers
