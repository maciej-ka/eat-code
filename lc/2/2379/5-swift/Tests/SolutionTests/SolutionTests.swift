import Testing
@testable import Solution

@Test func test1() async throws {
    let solution = Solution()
    let result = solution.minimumRecolors("WBBWWBBWBW", 7)
    #expect(result == 3)
}

@Test func test2() async throws {
    let solution = Solution()
    let result = solution.minimumRecolors("WBWBBBW", 2)
    #expect(result == 0)
}

@Test func test3() async throws {
    let solution = Solution()
    let result = solution.minimumRecolors("BWWWBB", 6)
    #expect(result == 3)
}
