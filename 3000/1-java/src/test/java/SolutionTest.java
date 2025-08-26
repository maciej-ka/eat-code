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
        var actual = solution.areaOfMaxDiagonal(new int[][] {{9, 3}, {8, 6}});
        var expected = 48;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.areaOfMaxDiagonal(new int[][] {{3, 4}, {4, 3}});
        var expected = 12;
        assertEquals(expected, actual);
    }
}