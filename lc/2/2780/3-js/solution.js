/**
 * @param {number[]} nums
 * @return {number}
 */

// dominant element can be identified
// on first scan of nums, regardless of split

var minimumIndex = function(nums) {
  // num => count
  const counts = new Map()
  let max = 0
  let dominant

  // find dominant
  for (let num of nums) {
    let count = counts.get(num) || 0
    counts.set(num, ++count)
    if (count > max) {
      max = count
      dominant = num
    }
  }

  // double the count for faster checking 
  // is over the half of length
  max = max << 1

  // dominant doesn't exist
  if (max <= nums.length) { return -1 }

  // dominant exists
  let countA = 0
  let countB = max
  let lenA = 0
  let lenB = nums.length
  while (lenA < nums.length) {
    // adjust counts
    if (nums[lenA++] === dominant) {
      countA += 2
      countB -= 2
    }
    lenB--
    // split is valid
    if (countA > lenA && countB > lenB) { return lenA - 1 }
  }

  // no valid split found
  return -1
};

export default minimumIndex
