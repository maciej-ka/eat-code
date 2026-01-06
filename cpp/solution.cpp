#include <gtest/gtest.h>
#include <vector>

using std::vector;

class Solution {
public:
    int solve(vector<int>& nums) {
        return nums.size();
    }
};

TEST(SolutionTest, test1) {
    Solution solution;
    vector<int> nums = {1, 2, 3};
    int actual = solution.solve(nums);
    int expected = 3;
    EXPECT_EQ(actual, expected);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
