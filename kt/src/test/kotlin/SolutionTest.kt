import org.junit.jupiter.api.Test
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Assertions.*

class SolutionTest {
    private lateinit var solution: Solution

    @BeforeEach
    fun setUp() {
        solution = Solution();
    }

    @Test
    fun test1() {
        val actual = solution.solve(intArrayOf(1, 2, 3))
        val expected = 3
        assertEquals(expected, actual)
    }

}
