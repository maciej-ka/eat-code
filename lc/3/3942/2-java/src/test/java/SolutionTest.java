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
        var actual = solution.minOperations(new int[] {0, 2, 1});
        var expected = 2;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.minOperations(new int[] {1, 0, 2});
        var expected = 2;
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        var actual = solution.minOperations(new int[] {2, 0, 1, 3});
        var expected = -1;
        assertEquals(expected, actual);
    }

    @Test
    void test4() {
        var actual = solution.minOperations(new int[] {0});
        var expected = 0;
        assertEquals(expected, actual);
    }

    @Test
    void test5() {
        var actual = solution.minOperations(new int[] {2, 3, 0, 1});
        var expected = 2;
        assertEquals(expected, actual);
    }

    @Test
    void test6() {
        var actual = solution.minOperations(new int[] {2, 1, 0, 3});
        var expected = 2;
        assertEquals(expected, actual);
    }

}