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
    if (b & 1) {
      r = (r + a) % MOD
      b--
    } else {
      a = (a << 1) % MOD
      b >>= 1
    }
  }
  return r
}

function fastModExp(a, b) {
  let r = 1;
  while (b) {
    if (b & 1) {
      r = fastModMulti(r, a)
      b--
    } else {
      a = fastModMulti(a, a)
      b >>= 1
    }
  }
  return r
}

var maximumScore = function(nums, k) {
  // find ranges
  const ileft = new Int32Array(nums.length)
  const iright = new Int32Array(nums.length)
  let stack

  // right
  stack = []
  for (let i = nums.length - 1; i >= 0; i--) {
    // for right extension only agree on candidate
    // that is stricly greater in priority
    while (sieve[nums[i]] >= sieve[nums[stack[stack.length - 1]]]) stack.pop()
    iright[i] = stack[stack.length - 1] ?? nums.length
    // add current index as candidate for next iterations
    stack.push(i)
  }

  // left
  stack = []
  for (let i = 0; i < nums.length; i++) {
    // for left extension agree on candidate
    // that is greater or equal in priority
    while (sieve[nums[i]] > sieve[nums[stack[stack.length - 1]]]) stack.pop()
    ileft[i] = stack[stack.length - 1] ?? -1
    // add current index as candidate for next iterations
    stack.push(i)
  }

  // sort decreasing
  const sorted = new Uint32Array(nums.length)
  for (let i = 0; i < sorted.length; i++) { sorted[i] = i }
  sorted.sort((a, b) => nums[b] - nums[a])

  // next element to grab
  let next = 0

  // solve
  let result = 1
  while (k) {
    const i = sorted[next++]
    let times = (i - ileft[i]) * (iright[i] - i)
    times = Math.min(times, k)
    result = fastModMulti(result, fastModExp(nums[i], times))
    k -= times
  }
  return Number(result)
}

export default maximumScore
