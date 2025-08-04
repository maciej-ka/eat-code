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
        var actual = solution.totalFruit(new int[] {1, 2, 1});
        var expected = 3;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.totalFruit(new int[] {0, 1, 2, 2});
        var expected = 3;
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        var actual = solution.totalFruit(new int[] {1, 2, 3, 2, 2});
        var expected = 4;
        assertEquals(expected, actual);
    }
}