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
        val actual = solution.countDaysTogether("08-15", "08-18", "08-16", "08-19")
        val expected = 3
        assertEquals(expected, actual)
    }

    // @Test
    // fun test2() {
    //     val actual = solution.countDaysTogether("10-01", "10-31", "11-01", "12-31")
    //     val expected = 0
    //     assertEquals(expected, actual)
    // }

}
