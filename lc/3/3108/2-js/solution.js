/**
 * @param {number} n
 * @param {number[][]} edges
 * @param {number[][]} query
 * @return {number[]}
 */

// implemented after watching Union Find
// called also Disjoint Set Union, DSU
// https://www.youtube.com/watch?v=ayW5B2W9hfo

var minimumCost = function(n, edges, query) {
  // store bit sum for each graph
  const sums = new Uint32Array(n)

  // in one structure hold two informations:
  // 1. what is graph id for a vertice
  // 2. how graphs have been joined
  // it's done by using vertice idx as graph id
  // which allows parent - child relationship
  const graphIds = new Uint32Array(n)

  // initialize
  for (let i = 0; i < n; i++) { 
    // assign vertice to it's own one element graph
    graphIds[i] = i

    // binary of ones, at least 10e5
    // which is 2 ** 17 - 1 = 131071
    sums[i] = 131071
  }

  // find graph identifier for vertice
  function getGraphId(v) {
    if (graphIds[v] === v) { return v }
    // when there is traversing
    // update graph on returing
    // so that future lookup will be shorter
    return graphIds[v] = getGraphId(graphIds[v])
  }

  for (let edge of edges) {
    const idx0 = getGraphId(edge[0])
    const idx1 = getGraphId(edge[1])
    // merge graph 1 into 0
    // shorten lookup for edge[1]
    graphIds[edge[1]] = graphIds[idx1] = idx0
    sums[idx0] = sums[idx0] & sums[idx1] & edge[2]
  }

  // resolve queries in place
  for (let i = 0; i < query.length; i++) {
    const idx0 = getGraphId(query[i][0])
    const idx1 = getGraphId(query[i][1])
    query[i] = idx0 === idx1
      ? sums[idx0]
      : -1
  }

  return query
};

export default minimumCost
