class SolutionSuite extends munit.FunSuite {

  test("test 1") {
    val actual = Solution.solve(Array(1, 2, 3))
    val expected = 3
    assertEquals(actual, expected)
  }

}
