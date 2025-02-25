/**
 * @param {number[]} arr
 * @return {number}
 */
var numOfSubarrays = function(arr) {
  // scan for at least one odd
  if (arr.findIndex(x => x % 2) < 0) {
    return 0
  }

  let result = 1
  for (let i=(arr.length - 1); i>0; i--) {
    result = (result << 1) % 1000000007
  }
  return result
};

export default numOfSubarrays;
