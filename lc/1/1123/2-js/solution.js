/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */

let maxDepth = 0
let result = null

// recurential visit
function visit(node, depth) {
  // keep track of maxDepth
  maxDepth = Math.max(maxDepth, depth)

  // is nil node
  if (!node) { return depth }

  // go down (update maxDepth)
  const leftDepth = visit(node.left, depth + 1)
  const rightDepth = visit(node.right, depth + 1)

  // if both left and right are maxDepth,
  // mark this node as solution candidate
  if (leftDepth === maxDepth && rightDepth === maxDepth) { result = node }

  // return subtree depth
  return Math.max(leftDepth, rightDepth)
}

/**
 * @param {TreeNode} root
 * @return {TreeNode}
 */
var lcaDeepestLeaves = function(root) {
  maxDepth = 0
  result = null
  visit(root, 0)
  // last solution candidate is highest in tree
  return result
};
