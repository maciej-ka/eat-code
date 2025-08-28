/**
 * @param {number[][]} grid
 * @return {number[]}
 */
var findMissingAndRepeatedValues = function(grid) {
  const counts = new Uint8Array(grid.length * grid.length + 1)
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid.length; j++) {
      counts[grid[i][j]]++
    }
  }

  const result = [null, null]
  for (let i = 1; i < counts.length; i++) {
    if (counts[i] == 2) result[0] = i
    if (counts[i] == 0) result[1] = i
  }

  return result
};

export default findMissingAndRepeatedValues
