/**
 * @param {number[]} arr
 * @return {number}
 */
var lenLongestFibSubseq = function(arr) {
  if (arr.length <= 2) return 0

  const numbers = new Map()
  for (let i = 0; i < arr.length; i++) {
    numbers.set(arr[i], true)
  }

  let best = 2
  for (let i = 0; i < arr.length - 1; i++) {
    for (let k = i + 1; k < arr.length; k++) {
      const len = fibLength(arr[i], arr[k], numbers)
      best = Math.max(best, len)
    }
  }

  return best > 2 ? best : 0
};

const fibLength = (n1, n2, numbers) => {
  const sum = n1 + n2
  if (!numbers.get(sum)) return 2
  return fibLength(n2, sum, numbers) + 1
}

export default lenLongestFibSubseq
