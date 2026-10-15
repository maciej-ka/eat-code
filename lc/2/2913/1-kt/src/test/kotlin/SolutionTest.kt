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
        val actual = solution.sumCounts(listOf(1,2,1))
        val expected = 15
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.sumCounts(listOf(1,1))
        val expected = 3
        assertEquals(expected, actual)
    }

    @Test
    fun test3() {
        val actual = solution.sumCounts(listOf(2,2,5,5))
        val expected = 22
        assertEquals(expected, actual)
    }

    @Test
    fun test4() {
        val actual = solution.sumCounts(listOf(5,2,4,2,1,3,2,4,3,1))
        val expected =578
        assertEquals(expected, actual)
    }

}
