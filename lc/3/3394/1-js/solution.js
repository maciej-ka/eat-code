/**
 * @param {number} n
 * @param {number[][]} rectangles [sx, sy, ex, ey][]
 * @return {boolean}
 */

var checkValidCuts = function (n, rectangles) {
  function check(start, end) {
    let slices = 0;
    rectangles.sort((a, b) => a[start] - b[start]);
    for (let i = 0; i < rectangles.length - 1; i++) {
      // next rectangle overlaps
      if (rectangles[i + 1][start] < rectangles[i][end]) {
        // don't drop the long first element
        rectangles[i + 1][end] = Math.max(
          rectangles[i][end],
          rectangles[i + 1][end]
        );
        continue;
      }
      // next rectangle is splitted
      if (++slices >= 2) { return true }
    }
    // we ran out of rectangles
    return false;
  }

  return (
    // check horizontal
    check(0, 2) ||
    // check vertical
    check(1, 3)
  );
};

export default checkValidCuts;
