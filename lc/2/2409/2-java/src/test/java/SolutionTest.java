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
        var actual = solution.countDaysTogether("08-15", "08-18", "08-16", "08-19");
        var expected = 3;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.countDaysTogether("10-01", "10-31", "11-01", "12-31");
        var expected = 0;
        assertEquals(expected, actual);
    }

}