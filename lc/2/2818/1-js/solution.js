const sieve = new Uint8Array(1e5 + 1)
for (let i = 2; i < sieve.length; i++) {
  if (!sieve[i]) {
    for (let k = i; k < sieve.length; k += i) {
      sieve[k]++
    }
  }
}

const MOD = BigInt(1e9 + 7)

function fastModExp(base, exp) {
  let r = BigInt(1)
  for (let bit of Number(exp).toString(2)) {
    r = r * r % MOD
    if (bit === '1') { r = (r * base) % MOD }
  }
  return r
}

var maximumScore = function(nums, k) {
  // sort decreasing
  const sorted = new Uint32Array(nums.length)
  for (let i in sorted) { sorted[i] = i }
  sorted.sort((a, b) => nums[a] - nums[b])

  // size of array
  let last = nums.length - 1

  // solve
  let result = BigInt(1)
  while (k) {
    // largest left num
    const i = sorted[last--]
    // extend right
    let r = 1
    while (
      i + r < nums.length
      && sieve[nums[i]] >= sieve[nums[i + r]]
    ) { r++ }
    // extend left
    let l = 1
    while (
      i - l >= 0
      && sieve[nums[i]] > sieve[nums[i - l]]
    ) { l++ }
    // result
    const times = Math.min(l * r, k)
    result = result * fastModExp(BigInt(nums[i]), BigInt(times)) % MOD
    k -= times
  }
  return Number(result)
}

export default maximumScore
