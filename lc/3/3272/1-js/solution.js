/**
 * @param {number} n
 * @param {number} k
 * @return {number}
 */

// when transforming from k-palindromic to good
// number of digits will not change

// normalized palindrome: rearange into largest good number
// abcba, 42724 => 74422

// having two different normalized palindromes n1, n2
// sets of good numbers that they can generate have no overlap
// because generated numbers will consist of different digits
// (however, one normalized palindrome can generate serveral palindromes)

// how to count (distinct) good integers
//
// aabbc
// 5! permutations
//   / 2 swapping pair of a results in same number
//   / 2 swapping pair of b results in same number
// 5! / 2! * 2! * 1!
//
// aaaab
// how many pairs of a can be swapped with same result
// 4 * 3 = 12 pairs, means 12 * 2 results are their duplicate
// 5! / 4! * 1!
//
// aaabb
// 5! / 3! * 2!
//
// aabb0
// zero cannot be on first position
// (4 * 4!) / (2! * 2! * 1!)
//
// aaa00
// (3 * 4!) / (3! * 2!)
// aaa00
// aa0a0
// aa00a
// a0aa0
// a0a0a
// a00aa

// solution approach:
// 1 walk over space of palindromes
// 2 for each, generate number
// 3 if it is divisable, make a normal form
// 4 if not visited: count good numbers

const fact = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800]

var countGoodIntegers = function(n, k) {
  // palindrome key: first half of palindrome
  const nhalf = 1 + n >> 1
  let pkey = 10 ** (nhalf - 1)

  // end of range to check
  const pkeyEnd = pkey * 10

  // make full palindrome from key
  function pmake(key) {
    const shift = n - nhalf
    let p = key * 10 ** shift
    // if odd drop last digit from key
    if (nhalf > shift) { key = ~~(key / 10) }
    // mirror key
    for (let i = 0; i < shift; i++) { 
      p += (key % 10) * 10 ** (shift - i - 1)
      key = ~~(key / 10)
    }
    return p
  }

  const normArr = new Uint8Array(n)
  // turn palindrome into normalized
  // aka: greatest good number
  function normalize(p) {
    for (let i = 0; i < n; i++) {
      normArr[i] = p % 10
      p = ~~(p / 10)
    }
    normArr.sort((a, b) => b - a)
    let pnorm = 0
    for (const d of normArr) { pnorm = pnorm * 10 + d }
    return pnorm
  }

  // looking at normArr
  // count number of good numbers
  function count() {
    let nonZeroes = n
    let last = normArr[0]
    let len = 0
    let duplicates = 1
    for (const d of normArr) {
      if (!d) { nonZeroes-- }
      if (d !== last) {
        duplicates *= fact[len]
        len = 0
        last = d
      }
      len++
    }
    duplicates *= fact[len]
    return (nonZeroes * fact[n - 1]) / duplicates
  }

  // visited normalized palindromes
  const visited = new Set()

  // walk over palindome keys
  let result = 0
  for (; pkey < pkeyEnd; pkey++) {
    const p = pmake(pkey)
    if (p % k !== 0) { continue }
    const pnorm = normalize(p)
    if (visited.has(pnorm)) { continue }
    result += count()
    visited.add(pnorm)
  }

  return result;
};

export default countGoodIntegers
