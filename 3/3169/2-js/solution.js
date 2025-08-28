/**
 * @param {number} days
 * @param {number[][]} meetings
 * @return {number}
 */
var countDays = function(days, meetings) {
  // build a treap of ranges
  // ordered by range start
  let root = { range: meetings[0] }

  // try to place range
  // in relation to node
  function add(range, n) {
    const isOnRight = range[0] > n.range[1] + 1
    const isOnLeft = range[1] < n.range[0] - 1

    if (isOnRight) {
      if (n.right) { return add(range, n.right) }
      n.right = { range, parent: n }
      return
    }

    if (isOnLeft) {
      if (n.left) { return add(range, n.left) }
      n.left = { range, parent: n }
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
    target.range[0] = Math.min(n.range[0], target.range[0])
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
    if (isDisjoint) { return expandRight(n.left, target) }

    // consume
    target.range[1] = Math.max(n.range[1], target.range[1])
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
    // random swap next meeting
    const offset = Math.random() * (meetings.length - i)
    const k = Math.floor(offset + i)
    const temp = meetings[i]
    meetings[i] = meetings[k]
    meetings[k] = temp

    // either add or set merge target
    const target = add(meetings[i], root)
    if (!target) { continue }

    // expand left
    if (target.range[0] > meetings[i][0]) {
      target.range[0] = meetings[i][0]
      expandLeft(target.left, target)
    }

    // expand right
    if (target.range[1] < meetings[i][1]) {
      target.range[1] = meetings[i][1]
      expandRight(target.right, target)
    }
  }

  function walk(n) {
    if (!n) { return }
    walk(n.left)
    days -= n.range[1] - n.range[0] + 1
    walk(n.right)
  }

  walk(root)
  return days
};

export default countDays
