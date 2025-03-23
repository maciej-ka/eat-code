/**
 * @param {number} n
 * @param {number[][]} roads
 * @return {number}
 */
var countPaths = function(n, roads) {
  // faster adjacency lookup
  const adj = new Array(n)
  for (let i = 0; i < n; i++) { adj[i] = [] }
  for (let [v,u,t] of roads) {
    adj[v].push([u,t])
    adj[u].push([v,t])
  }

  // time to reach vertice
  const time = new Array(n).fill(Infinity)
  time[0] = 0

  // [0] [1,2] [3,4,5,6]
  // queue of vertices
  const queue = new Uint8Array(n)
  // vertice queue position
  const vPos = new Uint8Array(n)
  // initialize
  for (let i in queue) { vPos[i] = queue[i] = i }
  // place removed elements at the end
  let qLength = n

  function _swap(i,k) {
    // swap vertice positions
    let temp = vPos[queue[i]]
    vPos[queue[i]] = vPos[queue[k]]
    vPos[queue[k]] = temp
    // swap values in queue
    temp = queue[i]
    queue[i] = queue[k]
    queue[k] = temp
  }

  // get next vertice
  function pop() {
    _swap(0, --qLength)
    let i = 0
    while (true) {
      let c = i;
      const c1 = (i << 1) + 1
      const c2 = (i << 1) + 2
      if (c1 >= qLength) { break }
      if (time[queue[c1]] < time[queue[c]]) { c = c1 }
      if (time[queue[c2]] < time[queue[c]] && c2 < qLength) { c = c2 }
      if (c === i) { break }
      _swap(i, c)
      i = c
    }
    return queue[qLength]
  }

  // bubble up vertice after value decrease
  function update(v) {
    let i = vPos[v]
    while (i) {
      const ip = (i - 1) >> 1
      if (time[queue[ip]] <= time[queue[i]]) { return }
      _swap(i, ip)
      i = ip
    }
  }

  // Dijkstra
  while(true) {
    // stop when target is visited
    const active = pop()
    if (active === n - 1) { break }
    // update adjecent times
    for (const [v,t] of adj[active]) {
      const tSum = time[active] + t
      if (tSum < time[v]) {
        time[v] = tSum
        update(v)
      }
    }
  }
};

export default countPaths
