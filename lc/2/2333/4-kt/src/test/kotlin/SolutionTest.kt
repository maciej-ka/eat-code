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
        val actual = solution.minSumSquareDiff(intArrayOf(1, 2, 3, 4), intArrayOf(2, 10, 20, 19), 0, 0)
        val expected = 579L
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.minSumSquareDiff(intArrayOf(1, 4, 10, 12), intArrayOf(5, 8, 6, 9), 1, 1)
        val expected = 43L
        assertEquals(expected, actual)
    }

    @Test
    fun test3() {
        val actual = solution.minSumSquareDiff(intArrayOf(105, 56, 1, 7, 0, 3, 77, 17), intArrayOf(103, 49, 3, 9, 9, 1, 80, 10), 2, 4)
        val expected = 122L
        assertEquals(expected, actual)
    }

}
