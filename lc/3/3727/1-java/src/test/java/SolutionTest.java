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
        var actual = solution.maxAlternatingSum(new int[] {1, 2, 3});
        var expected = 12;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.maxAlternatingSum(new int[] {1,-1,2,-2,3,-3});
        var expected = 16;
        assertEquals(expected, actual);
    }

}