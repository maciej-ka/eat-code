const sieve = new Uint8Array(1e5 + 1)
for (let i = 2; i < sieve.length; i++) {
  if (!sieve[i]) {
    for (let k = i; k < sieve.length; k += i) {
      sieve[k]++
    }
  }
}

const MOD = 1e9 + 7

function fastModMulti(a, b) {
  let r = 0;
  while (b) {
    if (b % 2 === 0) {
      a = (a << 1) % MOD
      b >>= 1
    } else {
      r = (r + a) % MOD
      b--
    }
  }
  return r
}

function fastModExp(a, b) {
  let r = 1;
  while (b) {
    if (b % 2 === 0) {
      a = fastModMulti(a, a)
      b >>= 1
    } else {
      r = fastModMulti(r, a)
      b--
    }
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
  let result = 1
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
    result = fastModMulti(result, fastModExp(nums[i], times))
    k -= times
  }
  return Number(result)
}

export default maximumScore
