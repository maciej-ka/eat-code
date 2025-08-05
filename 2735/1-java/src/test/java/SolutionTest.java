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
        var actual = solution.minCost(new int[] {20, 1, 15}, 5);
        var expected = 13;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.minCost(new int[] {1, 2, 3}, 4);
        var expected = 6;
        assertEquals(expected, actual);
    }
}