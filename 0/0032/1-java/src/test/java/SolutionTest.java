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
        var actual = solution.longestValidParentheses("(()");
        var expected = 2;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.longestValidParentheses(")()())");
        var expected = 4;
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        var actual = solution.longestValidParentheses("");
        var expected = 0;
        assertEquals(expected, actual);
    }

    @Test
    void test4() {
        var actual = solution.longestValidParentheses(")(()()))(()()(()))(");
        var expected = 10;
        assertEquals(expected, actual);
    }

}