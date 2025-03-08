/**
 * @param {string} blocks
 * @param {number} k
 * @return {number}
 */
var minimumRecolors = function(blocks, k) {
  // initial value
  let count = 0
  for (let i = 0; i < k; i++) {
    if (blocks[i] === 'W') count++
  }

  // slide window
  // adjust count to observed element out and element in
  let best = count
  for (let i = 0; i < blocks.length - k; i++) {
    if (blocks[i] === 'W') count--
    if (blocks[i + k] === 'W') count++
    if (count < best) best = count
  }

  return best
};

export default minimumRecolors
