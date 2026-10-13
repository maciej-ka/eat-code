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
        val actual = solution.maxDepthAfterSplit("(()())")
        val expected = intArrayOf(0,1,1,1,1,0)
        assertArrayEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.maxDepthAfterSplit("()(())()")
        val expected = intArrayOf(0,0,0,1,1,0,0,0)
        assertArrayEquals(expected, actual)
    }

}
