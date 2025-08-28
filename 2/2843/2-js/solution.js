/**
 * @param {number} low
 * @param {number} high
 * @return {number}
 */

// range is small enough that it's possible to
// prepopulate walking sum of symmetric count
const N = 1e4 + 1
const arr = new Uint16Array(N)

// notation for 10
const num = [0, 1]
let count = 0

// increase num in array format by one
function inc() {
  num[0]++
  let k = 0;
  while (num[k] === 10) {
    num[k] = 0
    // increase next or change from undefined to 1
    num[k + 1] = (num[k + 1] || 0) + 1
    k++
  }
}

// check that num is sym
function checkSym() {
  if (num.length % 2) { return }
  let sumA = 0, sumB = 0, k = 0
  for (; k < num.length / 2; k++) { sumA += num[k] }
  for (; k < num.length; k++) { sumB += num[k] }
  if (sumA === sumB) { count++ }
}

// populate arr
let i = 10
while (i < N) {
  arr[i] = count
  inc()
  i++
  checkSym()
}

var countSymmetricIntegers = function(low, high) {
  return arr[high] - arr[low - 1]
};

export default countSymmetricIntegers

// In 321x there may be only one x
// that makes the number symmetric.
