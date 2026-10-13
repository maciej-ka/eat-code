import org.junit.jupiter.api.Test
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Assertions.*

class SolutionTest {
    private lateinit var solution: Solution

    @BeforeEach
    fun setUp() {
        solution = Solution()
    }

    @Test
    fun test1() {
        val actual = solution.isValid("()[]{}")
        val expected = true
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.isValid("([)]")
        val expected = false
        assertEquals(expected, actual)
    }

    @Test
    fun test3() {
        val actual = solution.isValid("]")
        val expected = false
        assertEquals(expected, actual)
    }

}
