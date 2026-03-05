// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/submissions/1938546107/?envType=daily-question&envId=2026-03-05

#include <gtest/gtest.h>
#include <vector>

using namespace std;

class Solution {
public:
    int minOperations(string s) {
        int ans[] = {0, 0};
        for (int i = 0; i < s.length(); i++) {
            int is_odd = i % 2;
            int is_zero = s[i] == '0';
            ans[(is_odd + is_zero) % 2]++;
        }
        return ans[0] < ans[1] ? ans[0] : ans[1];
    }
};

TEST(SolutionTest, test1) {
    Solution solution;
    int actual = solution.minOperations("0100");
    int expected = 1;
    EXPECT_EQ(actual, expected);
}

TEST(SolutionTest, test2) {
    Solution solution;
    int actual = solution.minOperations("10");
    int expected = 0;
    EXPECT_EQ(actual, expected);
}

TEST(SolutionTest, test3) {
    Solution solution;
    int actual = solution.minOperations("1111");
    int expected = 2;
    EXPECT_EQ(actual, expected);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
