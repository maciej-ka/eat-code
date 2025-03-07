/**
 * @param {number} left
 * @param {number} right
 * @return {number[]}
 */
var closestPrimes = function(left, right) {
  if (right === 1) return [-1, -1]

  // construct linked list
  let i = 1, node = {}
  let primes = node
  while (i <= right) { node = node.next = {value: i++} }
  primes = primes.next

  // sieve
  let divisor = primes.next
  while (divisor) {
    let prev = divisor
    node = divisor.next
    while (node) {
      if (node.value % divisor.value === 0) {
        // remove current node from list
        prev.next = node.next
      } else {
        prev = node
      }
      node = node.next
    }
    divisor = divisor.next
  }

  // rewind to left
  let prev = primes
  while (prev.value < left) {
    prev = prev.next
  }
  node = prev.next

  // create result
  let num1 = -1, num2 = -1, best = Infinity
  while (node) {
    if(node.value - prev.value < best) {
      num1 = prev.value
      num2 = node.value
      best = num2 - num1
    }
    prev = node
    node = node.next
  }

  return [num1, num2]
};

export default closestPrimes
