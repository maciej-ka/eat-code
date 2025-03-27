/**
 * @param {number[]} nums
 * @return {number}
 */

// dominant element can be identified
// on first scan of nums, regardless of split

var minimumIndex = function(nums) {
  // find dominant using heap
  const heap = new CountHeap(nums.length)
  for (let num of nums) { heap.record(num) }

  // check is count over half of elements 
  const check = (count, len) => count << 1 > len

  // dominant doesn't exist
  if (!check(heap.counts[0], nums.length)) { return -1 }

  // dominant exists
  const dominant = heap.arr[0]
  let countA = 0
  let countB = heap.counts[0]
  for (let i = 0; i < nums.length; i++) {
    // adjust counts
    if (nums[i] === dominant) {
      countA++
      countB--
    }
    // check is it valid split
    if (check(countA, i + 1) && check(countB, nums.length - i - 1)) {
      return i
    }
  }

  // not found
  return -1
};

// 0 1 2 (3 4) (5 6)
class CountHeap {
  constructor(maxLen) {
    this.len = 0
    this.arr = new Uint32Array(maxLen);
    this.counts = new Uint32Array(maxLen);
    this.lookup = new Map()
  }

  record(num) {
    const i = this.lookup.get(num)
    // new element
    if (i === undefined) {
      this.arr[this.len] = num
      this.counts[this.len] = 1
      this.lookup.set(num, this.len)
      this.len++
      return
    }
    // seen before
    this.counts[i]++
    this.bubbleUp(i)
  }

  bubbleUp(i) {
    while (i !== 0) {
      const p = i - 1 >> 1
      if (this.counts[i] < this.counts[p]) { return }
      this.swap(i, p)
      i = p
    }
  }

  swap(i, k) {
    // swap lookup
    this.lookup.set(this.arr[i], k)
    this.lookup.set(this.arr[k], i)
    // swap heap
    let temp = this.arr[i]
    this.arr[i] = this.arr[k]
    this.arr[k] = temp
    // swap counts
    temp = this.counts[i]
    this.counts[i] = this.counts[k]
    this.counts[k] = temp
  }
}


export default minimumIndex
