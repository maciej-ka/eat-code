class SolutionSuite extends munit.FunSuite {

  test("test 1") {
    val actual = Solution.minTimeToVisitAllPoints(Array(Array(1,1),Array(3,4),Array(-1,0)))
    val expected = 7
    assertEquals(actual, expected)
  }

  test("test 2") {
    val actual = Solution.minTimeToVisitAllPoints(Array(Array(3,2),Array(-2,2)))
    val expected = 5
    assertEquals(actual, expected)
  }

}
