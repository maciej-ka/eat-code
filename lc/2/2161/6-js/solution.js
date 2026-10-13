/**
 * @param {number[]} nums
 * @param {number} pivot
 * @return {number[]}
 */
var pivotArray = function(nums, pivot) {
  // more then pivot, pivot count
  let greater = [], count = 0
  // calculate smaller as in place operation on  nums
  let k = 0;

  for (let i = 0; i < nums.length; i++) {
    if (nums[i] < pivot) {
      nums[k] = nums[i]
      k++
    } else if (nums[i] > pivot) {
      greater.push(nums[i]) 
    } else {
      count++
    }
  }

  const endIdx = k + count
  for (; k < endIdx; k++) {
    nums[k] = pivot
  }

  for (let i = 0; i < greater.length; i++) {
    nums[k + i] =  greater[i]
  }

  return nums
};

export default pivotArray
