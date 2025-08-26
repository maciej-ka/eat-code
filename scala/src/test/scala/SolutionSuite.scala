class SolutionSuite extends munit.FunSuite {
    test("test1") {
        val actual = Solution.myfunc(List(1,2,3))
        val expected = 6
        assertEquals(actual, expected)
    }
}
