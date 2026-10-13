#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int solve(vector<int>& nums) {
        return nums.size();
    }
};

TEST_CASE("test1") {
    Solution solution;
    vector<int> nums = {1, 2, 3};
    int actual = solution.solve(nums);
    int expected = 3;
    CHECK_EQ(actual, expected);
}
