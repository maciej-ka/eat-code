/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number}
 */

var countCompleteComponents = function(n, edges) {
  // find connected graphs using Disjoint Set Union
  const p = new Uint8Array(n)
  for (let i = 0; i < n; i++) { p[i] = i }
  function find(i) { return i === p[i] ? i : p[i] = find(p[i]) }

  // edge count for each graph
  const eCount = new Uint16Array(n)
  // vertice count for each graph
  const vCount = new Uint8Array(n).fill(1)

  for (let edge of edges) {
    const p0 = find(edge[0])
    const p1 = find(edge[1])
    eCount[p0]++

    // merge into first graph
    if (p0 !== p1) {
      p[edge[1]] = p[p1] = p0
      eCount[p0] += eCount[p1]
      vCount[p0] += vCount[p1]
    }
  }

  // graph with n vertices is complete
  // when it has 1 + 2 + (n - 1) distinct edges
  let result = 0
  const seen = new Set()

  for (let i = 0; i < n; i++) {
    const id = find(i)
    if (!seen.has(id)) {
      const v = vCount[id]
      if (eCount[id] === v * (v - 1) >> 1) { result++ }
      seen.add(id)
    }
  }

  return result
};

export default countCompleteComponents
