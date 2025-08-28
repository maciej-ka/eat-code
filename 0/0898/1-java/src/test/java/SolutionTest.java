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
        int actual = solution.subarrayBitwiseORs(new int[] {0});
        int expected = 1;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        int actual = solution.subarrayBitwiseORs(new int[] {1, 1, 2});
        int expected = 3;
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        int actual = solution.subarrayBitwiseORs(new int[] {1, 2, 4});
        int expected = 6;
        assertEquals(expected, actual);
    }
}