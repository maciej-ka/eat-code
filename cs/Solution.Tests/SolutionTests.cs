namespace Solution.Tests;

public class SolutionTests
{

    [Fact]
    public void test1()
    {
        var solution = new Solution();
        int[] nums = { 1, 2, 3 };
        int result = solution.Solve(nums);
        Assert.Equal(3, result);
    }

}