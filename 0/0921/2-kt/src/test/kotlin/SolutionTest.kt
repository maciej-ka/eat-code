import java.util.*
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
        val actual = solution.minAddToMakeValid("())")
        val expected = 1
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.minAddToMakeValid("(((")
        val expected = 3
        assertEquals(expected, actual)
    }

}
