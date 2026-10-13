/**
 * @param {string} s
 * @return {number[]}
 */

// as many parts as possible

var partitionLabels = function(s) {
  // for each letter find range [start, end]
  const ranges = {}
  for (let i = 0; i < s.length; i++) {
    ranges[s[i]]
      ? ranges[s[i]][1] = i
      : ranges[s[i]] = [i, i]
  }

  // sort ranges by start
  const sorted = Object.values(ranges).sort((a, b) => a[0] - b[0])

  // walk them one by one
  let result = []
  // start of split
  let start = -1
  for (let i = 0; i < sorted.length - 1; i++) {
    // there is overlap
    if (sorted[i][1] > sorted[i + 1][0]) {
      // possibly extend next range end
      sorted[i + 1][1] = Math.max(sorted[i][1], sorted[i + 1][1])
      continue
    }
    // no overlap, record partition
    result.push(sorted[i][1] - start)
    start = sorted[i][1]
  }
  // one more partition, when reaching end
  result.push(sorted[sorted.length - 1][1] - start)

  return result
};

export default partitionLabels
