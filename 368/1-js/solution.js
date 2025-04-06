/**
 * @param {number[]} nums
 * @return {number[]}
 */

// all numbers in result are multiplications of the lowest

// x3 % x2 === 0 and x2 % x1 === 0 then x3 % x1
//
// if [x2, x3 ...] are divisible and x1 divides x2,
// then including that x1 cannot throw anything out from set

// 1 can be always included in the result
// (and omited in calculations, to simplify)

var largestDivisibleSubset = function(nums) {
  // sort in place
  nums.sort((a, b) => a - b)

  // memo of lengths and next
  const lengths = new Int8Array(nums.length)
  const next = new Uint16Array(nums.length)

  // max length and max start index
  let max = 0;
  let imax;

  // explore subsets of nums[i]
  // which is a part of len elements
  // return: max length from nums[i]
  function explore(i) {
    if (lengths[i]) { return lengths[i] }

    // max length from nums
    let len = 0

    // explore continuations
    for (let k = i + 1; k < nums.length; k++) {
      if(nums[k] % nums[i] === 0) {
        const kLen = explore(k);
        // best so far
        if (kLen > len) {
          len = kLen
          next[i] = k
        }
      }
    }

    // add current element
    // and check is it best so far
    if (++len > max) {
      max = len
      imax = i
    }

    // memoize
    lengths[i] = len
    return len
  }

  // visit each
  for (let i = 0; i < nums.length; i++) { explore(i) }

  // reconstruct the result
  const result = []
  let i = imax
  while (true) {
    result.push(nums[i])
    i = next[i]
    if (!i) { break }
  }

  return result;
};

export default largestDivisibleSubset
