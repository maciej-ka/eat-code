/**
 * @param {number[]} arr
 * @return {number}
 */
var lenLongestFibSubseq = function(arr) {
  const numbers = new Map()
  for (let i = 0; i < arr.length; i++) {
    numbers.set(arr[i], 1)
  }

  let best = 2
  for (let i = 0; i < arr.length - 1; i++) {
    for (let k = i + 1; k < arr.length - 1; k++) {
      best = Math.max(best, fibLen(arr[i], arr[k], numbers))
    }
  }

  return best > 2 ? best : 0
};

const fibLen = (n1, n2, numbers) => {
  if (!numbers.get(n1 + n2)) return 2
  return fibLen(n2, n1 + n2, numbers) + 1
}

export default lenLongestFibSubseq
