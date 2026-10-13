/**
 * @param {number[]} nums
 * @return {number}
 */
var longestNiceSubarray = function(nums) {
  // sliding variable length window
  // left index 3 means next leave will be 3
  // right index 3 means next enter will be 3

  // initialize window on first element
  let left = 0, right = 1

  // remember best result
  let best = 1

  // maintain bitmask sum of window
  let mask = nums[0]

  // while there is anything left to enter
  while (right < nums.length) {

    // prioritize enter if bitwise AND is zero
    if ((nums[right] & mask) === 0) {
      // on enter use bitwise OR to add to mask
      mask |= nums[right++]
      best = Math.max(right - left, best)
      continue
    }

    // on leave use XOR to remove from mask
    mask ^= nums[left++]
  }

  return best
};

// test case 1
// [1,3,8,48,10]
// ['1', '11', '1000', '110000', '1010']
//
// window [1], left: 0, right: 1
// sum 1 
//
// cannot expand with 11
// because 1 & 3 = 1 and is nonzero
//
// leave 1
// window [], left: 1, right: 1
// sum 1 ^ 1 = 0
//
// enter 3 because 3 & 0 = 0
// window [3], left: 1, right: 2
// sum: 3 | 0 = 3
//
// becuase 8 | 3 = 0, then enter
// window [3, 8], left: 1, right: 3
// sum: 3 | 8 = 11

export default longestNiceSubarray
