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
        int actual = solution.myFunction(new int[] {3, 6, 1, 2, 5}, 2);
        int expected = 2;
        assertEquals(expected, actual);
    }
}