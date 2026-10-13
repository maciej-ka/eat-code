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
        var list = List.of(
            List.of("a"),
            List.of("c"),
            List.of("d"),
            List.of("a", "b"),
            List.of("c", "b"),
            List.of("d", "a")
       );
        var actual = solution.deleteDuplicateFolder(list);
        var expected = List.of(
            List.of("d"),
            List.of("d", "a")
        );
        assertEquals(expected, actual);
    }

    @Test
    void test2() {
        var list = List.of(
            List.of("a"),
            List.of("c"),
            List.of("a", "b"),
            List.of("c", "b"),
            List.of("a", "b", "x"),
            List.of("a", "b", "x", "y"),
            List.of("w"),
            List.of("w", "y")
       );
        var actual = solution.deleteDuplicateFolder(list);
        var expected = List.of(
            List.of("a"),
            List.of("a", "b"),
            List.of("c"),
            List.of("c", "b")
        );
        assertEquals(expected, actual);
    }

    @Test
    void test3() {
        var list = List.of(
            List.of("a", "b"),
            List.of("c", "d"),
            List.of("c"),
            List.of("a")
       );
        var actual = solution.deleteDuplicateFolder(list);
        var expected = List.of(
            List.of("a"),
            List.of("a", "b"),
            List.of("c"),
            List.of("c", "d")
        );
        assertEquals(expected, actual);
    }

}