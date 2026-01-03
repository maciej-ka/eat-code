import org.junit.jupiter.api.Test
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Assertions.*

class SolutionTest {

    @BeforeEach
    void setUp() {
      solution = new Solution();
    }

    @Test
    fun test1() {
        val actual = solution.solve(arrayOf(1, 2, 3))
        val expected = 3
        assertEquals(expected, actual)
    }
}
