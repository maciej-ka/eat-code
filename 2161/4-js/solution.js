/**
 * @param {number[]} nums
 * @param {number} pivot
 * @return {number[]}
 */

var pivotArray = function(nums, pivot) {
  return nums.filter(x => x < pivot).concat(
    nums.filter(x => x === pivot),
    nums.filter(x => x > pivot)
  )
};

export default pivotArray
