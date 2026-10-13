/**
 * @param {number[]} nums
 * @param {number} pivot
 * @return {number[]}
 */

var pivotArray = function(nums, pivot) {
  // smaller than pivot linked list
  let smallerHead, smaller
  smallerHead = smaller = {}
  // greater than pivot linked list
  let greaterHead, greater
  greaterHead = greater = {}
  // pivot elements count
  let pivotCount = 0

  for (let i = 0; i < nums.length; i++) {
    if (nums[i] < pivot) {
      smaller.value = nums[i]
      smaller.next = {}
      smaller = smaller.next
    } else if (nums[i] > pivot) {
      greater.value = nums[i]
      greater.next = {}
      greater = greater.next
    } else {
      pivotCount++
    }
  }

  // walk lists to recreate result
  const result = []
  smaller = smallerHead
  greater = greaterHead

  while (smaller.next) {
    result.push(smaller.value)
    smaller = smaller.next
  }
  while (pivotCount) {
    result.push(pivot)
    pivotCount--
  }
  while (greater.next) {
    result.push(greater.value)
    greater = greater.next
  }

  return result
};

export default pivotArray
