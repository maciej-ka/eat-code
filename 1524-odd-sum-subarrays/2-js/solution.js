/**
 * @param {number[]} arr
 * @return {number}
 */
class Node {
  constructor(prev, value, index) {
    this.prev = prev
    this.value = value
    this.index = index
  }
}

var numOfSubarrays = function(arr) {
  let node = new Node(null, 0, 0)
  let index = 2
  for (let i = 0, k = 0; i <= arr.length; i++) {
    if (i === arr.length || arr[i] % 2) {
      const value = i - k + (node?.prev?.value ?? 0)
      node = new Node(node, value, ~~(index >> 1))
      index = index + 1
      k = i + 1
    }
  }

  let result = 0
  while (node.prev) {
    const tail = node.value - (node.prev?.prev?.value ?? 0) + 1
    const pre = node.prev.value + node.prev.index
    result = (result + tail * pre) % 1000000007
    node = node.prev
  }
  return result
};

export default numOfSubarrays;
