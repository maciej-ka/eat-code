import Testing
@testable import Solution

@Test func test1() async throws {
    let solution = Solution()
    let actual = solution.solve([1, 2, 3])
    let expected = 3
    #expect(actual == expected)
}
