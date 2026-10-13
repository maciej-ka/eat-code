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
        var actual = solution.bulbSwitch(3);
        var expected = 1;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.bulbSwitch(0);
        var expected = 0;
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        var actual = solution.bulbSwitch(1);
        var expected = 1;
        assertEquals(expected, actual);
    }

}