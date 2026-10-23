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
        val actual = solution.minOperations(intArrayOf(0, 2, 1))
        val expected = 2
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.minOperations(intArrayOf(1, 0, 2))
        val expected = 3
        assertEquals(expected, actual)
    }

    @Test
    fun test3() {
        val actual = solution.minOperations(intArrayOf(2, 0, 1, 3))
        val expected = -1
        assertEquals(expected, actual)
    }

    @Test
    fun test4() {
        val actual = solution.minOperations(intArrayOf(2))
        val expected = 0
        assertEquals(expected, actual)
    }

    @Test
    fun test5() {
        val actual = solution.minOperations(intArrayOf(2, 3, 0, 1))
        val expected = 2
        assertEquals(expected, actual)
    }

    @Test
    fun test6() {
        val actual = solution.minOperations(intArrayOf(2, 1, 0, 3))
        val expected = 2
        assertEquals(expected, actual)
    }

}
