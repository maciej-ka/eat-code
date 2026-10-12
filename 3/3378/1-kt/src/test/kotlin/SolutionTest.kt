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
        val actual = solution.countComponents(intArrayOf(2, 4, 8, 3, 9), 5)
        val expected = 4
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.countComponents(intArrayOf(2, 4, 8, 3, 9, 12), 10)
        val expected = 2
        assertEquals(expected, actual)
    }

}
