/**
 * @param {number[]} colors
 * @param {number} k
 * @return {number}
 */
var numberOfAlternatingGroups = function(colors, k) {
  // try to calculate result in one pass
  const score = len => result += Math.max(0, len - k + 1)
  let result = 0
  let i = 1
  // start of last alternating group
  let start = 0

  // extension is part that will be added to last group
  let extension = 0
  if (colors[0] !== colors[colors.length - 1]) {
    for (; i < colors.length; i++) {
      if (colors[i] === colors[i - 1]) {
        start = extension = i
        break
      }
    }
    // ideal scenario: no duplicate and colors cycle
    if (i === colors.length) { return i }
  }

  // detect alternating groups and adjust result on the go
  for (; i < colors.length; i++) {
    if (colors[i] === colors[i - 1]) {
      score(i - start)
      start = i
    }
  }

  // score last group
  score(i - start + extension)
  return result
};

export default numberOfAlternatingGroups
