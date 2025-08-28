/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number}
 */

var countCompleteComponents = function(n, edges) {
  // find connected graphs using Disjoint Set Union
  const p = new Uint8Array(n)
  for (let i in p) { p[i] = i }
  function find(i) { return i === p[i] ? i : p[i] = find(p[i]) }

  // edge count for each graph
  const eCount = new Uint16Array(n)

  for (let edge of edges) {
    const p0 = find(edge[0])
    const p1 = find(edge[1])
    eCount[p0]++

    // merge into first graph
    if (p0 !== p1) {
      p[edge[1]] = p[p1] = p0
      eCount[p0] += eCount[p1]
    }
  }

  // vertice count for each graph
  const vCount = new Map()
  for (let i in p) {
    const id = find(i)
    vCount.set(id, (vCount.get(id) || 0) + 1)
  }

  // graph with n vertices is complete
  // when it has 1 + 2 + (n - 1) distinct edges
  let result = 0
  for (let id of vCount.keys()) {
    const v = vCount.get(id)
    const e = eCount[id];
    if (e === v * (v - 1) >> 1) { result++ }
  }

  return result
};

export default countCompleteComponents
