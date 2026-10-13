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
        val actual = solution.maxSum(listOf(2,6,5,8), 2)
        val expected = 261
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.maxSum(listOf(4,5,4,7), 3)
        val expected = 90
        assertEquals(expected, actual)
    }

}
