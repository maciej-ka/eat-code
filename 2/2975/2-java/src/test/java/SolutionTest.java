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
    var actual = solution.maximizeSquareArea(4, 3, new int[] {2, 3}, new int[] {2});
    var expected = 4;
    assertEquals(expected, actual);
  }

  @Test
  void test2() {
    var actual = solution.maximizeSquareArea(6, 7, new int[] {2}, new int[] {4});
    var expected = -1;
    assertEquals(expected, actual);
  }

}