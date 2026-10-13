// https://leetcode.com/problems/max-dot-product-of-two-subsequences/submissions/1879214613

#include <gtest/gtest.h>
#include <vector>

using namespace std;

class Solution {
public:
    int maxDotProduct(vector<int>& nums1, vector<int>& nums2) {
        int s1 = nums1.size();
        int s2 = nums2.size();
        vector<vector<int>> dp(s1, vector<int>(s2));

        for (int i = 0; i < s1; i++) {
            for (int j = 0; j < s2; j++) {
                int best = nums1[i] * nums2[j];
                if (i > 0 && j > 0) {
                    int prev = dp[i - 1][j - 1];
                    if (prev > 0) { best += prev; }
                }
                if (i > 0) { best = max(best, dp[i - 1][j]); }
                if (j > 0) { best = max(best, dp[i][j - 1]); }
                dp[i][j] = best;
            }
        }

        return dp[s1 - 1][s2 - 1];
    }
};

TEST(SolutionTest, test1) {
    Solution solution;
    vector<int> nums1 = {2, 1, -2, 5};
    vector<int> nums2 = {3, 0, -6};
    int actual = solution.maxDotProduct(nums1, nums2);
    int expected = 18;
    EXPECT_EQ(actual, expected);
}

TEST(SolutionTest, test2) {
    Solution solution;
    vector<int> nums1 = {3, -2};
    vector<int> nums2 = {2, -6, 7};
    int actual = solution.maxDotProduct(nums1, nums2);
    int expected = 21;
    EXPECT_EQ(actual, expected);
}

TEST(SolutionTest, test3) {
    Solution solution;
    vector<int> nums1 = {-1, -1};
    vector<int> nums2 = {1, 1};
    int actual = solution.maxDotProduct(nums1, nums2);
    int expected = -1;
    EXPECT_EQ(actual, expected);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
