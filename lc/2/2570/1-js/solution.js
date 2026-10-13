/**
 * @param {number[][]} nums1
 * @param {number[][]} nums2
 * @return {number[][]}
 */
var mergeArrays = function(nums1, nums2) {
  let i1 = 0, i2 = 0, result = []
  while (i1 < nums1.length && i2 < nums2.length) {
    const n1 = nums1[i1], n2 = nums2[i2]
    if (n1[0] < n2[0]) {
      result.push(n1)
      i1++
    } else if (n1[0] > n2[0]) {
      result.push(n2)
      i2++
    } else {
      // indices are equal
      result.push([n1[0], n1[1] + n2[1]])
      i1++
      i2++
    }
  }

  // one of slices will be empty
  return [...result, ...nums1.slice(i1), ...nums2.slice(i2)]
};

export default mergeArrays
