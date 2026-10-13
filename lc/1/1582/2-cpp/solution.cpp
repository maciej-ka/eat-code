// https://leetcode.com/problems/special-positions-in-a-binary-matrix/submissions/1937707838/?envType=daily-question&envId=2026-03-04

#include <gtest/gtest.h>
#include <vector>

using namespace std;

class Solution {
public:
    int numSpecial(vector<vector<int>>& mat) {
        int ans = 0;

        int rlen = mat.size();
        int clen = mat[0].size();

        auto rowCount = vector<int>(rlen);
        auto colCount = vector<int>(clen);

        for (int i = 0; i < rlen; i++) {
            for (int k = 0; k < clen; k++) {
                if (mat[i][k]) {
                    rowCount[i]++;
                    colCount[k]++;
                }
            }
        }

        for (int i = 0; i < rlen; i++) {
            for (int k = 0; k < clen; k++) {
                if (mat[i][k] && rowCount[i] == 1 && colCount[k] == 1) {
                    ans++;
                    break;
                }
            }
        }

        return ans;
    }
};

TEST(SolutionTest, test1) {
    Solution solution;
    vector<vector<int>> nums = {{1, 0, 0}, {0, 0, 1}, {1, 0, 0}};
    int actual = solution.numSpecial(nums);
    int expected = 1;
    EXPECT_EQ(actual, expected);
}

TEST(SolutionTest, test2) {
    Solution solution;
    vector<vector<int>> nums = {{1, 0, 0}, {0, 1, 0}, {0, 0, 1}};
    int actual = solution.numSpecial(nums);
    int expected = 3;
    EXPECT_EQ(actual, expected);
}

TEST(SolutionTest, test3) {
    Solution solution;
    vector<vector<int>> nums = {{0, 0}, {0, 0}, {1, 0}};
    int actual = solution.numSpecial(nums);
    int expected = 1;
    EXPECT_EQ(actual, expected);
}

TEST(SolutionTest, test4) {
    Solution solution;
    vector<vector<int>> nums = {{0,0,0,0,0,1,0,0},{0,0,0,0,1,0,0,1},{0,0,0,0,1,0,0,0},{1,0,0,0,1,0,0,0},{0,0,1,1,0,0,0,0}};
    int actual = solution.numSpecial(nums);
    int expected = 1;
    EXPECT_EQ(actual, expected);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
