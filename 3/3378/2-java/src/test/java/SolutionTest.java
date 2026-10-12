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
        var actual = solution.countComponents(new int[] {2, 4, 8, 3, 9}, 5);
        var expected = 4;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.countComponents(new int[] {2, 4, 8, 3, 9, 12}, 10);
        var expected = 2;
        assertEquals(expected, actual);
    }

}