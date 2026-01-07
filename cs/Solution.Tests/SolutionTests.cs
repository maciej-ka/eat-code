namespace Solution.Tests;

public class SolutionTests {

    [Fact]
    public void test1() {
        int actual = new Solution().Solve(new[] { 1, 2, 3 });
        int expected = 3;
        Assert.Equal(expected, actual);
    }

}