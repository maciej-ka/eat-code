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

  // try to add new node
  function add(range, n) {
    const isOnTheRight = range[0] > n.range[1] + 1
    const isOnTheLeft = range[1] < n.range[0] - 1

    if (isOnTheRight) {
      if (n.right) { return add(range, n.right) }
      n.right = { range, priority: Math.random(), parent: n }
      _balance(n.right)
      return true
    }

    if (isOnTheLeft) {
      if (n.left) { return add(range, n.left) }
      n.left = { range, priority: Math.random(), parent: n }
      _balance(n.left)
      return true
    }

    // there is an overlap
    // return merge target
    return n
  }

  for (let i = 1; i < meetings.length; i++) {
    // try to add
    add(meetings[i], root)
  }

  function print(n) {
    if (!n) { return }
    print(n.left)
    console.log(n.range, 'L:', n.left?.range, 'R:', n.right?.range)
    print(n.right)
  }
  print(root)

  // walk the tree, perform count
};

export default countDays
