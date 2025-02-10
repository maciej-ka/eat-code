/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
const topKFrequent = function (nums, k) {
  // count in hash
  const counts = new Map();
  nums.forEach((value) => {
    counts.set(value, (counts.get(value) || 0) + 1)
  });

  // sort result
  const entries = [...counts.entries()];
  entries.sort((a, b) => b[1] - a[1]);
  return entries.splice(0, k).map(([value]) => +value);
};

module.exports = { topKFrequent };
