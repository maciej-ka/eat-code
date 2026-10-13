// https://leetcode.com/problemsmallest-subtree-with-all-the-deepest-nodes/submissions/1879905740

namespace Solution;

public class TreeNode {
    public int val;
    public TreeNode left;
    public TreeNode right;
    public TreeNode(int val=0, TreeNode left=null, TreeNode right=null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
    public override String ToString() {
        return $"{this.val} ({this.left}, {this.right})";
    }
}

public class Solution {
    private TreeNode? left;
    private TreeNode? right;
    private int maxLevel = 0;
    private Dictionary<TreeNode, TreeNode> parent = new();

    public TreeNode SubtreeWithAllDeepest(TreeNode root) {
        Walk(root);
        while (left != right) {
            left = parent[left!];
            right = parent[right!];
        }
        return left!;
    }

    private void Walk(TreeNode node, int level = 1) {
        if (level > maxLevel) {
            left = right = node;
            maxLevel = level;
        }
        if (level == maxLevel) {
            right = node;
        }
        if (node.left != null) {
            parent.Add(node.left, node);
            Walk(node.left, level + 1);
        }
        if (node.right != null) {
            parent.Add(node.right, node);
            Walk(node.right, level + 1);
        }
    }
}
