/**
 * @param {number[][]} grid
 * @param {number[]} queries
 * @return {number[]}
 */

// process smallest queries first
// and for each larger, expand teritory
// (don't calculate it from zero)

var maxPoints = function(grid, queries) {
  // place to store query results
  const results = new Map()

  // ascending queries
  const sorted = Array.from(new Set(queries)).sort((a,b) => a - b)

  // fields visited
  const visited = new Set()

  // fields that can be visited
  // but so far had too large value
  let border = [[0,0]]

  // stack of fields to resolve
  // into either visited or border
  let stack = []

  // grid size
  const xLast = grid.length - 1
  const yLast = grid[0].length - 1

  // plan to visit
  function plan(f) {
    if(!visited.has(f.toString())) {
      stack.push(f)
    }
  }

  // process queries one by one
  // try to enlarge teritory
  for (let q of sorted) {
    stack = border
    border = []

    while (stack.length) {
      const f = stack.pop()
      // too large
      if (grid[f[0]][f[1]] >= q) {
        border.push(f);
        continue
      }
      // visit
      visited.add(f.toString())
      if (f[0] > 0) { plan([f[0] - 1, f[1]]) }
      if (f[1] > 0) { plan([f[0], f[1] - 1]) }
      if (f[0] < xLast) { plan([f[0] + 1, f[1]]) }
      if (f[1] < yLast) { plan([f[0], f[1] + 1]) }
    }

    results.set(q, visited.size)
  }

  // format result in place
  for (let i in queries) {
    queries[i] = results.get(queries[i])
  }
  return queries
};

export default maxPoints
