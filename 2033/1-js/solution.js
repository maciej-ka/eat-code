/**
 * @param {number[][]} grid
 * @param {number} x
 * @return {number}
 */

// wrong assumption:
// best goal number is average of elements

// correct:
// best goal number is median of elements

// to check is operation possible
// check that n % x is same for all elements

var minOperations = function(grid, x) {
  // remember the modulo of first element
  const mod = grid[0][0] % x

  // convert to flat array of x multiplication
  const arr = new Uint16Array(grid.length * grid[0].length)
  let i = 0
  for (let row of grid) {
    for (let num of row) {
      if (num % x !== mod) { return -1 }
      arr[i++] = (num - mod) / x
    }
  }

  // find median
  arr.sort()
  const goal = arr[arr.length >> 1]

  let result = 0
  for (let num of arr) {
    result += Math.abs(goal - num)
  }

  return result
};

export default minOperations
