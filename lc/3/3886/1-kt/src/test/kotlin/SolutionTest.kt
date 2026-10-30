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
        val actual = solution.sortableIntegers(intArrayOf(3, 1, 2))
        val expected = 3
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.sortableIntegers(intArrayOf(7, 6, 5))
        val expected = 0
        assertEquals(expected, actual)
    }

    @Test
    fun test3() {
        val actual = solution.sortableIntegers(intArrayOf(5, 8))
        val expected = 3
        assertEquals(expected, actual)
    }

    @Test
    fun test4() {
        val actual = solution.sortableIntegers(intArrayOf(2, 1, 1, 2, 3, 2))
        val expected = 3
        assertEquals(expected, actual)
    }

    @Test
    fun test5() {
        val actual = solution.sortableIntegers(intArrayOf(3, 4, 5, 1, 2))
        val expected = 5
        assertEquals(expected, actual)
    }

    @Test
    fun test6() {
        val actual = solution.sortableIntegers(intArrayOf(2, 3, 4, 5, 6, 1))
        val expected = 6
        assertEquals(expected, actual)
    }

}
