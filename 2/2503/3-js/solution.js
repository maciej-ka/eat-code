/**
 * @param {number[][]} grid
 * @param {number[]} queries
 * @return {number[]}
 */

// process smallest queries first
// and for each larger, expand teritory
// (don't calculate it from zero)

// organize border as minheap
// to fast detect next possible expansion

var maxPoints = function(grid, queries) {
  // ascending queries
  const sorted = Array
    .from(new Set(queries))
    .sort((a,b) => a - b)

  // count visited
  let visited = 0

  // fields that can be visited
  // but so far had too large value
  const border = new MinHeap()

  // expand border
  function expand(loc) {
    if (!grid[loc[0]][loc[1]]) { return }
    border.add(loc, grid[loc[0]][loc[1]])
    // mark field as in border
    grid[loc[0]][loc[1]] = false
  }

  // initalize border
  expand([0,0])

  // place to store query results
  const results = new Map()

  // grid size
  const xLast = grid.length - 1
  const yLast = grid[0].length - 1

  // process queries one by one
  // try to enlarge teritory
  for (let q of sorted) {
    while (q > border.top()?.[1]) {
      let f = border.pop()[0]
      visited++
      if (f[0] > 0) { expand([f[0] - 1, f[1]]) }
      if (f[1] > 0) { expand([f[0], f[1] - 1]) }
      if (f[0] < xLast) { expand([f[0] + 1, f[1]]) }
      if (f[1] < yLast) { expand([f[0], f[1] + 1]) }
    }
    results.set(q, visited)
  }

  // format result in place
  for (let i in queries) {
    queries[i] = results.get(queries[i])
  }
  return queries
};



class MinHeap {
  constructor() {
    this.arr = []
  }

  top() {
    return this.arr[0]
  }

  swap(i, k) {
    const temp = this.arr[i]
    this.arr[i] = this.arr[k]
    this.arr[k] = temp
  }

  bubbleUp(i) {
    while (i) {
      const p = i - 1 >> 1
      if (this.arr[i][1] >= this.arr[p][1]) { return }
      this.swap(i, p)
      i = p
    }
  }

  bubbleDown(i) {
    while (i << 1 < this.arr.length) {
      let c = i
      const c1 = (i << 1) + 1
      const c2 = c1 + 1
      if (this.arr[c1]?.[1] < this.arr[c][1]) { c = c1 }
      if (this.arr[c2]?.[1] < this.arr[c][1]) { c = c2 }
      if (c === i) { return }
      this.swap(i, c)
      i = c
    }
  }

  add(el, val) {
    this.arr.push([el, val])
    this.bubbleUp(this.arr.length - 1)
  }

  pop() {
    const result = this.arr[0]
    this.arr[0] = this.arr[this.arr.length - 1]
    this.arr.pop()
    this.bubbleDown(0)
    return result
  }

  print() {
    console.log(this.arr);
  }
}

export default maxPoints
