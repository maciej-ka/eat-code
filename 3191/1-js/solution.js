/**
 * @param {number[]} nums
 * @return {number}
 */
var minOperations = function(nums) {
  let k = 0
  // walk the array
  let i
  for (i = 0; i < nums.length - 2; i++) {
    // switch when current element needs
    if (!nums[i]) {
      nums[i+1] = nums[i+1] ? 0 : 1
      nums[i+2] = nums[i+2] ? 0 : 1
      // count switches
      k++
    }
  }

  // return -1 if the last two are 0
  return nums[i] && nums[i+1] ? k : -1
};

export default minOperations

// test case 1
// [0,1,1,1,0,0]
// i:0, k:1, [1,0,0,1,0,0]
// i:1, k:2, [1,1,1,0,0,0]
// i:2, k:2, [1,1,1,0,0,0]
// i:3, k:3, [1,1,1,1,1,1]

// test case 2
// [0,1,1,1]
// i:0, k:1, [1,1,1,0]
// i:1, k:0, [1,1,1,0]
// two last non 1
// return -1
