/**
 * @param {number[]} nums
 * @return {number[]}
 */
var applyOperations = function(nums) {
  const nonzeros = []
  let zeros = 0

  for (let i = 0; i < nums.length; i++) {
    if (nums[i] === nums[i + 1]) {
      if (nums[i] === 0) {
        zeros += 2
      } else {
        nonzeros.push(nums[i] << 1)
        zeros++
      }
      i++
    } else {
      if (nums[i] === 0) {
        zeros++
      } else {
        nonzeros.push(nums[i])
      }
    }
  }

  return [...nonzeros, ...new Array(zeros).fill(0)]
};

export default applyOperations
