import java.util.*
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
        val actual = solution.maxSumOfSquares(2, 3)
        val expected = "30"
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.maxSumOfSquares(2, 17)
        val expected = "98"
        assertEquals(expected, actual)
    }

    @Test
    fun test3() {
        val actual = solution.maxSumOfSquares(1, 10)
        val expected = ""
        assertEquals(expected, actual)
    }

    @Test
    fun test4() {
        val actual = solution.maxSumOfSquares(1, 9)
        val expected = "9"
        assertEquals(expected, actual)
    }

}
