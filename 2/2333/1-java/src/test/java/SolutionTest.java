import java.util.*;
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
        var actual = solution.minSumSquareDiff(new int[] {1, 2, 3, 4}, new int[] {2, 10, 20, 19}, 0, 0);
        var expected = 579;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.minSumSquareDiff(new int[] {1, 4, 10, 12}, new int[] {5, 8, 6, 9}, 1, 1);
        var expected = 43;
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        var actual = solution.minSumSquareDiff(new int[] {105, 56, 1, 7, 0, 3, 77, 17}, new int[] {103, 49, 3, 9, 9, 1, 80, 10}, 2, 4);
        var expected = 122;
        assertEquals(expected, actual);
    }

}