// https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/submissions/1937097216/?envType=daily-question&envId=2026-03-03

#include <gtest/gtest.h>

using namespace std;

class Solution {
    int solve(int n, int k) {
        if (n == 1) return 0;
        int last = (1 << n) - 2;
        int half = last >> 1;

        if (k == half) return 1;
        if (k < half) return solve(n - 1, k);
        return solve(n - 1, last - k) ? 0 : 1;
    }

public:
    char findKthBit(int n, int k) {
        return solve(n, k - 1) == 0 ? '0' : '1';
    }
};

TEST(SolutionTest, test1) {
    Solution solution;
    char actual = solution.findKthBit(3, 1);
    char expected = '0';
    EXPECT_EQ(actual, expected);
}

TEST(SolutionTest, test2) {
    Solution solution;
    char actual = solution.findKthBit(4, 11);
    char expected = '1';
    EXPECT_EQ(actual, expected);
}

TEST(SolutionTest, test3) {
    Solution solution;
    char actual = solution.findKthBit(3, 5);
    char expected = '0';
    EXPECT_EQ(actual, expected);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
