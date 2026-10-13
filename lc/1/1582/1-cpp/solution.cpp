// https://leetcode.com/problems/special-positions-in-a-binary-matrix/submissions/1937703727/?envType=daily-question&envId=2026-03-04

#include <gtest/gtest.h>
#include <vector>

using namespace std;

class Solution {
    int rlen;
    int clen;

    bool check(int i, int k, vector<vector<int>>& mat) {
        for (int j = 0; j < rlen; j++) {
            if (j != i && mat[j][k]) {
                return false;
            }
        }
        for (int j = 0; j < clen; j++) {
            if (j != k && mat[i][j]) {
                return false;
            }
        }
        return true;
    }

public:
    int numSpecial(vector<vector<int>>& mat) {
        int ans = 0;
        rlen = mat.size();
        clen = mat[0].size();

        for (int i = 0; i < rlen; i++) {
            for (int k = 0; k < clen; k++) {
                if (!mat[i][k]) continue;
                if (check(i, k, mat)) {
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
