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
        var actual = solution.sumOfSquares(new int[] {1, 2, 3, 4});
        var expected = 21;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.sumOfSquares(new int[] {2, 7, 1, 19, 18, 3});
        var expected = 63;
        assertEquals(expected, actual);
    }

}