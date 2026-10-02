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
        var actual = solution.generateParenthesis(3);
        var expected = List.of("((()))", "(()())", "(())()", "()(())", "()()()");
        assertEquals(expected, actual);
    }

}