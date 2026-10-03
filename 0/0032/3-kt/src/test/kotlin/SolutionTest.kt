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
        val actual = solution.longestValidParentheses("(()")
        val expected = 2
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.longestValidParentheses(")()())")
        val expected = 4
        assertEquals(expected, actual)
    }

    @Test
    fun test3() {
        val actual = solution.longestValidParentheses("")
        val expected = 0
        assertEquals(expected, actual)
    }

}
