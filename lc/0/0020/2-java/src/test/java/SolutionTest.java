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
        var actual = solution.isValid("()");
        var expected = true;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.isValid("()[]{}");
        var expected = true;
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        var actual = solution.isValid("(]");
        var expected = false;
        assertEquals(expected, actual);
    }

    @Test
    void test4() {
        var actual = solution.isValid("([])");
        var expected = true;
        assertEquals(expected, actual);
    }

    @Test
    void test5() {
        var actual = solution.isValid("([)]");
        var expected = false;
        assertEquals(expected, actual);
    }

    @Test
    void test6() {
        var actual = solution.isValid("]");
        var expected = false;
        assertEquals(expected, actual);
    }

}