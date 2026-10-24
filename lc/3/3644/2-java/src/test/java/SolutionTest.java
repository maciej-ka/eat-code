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
        var actual = solution.sortPermutation(new int[] {0, 3, 2, 1});
        var expected = 1;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.sortPermutation(new int[] {0, 1, 3, 2});
        var expected = 2;
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        var actual = solution.sortPermutation(new int[] {3, 2, 1, 0});
        var expected = 0;
        assertEquals(expected, actual);
    }

}