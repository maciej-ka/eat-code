/**
 * @param {number[]} nums
 * @param {number} pivot
 * @return {number[]}
 */
var pivotArray = function(nums, pivot) {
  // less then pivon, more then pivot, pivot count
  let less = [], more = [], count = 0

  for (let i = 0; i < nums.length; i++) {
    if (nums[i] < pivot) {
      less.push(nums[i])
    } else if (nums[i] > pivot) {
      more.push(nums[i]) 
    } else {
      count++
    }
  }

  return [...less, ...new Array(count).fill(pivot), ...more]
};

export default pivotArray
