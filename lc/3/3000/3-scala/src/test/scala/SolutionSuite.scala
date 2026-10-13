class SolutionSuite extends munit.FunSuite {
    test("test1") {
        val actual = Solution.areaOfMaxDiagonal(Array(Array(9,3), Array(8,6)))
        val expected = 48
        assertEquals(actual, expected)
    }

    test("test2") {
        val actual = Solution.areaOfMaxDiagonal(Array(Array(3,4), Array(4,3)))
        val expected = 12
        assertEquals(actual, expected)
    }
}
