import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;
import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {
  private Solution solution;

  @BeforeEach
  void setUp() {
    solution = new Solution();
  }

  @Test
  void test1() {
    var root = new TreeNode(
      1,
      new TreeNode(
        7,
        new TreeNode(7, null, null),
        new TreeNode(-8, null, null)
      ),
      new TreeNode(0, null, null)
    );
    var actual = solution.maxLevelSum(root);
    var expected = 2;
    assertEquals(expected, actual);
  }

  @Test
  void test2() {
    var root = new TreeNode(
      989,
      null,
      new TreeNode(
        10250,
        new TreeNode(98693, null, null),
        new TreeNode(-89388, null, null)
      )
    );
    var actual = solution.maxLevelSum(root);
    var expected = 2;
    assertEquals(expected, actual);
  }

}