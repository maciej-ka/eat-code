namespace Solution.Tests;

public class SolutionTests {

    [Fact]
    public void test1() {
        TreeNode expected = new TreeNode {
            val = 2,
            left = new TreeNode { val = 7 },
            right = new TreeNode { val = 4 }
        };
        TreeNode root = new TreeNode {
            val = 3,
            left = new TreeNode {
                val = 5,
                left = new TreeNode { val = 6 },
                right = expected
            },
            right = new TreeNode {
                val = 1,
                left = new TreeNode { val = 0 },
                right = new TreeNode { val = 8 }
            }
        };
        TreeNode actual = new Solution().SubtreeWithAllDeepest(root);
        Assert.Equal(expected, actual);
    }

    [Fact]
    public void test2() {
        TreeNode root = new TreeNode { val = 1 };
        TreeNode actual = new Solution().SubtreeWithAllDeepest(root);
        Assert.Equal(root, actual);
    }

    [Fact]
    public void test3() {
        TreeNode expected = new TreeNode { val = 2 };
        TreeNode root = new TreeNode {
            val = 0,
            left = new TreeNode {
                val = 1,
                right = expected
            },
            right = new TreeNode { val = 3 }
        };
        TreeNode actual = new Solution().SubtreeWithAllDeepest(root);
        Assert.Equal(expected, actual);
    }

}