/**
 * @param {number[]} arr
 * @return {number}
 */
var numOfSubarrays = function(arr) {
  const agg = [0, 0]
  for (let i = 0, k = 0; i <= arr.length; i++) {
    if (i === arr.length || arr[i] % 2) {
      agg.push(i - k + agg[agg.length - 2])
      k = i + 1
    }
  }

  let result = 0
  for (let i = agg.length - 1; i > 2; i--) {
    const tail = agg[i] - agg[i - 2] + 1
    const pre = agg[i - 1] + 1 + ~~((i - 3) >> 1)
    result = (result + tail * pre) % 1000000007
  }
  return result
};

export default numOfSubarrays;
