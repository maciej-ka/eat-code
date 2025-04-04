/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }

// two phases
// descend to find deepest leaves
// ascend to find common ancestor

// for both use the same set of nodes

/**
 * @param {TreeNode} root
 * @return {TreeNode}
 */
var lcaDeepestLeaves = function(root) {
  // a set of nodes
  let set = new Set([root])
  let newSet = new Set()
  let temp;

  // descend
  while (true) {
    newSet.clear()
    for (let node of set) {
      if (node.left) {
        node.left.p = node
        newSet.add(node.left)
      }
      if (node.right) {
        node.right.p = node
        newSet.add(node.right)
      }
    }
    if (!newSet.size) { break }
    temp = set
    set = newSet
    newSet = temp
  }

  // ascend
  while (set.size > 1) {
    newSet.clear()
    for (let node of set) {
      newSet.add(node.p)
    }
    temp = set
    set = newSet
    newSet = temp
  }

  return Array.from(set)[0]
};
