/**
 * @param {number} n
 * @param {number[][]} edges
 * @param {number[][]} query
 * @return {number[]}
 */

// because cost is a bitwise and
// we can traverse same edge several times
// and it doesn't affect the result
// a & a === a
//
// order is irrelevant
// as cost operator is associative
// (a & b) & c = a & (b & c)
//
// end result has bit set to 1
// when it's present in every used weight
//
// most beneficial is situation where
// some fragment of bitwise sum will result in 0
// ... & 1100 & 0011 & ... = 0
//
// using vertice with bitwise cost 111111
// doesn't affect existing solution
// a & 111111 = a

// ideas
//
// a
// is it always beneficial to include all
// possible edges, so that this situation is
// more likely to occur?
//
// b
// create hash: h(edge) => set of possible edges to arrive
//
// c
// any two edges that are part of same connected graph
// will have the same result

var minimumCost = function(n, edges, query) {
  // create list of connected graphs
  const graphs = [null]

  // create map: edge => graph idx
  const lookup = new Map()

  // create map: graph idx => sum
  const sums = new Map()

  // walk the edges, update graphs, lookup and sums
  for (let edge of edges) {
    const idx0 = lookup.get(edge[0])
    const idx1 = lookup.get(edge[1])

    // both missing: create new set
    if (!idx0 && !idx1) {
      graphs.push(new Set([edge[0], edge[1]]))
      const idx = graphs.length - 1
      lookup.set(edge[0], idx)
      lookup.set(edge[1], idx)
      sums.set(idx, edge[2])
      continue
    }

    // one missing: add to set 0
    if (idx0 && !idx1) {
      graphs[idx0].add(edge[1])
      lookup.set(edge[1], idx0)
      sums.set(idx0, sums.get(idx0) & edge[2])
      continue
    }

    // one missing: add to set 1
    if (idx1 && !idx0) {
      graphs[idx1].add(edge[0])
      lookup.set(edge[0], idx1)
      sums.set(idx1, sums.get(idx1) & edge[2])
      continue
    }

    // both present: merge sets
    const [smallerIdx, largerIdx] = 
      graphs[idx0].size < graphs[idx1].size
        ? [idx0, idx1]
        : [idx1, idx0]
    const larger = graphs[largerIdx]
    for (let i of graphs[smallerIdx]) {
      larger.add(i)
      lookup.set(i, largerIdx)
    }
    sums.set(largerIdx, sums.get(largerIdx) & sums.get(smallerIdx) & edge[2])
  }

  // for each query, find their connected graph
  for (let i = 0; i < query.length; i++) {
    // if it exists return that graph sum, otherwise -1
    const set0 = lookup.get(query[i][0])
    const set1 = lookup.get(query[i][1])
    query[i] = !!set0 && !!set1 && set0 === set1
      ? sums.get(set0)
      : -1
  }

  return query
};

export default minimumCost

// case 1
// 0 <-(111)-> 1
// 1 <-(111)-> 3
// 1 <-(001)-> 2
//
// query 1: 0,3
//
// visited: [0]
// possible: [1]
// sum: null 
//
// visited: [0, 1]
// possible: [2, 3]
// sum: 111
//
// visited: [0, 1, 2]
// possible: [3]
// sum: 111
//
// visited: [0, 1, 2, 3]
// possible: []
// sum: 001
//
// result: 1
//
// query: 2: 3,4
// not possible

// case 2
// [[0,2,7],[0,1,15],[1,2,6],[1,2,1]]
// [[0,2,"111"],[0,1,"1111"],[1,2,"110"],[1,2,"1"]]
//
// [0,2,"111"]
// 0 => A
// 2 => A
// A => 111
//
// [0,1,"1111"]
// zero is already in A, so add vertice 1 to it
// 0 => A
// 1 => A
// 2 => A
// A => 111
//
// [1,2,"110"]
// 0 => A
// 1 => A
// 2 => A
// A => 110
//
// [1,2,"1"]
// 0 => A
// 1 => A
// 2 => A
// A => 0
