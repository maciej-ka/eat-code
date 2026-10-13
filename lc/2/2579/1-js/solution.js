/**
 * @param {number} n
 * @return {number}
 */
var coloredCells = function (n) {
  return n === 1
    ? 1
    : n * n + (n - 1) * (n - 1);
};

export default coloredCells;
