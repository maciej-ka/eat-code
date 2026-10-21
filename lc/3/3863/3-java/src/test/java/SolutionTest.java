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
        var actual = solution.minOperations("dog");
        var expected = 1;
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var actual = solution.minOperations("card");
        var expected = 2;
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        var actual = solution.minOperations("gf");
        var expected = -1;
        assertEquals(expected, actual);
    }

    @Test
    void test4() {
        var actual = solution.minOperations("a");
        var expected = 0;
        assertEquals(expected, actual);
    }

    @Test
    void test5() {
        var actual = solution.minOperations("acrd");
        var expected = 1;
        assertEquals(expected, actual);
    }

    @Test
    void test6() {
        var actual = solution.minOperations("daez");
        var expected = 1;
        assertEquals(expected, actual);
    }

    @Test
    void test7() {
        var actual = solution.minOperations("edc");
        var expected = 3;
        assertEquals(expected, actual);
    }

    @Test
    void test8() {
        var actual = solution.minOperations("jgg");
        var expected = 2;
        assertEquals(expected, actual);
    }

    @Test
    void test9() {
        var actual = solution.minOperations("zaza");
        var expected = 2;
        assertEquals(expected, actual);
    }

    @Test
    void test10() {
        var actual = solution.minOperations("zzaa");
        var expected = 2;
        assertEquals(expected, actual);
    }

    @Test
    void test11() {
        var actual = solution.minOperations("zaa");
        var expected = 2;
        assertEquals(expected, actual);
    }

}