/**
 * @param {number} days
 * @param {number[][]} meetings
 * @return {number}
 */
var countDays = function(days, meetings) {
  // build a treap of ranges
  // ordered by range start
  let root = { range: meetings[0], priority: Math.random() }

  function _rotateRight(n) {
    const nextTop = n.left
    // move to top
    nextTop.parent = n.parent
    if (!n.parent) { root = nextTop }
    if (n === n.parent?.left) { n.parent.left = nextTop }
    if (n === n.parent?.right) { n.parent.right = nextTop }
    // grab right child
    n.left = nextTop.right
    if (nextTop.right) { nextTop.right.parent = n }
    // reassign n
    nextTop.right = n
    n.parent = nextTop
  }

  function _rotateLeft(n) {
    const nextTop = n.right
    // move to top
    nextTop.parent = n.parent
    if (!n.parent) { root = nextTop }
    if (n === n.parent?.left) { n.parent.left = nextTop }
    if (n === n.parent?.right) { n.parent.right = nextTop }
    // grab left child
    n.right = nextTop.left
    if (nextTop.left) { nextTop.left.parent = n }
    // reassign n
    nextTop.left = n
    n.parent = nextTop
  }

  // adjust priorities
  function _balance(n) {
    while (n.priority > n?.parent?.priority) {
      n === n.parent.left
        ? _rotateRight(n.parent)
        : _rotateLeft(n.parent)
    }
  }

  // try to place range
  // in relation to node
  function add(range, n) {
    const isOnRight = range[0] > n.range[1] + 1
    const isOnLeft = range[1] < n.range[0] - 1

    if (isOnRight) {
      if (n.right) { return add(range, n.right) }
      n.right = { range, priority: Math.random(), parent: n }
      _balance(n.right)
      return
    }

    if (isOnLeft) {
      if (n.left) { return add(range, n.left) }
      n.left = { range, priority: Math.random(), parent: n }
      _balance(n.left)
      return
    }

    // there is an overlap
    // return merge target
    return n
  }

  // target range has grown
  // check can it consume other nodes
  function expandLeft(n, target) {
    if (!n) { return }
    const isDisjoint = target.range[0] > n.range[1] + 1
    if (isDisjoint) { return expandLeft(n.right, target) }

    // consume
    target.range[0] = n.range[0]
    // substitue
    const newN = n.left
    if (n === n.parent.left) { n.parent.left = newN }
    if (n === n.parent.right) { n.parent.right = newN }
    if (newN) {
      newN.parent = n.parent
      expandLeft(newN, target)
    }
  }

  // target range has grown
  // check can it consume other nodes
  function expandRight(n, target) {
    if (!n) { return }
    const isDisjoint = target.range[1] < n.range[0] - 1
    if (isDisjoint) { return expandLeft(n.left, target) }

    // consume
    target.range[0] = n.range[0]
    // substitue
    const newN = n.right
    if (n === n.parent.left) { n.parent.left = newN }
    if (n === n.parent.right) { n.parent.right = newN }
    if (newN) {
      newN.parent = n.parent
      expandRight(newN, target)
    }
  }

  // main loop
  for (let i = 1; i < meetings.length; i++) {
    // either add or set merge target
    const target = add(meetings[i], root)
    if (!target) { continue }

    // expand left
    if (meetings[i][0] < target.range[0]) {
      target.range[0] = meetings[i][0]
      expandLeft(target.left, target)
    }

    // expand right
    if (meetings[i][1] > target.range[1]) {
      target.range[1] = meetings[i][1]
      expandRight(target.right, target)
    }
  }

  // walk the tree, perform count
  function walk(n) {
    if (!n) { return }
    walk(n.left)
    days -= n.range[1] - n.range[0] + 1
    console.log(n.range, 'L:', n.left?.range, 'R:', n.right?.range)
    walk(n.right)
  }

  walk(root)
  return days
};

export default countDays
