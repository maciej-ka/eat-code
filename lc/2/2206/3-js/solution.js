/**
 * @param {number[]} nums
 * @return {boolean}
 */
var divideArray = function(nums) {
  const set = new Set()
  for (const num of nums) {
    set.has(num)
      ? set.delete(num)
      : set.add(num)
  }
  return !set.size
};

export default divideArray
