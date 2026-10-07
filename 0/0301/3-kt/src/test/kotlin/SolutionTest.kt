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
        val actual = solution.removeInvalidParentheses("()())()")
        val expected = listOf("(())()", "()()()")
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.removeInvalidParentheses("(a)())()")
        val expected = listOf("(a())()", "(a)()()")
        assertEquals(expected, actual)
    }

    @Test
    fun test3() {
        val actual = solution.removeInvalidParentheses(")(")
        val expected = listOf("")
        assertEquals(expected, actual)
    }

    @Test
    fun test4() {
        val actual = solution.removeInvalidParentheses("()())(()")
        val expected = listOf("(())()", "()()()")
        assertEquals(expected, actual)
    }

    @Test
    fun test5() {
        val actual = solution.removeInvalidParentheses("()))()((()")
        val expected = listOf("()()()")
        assertEquals(expected, actual)
    }

    @Test
    fun test6() {
        val actual = solution.removeInvalidParentheses("((()")
        val expected = listOf("()")
        assertEquals(expected, actual)
    }

    @Test
    fun test7() {
        val actual = solution.removeInvalidParentheses("()))")
        val expected = listOf("()")
        assertEquals(expected, actual)
    }

    @Test
    fun test8() {
        val actual = solution.removeInvalidParentheses("(((k()((")
        val expected = listOf("(k)", "k()")
        assertEquals(expected, actual)
    }

}
