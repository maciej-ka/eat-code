/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
const topKFrequent = function (nums, k) {
  // count in hash
  const counts = {};
  nums.forEach((value) => {
    counts[value] = counts[value] ? counts[value] + 1 : 1;
  });

  // sort result
  const entries = Object.entries(counts);
  entries.sort((a, b) => b[1] - a[1]);
  return entries.splice(0, k).map(([value]) => +value);
};

module.exports = { topKFrequent };
