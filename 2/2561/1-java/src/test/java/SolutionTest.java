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
        var actual = solution.minCost(new int[] {4, 2, 2, 2}, new int[] {1, 4, 1, 2});
        var expected = 1;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.minCost(new int[] {2, 3, 4, 1}, new int[] {3, 2, 5, 1});
        var expected = -1;
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        var actual = solution.minCost(new int[] {84,80,43,8,80,88,43,14,100,88}, new int[] {32,32,42,68,68,100,42,84,14,8});
        var expected = 48;
        assertEquals(expected, actual);
    }
}