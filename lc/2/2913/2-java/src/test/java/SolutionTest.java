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
        var actual = solution.sumCounts(List.of(1,2,1));
        var expected = 15;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.sumCounts(List.of(1,1));
        var expected = 3;
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        var actual = solution.sumCounts(List.of(2,2,5,5));
        var expected = 22;
        assertEquals(expected, actual);
    }

    @Test
    void test4() {
        var actual = solution.sumCounts(List.of(5,2,4,2,1,3,2,4,3,1));
        var expected = 578;
        assertEquals(expected, actual);
    }

}