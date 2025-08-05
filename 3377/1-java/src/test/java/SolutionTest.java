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
        var actual = solution.numOfUnplacedFruits(new int[] {4, 2, 5}, new int[] {3, 5, 4});
        var expected = 1;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.numOfUnplacedFruits(new int[] {3, 6, 1}, new int[] {6, 4, 7});
        var expected = 0;
        assertEquals(expected, actual);
    }
}