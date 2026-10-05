import java.util.*
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
        val list = listOf(
            listOf("a"),
            listOf("c"),
            listOf("d"),
            listOf("a", "b"),
            listOf("c", "b"),
            listOf("d", "a")
        )
        val actual = solution.deleteDuplicateFolder(list)
        var expected = listOf(
            listOf("d"),
            listOf("d", "a")
        )
        assertEquals(expected, actual)
    }

}
