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
        val actual = solution.sumFourDivisors(intArrayOf(21, 4, 7))
        val expected = 32
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.sumFourDivisors(intArrayOf(21, 21))
        val expected = 64
        assertEquals(expected, actual)
    }

    @Test
    fun test3() {
        val actual = solution.sumFourDivisors(intArrayOf(1, 2, 3, 4, 5))
        val expected = 0
        assertEquals(expected, actual)
    }

    @Test
    fun test4() {
        val actual = solution.sumFourDivisors(intArrayOf(1, 2, 3, 4, 5, 6, 7, 8, 9, 10))
        val expected = 45
        assertEquals(expected, actual)
    }

}
