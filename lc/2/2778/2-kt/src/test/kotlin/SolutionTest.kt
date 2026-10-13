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
        val actual = solution.sumOfSquares(intArrayOf(1, 2, 3, 4))
        val expected = 21
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.sumOfSquares(intArrayOf(2,7,1,19,18,3))
        val expected = 63 
        assertEquals(expected, actual)
    }

}
