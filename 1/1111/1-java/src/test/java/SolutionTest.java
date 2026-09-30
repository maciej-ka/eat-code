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
        var actual = solution.maxDepthAfterSplit("(()())");
        var expected = new int[] {0,1,1,1,1,0};
        assertArrayEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.maxDepthAfterSplit("()(())()");
        var expected = new int[] {0,0,0,1,1,0,0,0};
        assertArrayEquals(expected, actual);
    }

}