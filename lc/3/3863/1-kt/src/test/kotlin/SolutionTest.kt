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
        val actual = solution.minOperations("dog")
        val expected = 1
        assertEquals(expected, actual)
    }

    @Test
    fun test2() {
        val actual = solution.minOperations("card")
        val expected = 2
        assertEquals(expected, actual)
    }

    @Test
    fun test3() {
        val actual = solution.minOperations("gf")
        val expected = -1
        assertEquals(expected, actual)
    }

    @Test
    fun test4() {
        val actual = solution.minOperations("a")
        val expected = 0
        assertEquals(expected, actual)
    }

    @Test
    fun test5() {
        val actual = solution.minOperations("acrd")
        val expected = 1
        assertEquals(expected, actual)
    }

    @Test
    fun test6() {
        val actual = solution.minOperations("daez")
        val expected = 1
        assertEquals(expected, actual)
    }

    @Test
    fun test7() {
        val actual = solution.minOperations("edc")
        val expected = 3
        assertEquals(expected, actual)
    }

    @Test
    fun test8() {
        val actual = solution.minOperations("jgg")
        val expected = 2
        assertEquals(expected, actual)
    }

    @Test
    fun test9() {
        val actual = solution.minOperations("zaza")
        val expected = 2
        assertEquals(expected, actual)
    }

    @Test
    fun test10() {
        val actual = solution.minOperations("zzaa")
        val expected = 2
        assertEquals(expected, actual)
    }

    @Test
    fun test11() {
        val actual = solution.minOperations("zaa")
        val expected = 2
        assertEquals(expected, actual)
    }

}
