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

  // max length
  let max = 0;
  // result
  let res = []

  // memo of tail lengths
  // const tailMemo = new Int8Array(nums.length).fill(-1)

  // explore subsets of nums[i]
  // which is a part of len elements
  // return: total length of longest find
  function explore(i, len) {
    // new max: reset path
    if (len > max) {
      max = len;
      res = [];
    }

    // if (tailMemo[i] < 0) {

      // best continuation length
      let tailLen = 0
      // explore continuations
      for (let k = i + 1; k < nums.length; k++) {
        if(nums[k] % nums[i] === 0) {
          tailLen = Math.max(tailLen, explore(k, len + 1) - len)
        }
      }

    //   // store
      // tailMemo[i] = tailLen
    // }

    // const total = len + tailMemo[i]


    const total = len + tailLen
    // this call is on the max path
    // make sure that we write only once per level
    if (total === max && max - res.length === len) {
      res.push(nums[i])
    }

    // return total length
    return total
  }

  // visit each unexplored
  for (let i = 0; i < nums.length; i++) {
    // if (tailMemo[i] < 0) {
      explore(i, 1)
    // }
  }

  return res;
};

export default largestDivisibleSubset
