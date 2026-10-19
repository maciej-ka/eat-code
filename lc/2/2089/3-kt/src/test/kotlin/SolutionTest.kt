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
        val actual = solution.targetIndices(intArrayOf(1, 2, 5, 2, 3), 2)
        val expected = listOf(1, 2)
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.targetIndices(intArrayOf(1, 2, 5, 2, 3), 3)
        val expected = listOf(3)
        assertEquals(expected, actual)
    }

    @Test
    fun test3() {
        val actual = solution.targetIndices(intArrayOf(1, 2, 5, 2, 3), 5)
        val expected = listOf(4)
        assertEquals(expected, actual)
    }

    @Test
    fun test4() {
        val actual = solution.targetIndices(intArrayOf(1, 2, 5, 2, 3), 6)
        val expected = listOf<Int>()
        assertEquals(expected, actual)
    }

}
