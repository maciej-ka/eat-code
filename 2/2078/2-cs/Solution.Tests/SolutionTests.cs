namespace Solution.Tests;

public class SolutionTests {

    [Fact]
    public void test1() {
        int actual = new Solution().MaxDistance(new[] { 1, 1, 1, 6, 1, 1, 1 });
        int expected = 3;
        Assert.Equal(expected, actual);
    }

    [Fact]
    public void test2() {
        int actual = new Solution().MaxDistance(new[] { 1, 8, 3, 8, 3 });
        int expected = 4;
        Assert.Equal(expected, actual);
    }

    [Fact]
    public void test3() {
        int actual = new Solution().MaxDistance(new[] { 0, 1 });
        int expected = 1;
        Assert.Equal(expected, actual);
    }

}