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
        val actual = solution.checkValidString("(**(()**)")
        val expected = true
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.checkValidString("(**(*")
        val expected = true
        assertEquals(expected, actual)
    }

    @Test
    fun test3() {
        val actual = solution.checkValidString("(**((*")
        val expected = false
        assertEquals(expected, actual)
    }

    @Test
    fun test4() {
        val actual = solution.checkValidString("(*))")
        val expected = true
        assertEquals(expected, actual)
    }

    @Test
    fun test5() {
        val actual = solution.checkValidString("(((((()*)(*)*))())())(()())())))((**)))))(()())()")
        val expected = false
        assertEquals(expected, actual)
    }

}
