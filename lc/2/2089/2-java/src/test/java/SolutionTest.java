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
        var actual = solution.targetIndices(new int[] {1, 2, 5, 2, 3}, 2);
        var expected = List.of(1, 2);
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.targetIndices(new int[] {1, 2, 5, 2, 3}, 3);
        var expected = List.of(3);
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        var actual = solution.targetIndices(new int[] {1, 2, 5, 2, 3}, 5);
        var expected = List.of(4);
        assertEquals(expected, actual);
    }

    @Test
    void test4() {
        var actual = solution.targetIndices(new int[] {1, 2, 5, 2, 3}, 6);
        var expected = List.of();
        assertEquals(expected, actual);
    }

}