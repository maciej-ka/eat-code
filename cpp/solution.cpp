#include <gtest/gtest.h>
#include <vector>

class Solution {
public:
    int solve(std::vector<int> nums) {
        return nums.size();
    }
};

TEST(SolutionTest, test1) {
    Solution solution;
    std::vector<int> nums = {1, 2, 3};
    EXPECT_EQ(solution.solve(nums), 3);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
