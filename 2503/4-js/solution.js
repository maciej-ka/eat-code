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
  function expand(x, y) {
    if (!grid[x][y]) { return }
    border.add(grid[x][y], x, y)
    // mark field as in border
    grid[x][y] = false
  }

  // initalize border
  expand(0, 0)

  // place to store query results
  const results = new Map()

  // grid size
  const xLast = grid.length - 1
  const yLast = grid[0].length - 1

  // process queries one by one
  // try to enlarge teritory
  for (let q of sorted) {
    while (q > border.top()) {
      const [x, y] = border.pop()
      visited++
      if (x > 0) { expand(x - 1, y) }
      if (y > 0) { expand(x, y - 1) }
      if (x < xLast) { expand(x + 1, y) }
      if (y < yLast) { expand(x, y + 1) }
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
    this.last = -1
    this.val = []
    this.x = []
    this.y = []
  }

  top() {
    if (this.last < 0) { return undefined }
    return this.val[0]
  }

  swap(i, k) {
    // val
    let temp = this.val[i]
    this.val[i] = this.val[k]
    this.val[k] = temp
    // x
    temp = this.x[i]
    this.x[i] = this.x[k]
    this.x[k] = temp
    // y
    temp = this.y[i]
    this.y[i] = this.y[k]
    this.y[k] = temp
  }

  bubbleUp(i) {
    while (i) {
      const p = i - 1 >> 1
      if (this.val[i] >= this.val[p]) { return }
      this.swap(i, p)
      i = p
    }
  }

  bubbleDown(i) {
    while (i << 1 < this.last) {
      let c = i
      const c1 = (i << 1) + 1
      const c2 = c1 + 1
      if (c1 <= this.last && this.val[c1] < this.val[c]) { c = c1 }
      if (c2 <= this.last && this.val[c2] < this.val[c]) { c = c2 }
      if (c === i) { return }
      this.swap(i, c)
      i = c
    }
  }

  add(val, x, y) {
    this.last++
    this.val[this.last] = val
    this.x[this.last] = x
    this.y[this.last] = y
    this.bubbleUp(this.last)
  }

  pop() {
    const x = this.x[0]
    const y = this.y[0]
    this.swap(0, this.last)
    this.last--
    this.bubbleDown(0)
    return [x, y]
  }
}

export default maxPoints
