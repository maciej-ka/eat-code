// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/submissions/1876749534

import java.util.HashMap;

class TreeNode {
  int val;
  TreeNode left;
  TreeNode right;
  TreeNode() {}
  TreeNode(int val) { this.val = val; }
  TreeNode(int val, TreeNode left, TreeNode right) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

class Solution {
  private HashMap<Integer, Integer> sums = new HashMap<>();

  public int maxLevelSum(TreeNode root) {
    visit(root, 1);

    var max = sums.get(1);
    var level = 1;
    for (var key: sums.keySet()) {
      var sum = sums.get(key);
      if (sum > max) {
        max = sum;
        level = key;
      }
    }

    return level;
  }

  void visit(TreeNode node, int level) {
    if (node == null) { return; }
    var prev = sums.getOrDefault(level, 0);
    sums.put(level, prev + node.val);
    visit(node.left, level + 1);
    visit(node.right, level + 1);
  }
}
