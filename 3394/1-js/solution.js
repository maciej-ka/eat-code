/**
 * @param {number} n
 * @param {number[][]} rectangles [sx, sy, ex, ey][]
 * @return {boolean}
 */

var checkValidCuts = function (n, rectangles) {
  function check(sIdx, eIdx) {
    let slices = 0;
    rectangles.sort((a, b) => a[sIdx] - b[sIdx]);
    for (let i = 0; i < rectangles.length - 1; i++) {
      // next rectangle overlaps
      if (rectangles[i + 1][sIdx] < rectangles[i][eIdx]) {
        // don't drop the long first element
        rectangles[i + 1][eIdx] = Math.max(
          rectangles[i][eIdx],
          rectangles[i + 1][eIdx]
        );
        continue;
      }
      if (++slices >= 2) { return true }
    }
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
