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

  // check is count over half of elements 
  const check = (count, len) => count << 1 > len

  // dominant doesn't exist
  if (!check(max, nums.length)) { return -1 }

  // dominant exists
  let countA = 0
  let countB = max
  for (let i = 0; i < nums.length; i++) {
    // adjust counts
    if (nums[i] === dominant) {
      countA++
      countB--
    }
    // split is valid
    if (check(countA, i + 1) && check(countB, nums.length - i - 1)) {
      return i
    }
  }

  // no valid split found
  return -1
};

export default minimumIndex
