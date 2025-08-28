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
        int actual = solution.partitionArray(new int[] {3, 6, 1, 2, 5}, 2);
        int expected = 2;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        int actual = solution.partitionArray(new int[] {1, 2, 3}, 1);
        int expected = 2;
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        int actual = solution.partitionArray(new int[] {2, 2, 4, 5}, 0);
        int expected = 3;
        assertEquals(expected, actual);
    }
}