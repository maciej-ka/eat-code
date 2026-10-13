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
        val actual = solution.maxAlternatingSum(intArrayOf(1, 2, 3))
        val expected = 12L
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.maxAlternatingSum(intArrayOf(-3,-1,-2,1,3,2))
        val expected = 16L
        assertEquals(expected, actual)
    }

}
