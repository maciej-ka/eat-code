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
        val actual = solution.countIslands(arrayOf(
            intArrayOf(0,2,1,0,0),
            intArrayOf(0,5,0,0,5),
            intArrayOf(0,0,1,0,0),
            intArrayOf(0,1,4,7,0),
            intArrayOf(0,2,0,0,8)
        ), 5)
        val expected = 2
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.countIslands(arrayOf(
            intArrayOf(3,0,3,0),
            intArrayOf(0,3,0,3),
            intArrayOf(3,0,3,0),
        ), 3)
        val expected = 6
        assertEquals(expected, actual)
    }

}
