/**
 * @param {number[]} colors
 * @param {number} k
 * @return {number}
 */
var numberOfAlternatingGroups = function(colors, k) {
  // try to calculate result in one pass
  let result = 0
  let i = 1

  // extension is part that will be added to last group
  let extension = 0
  if (colors[0] !== colors[colors.length - 1]) {
    for (; i < colors.length; i++) {
      if (colors[i] === colors[i - 1]) {
        extension = i
        break
      }
    }
    // ideal scenario: no duplicate and colors cycle
    if (i === colors.length) {
      return colors.length
    }
  }

  // start of alternating group
  let startIdx = extension ? i : 0
  // detect alternating groups and adjust result on the go
  for (; i < colors.length; i++) {
    if (colors[i] === colors[i - 1]) {
      const len = i - startIdx
      result += Math.max(0, len - k + 1)
      startIdx = i
    }
  }

  // score last group
  const len = i - startIdx + extension
  result += Math.max(0, len - k + 1)

  return result
};

export default numberOfAlternatingGroups
