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
        var actual = solution.maxSum(List.of(2,6,5,8), 2);
        var expected = 261;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.maxSum(List.of(4,5,4,7), 3);
        var expected = 90;
        assertEquals(expected, actual);
    }

}