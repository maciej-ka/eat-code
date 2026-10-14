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
        var actual = solution.maxSumOfSquares(2, 3);
        var expected = "30";
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.maxSumOfSquares(2, 17);
        var expected = "98";
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        var actual = solution.maxSumOfSquares(1, 10);
        var expected = "";
        assertEquals(expected, actual);
    }

}