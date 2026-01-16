import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.util.List;

class SolutionTest {

  @Test
  void test_basic() {
    var actual = Solution.processArray(new int[] {-1, -2, -3, 2});
    var expected = List.of(-1, -3);
    assertEquals(expected, actual);
  }

  @Test
  void test_zero() {
    var actual = Solution.processArray(new int[] {-1, 0, -2, -3, 2});
    var expected = List.of(-1, -3);
    assertEquals(expected, actual);
  }

  @Test
  void test_out_of_bounds() {
    var actual = Solution.processArray(new int[] {-1, -2, -3, 20000});
    var expected = List.of(-1, -2, -3);
    assertEquals(expected, actual);
  }

  @Test
  void test_shuffled() {
    var actual = Solution.processArray(new int[] {3, -1, -2, 1, -3});
    var expected = List.of(-2);
    assertEquals(expected, actual);
  }

  @Test
  void test_repeated_skip() {
    var actual = Solution.processArray(new int[] {-1, -2, -3, 2, 2, 3});
    var expected = List.of(-1);
    assertEquals(expected, actual);
  }

}