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
        val actual = solution.closedIsland(arrayOf(
            intArrayOf(1,1,1,1,1,1,1,0),
            intArrayOf(1,0,0,0,0,1,1,0),
            intArrayOf(1,0,1,0,1,1,1,0),
            intArrayOf(1,0,0,0,0,1,0,1),
            intArrayOf(1,1,1,1,1,1,1,0),
        ))
        val expected = 2
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.closedIsland(arrayOf(
            intArrayOf(0,0,1,0,0),
            intArrayOf(0,1,0,1,0),
            intArrayOf(0,1,1,1,0),
        ))
        val expected = 1
        assertEquals(expected, actual)
    }

    @Test
    fun test3() {
        val actual = solution.closedIsland(arrayOf(
            intArrayOf(1,1,1,1,1,1,1),
            intArrayOf(1,0,0,0,0,0,1),
            intArrayOf(1,0,1,1,1,0,1),
            intArrayOf(1,0,1,0,1,0,1),
            intArrayOf(1,0,1,1,1,0,1),
            intArrayOf(1,0,0,0,0,0,1),
            intArrayOf(1,1,1,1,1,1,1)
        ))
        val expected = 2
        assertEquals(expected, actual)
    }

}
