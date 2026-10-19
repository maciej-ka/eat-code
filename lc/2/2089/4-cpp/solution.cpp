// https://leetcode.com/problems/find-target-indices-after-sorting-array/submissions/2176117237/?envType=daily-question&envId=2026-10-19
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> targetIndices(vector<int>& nums, int target) {
        int count = 0;
        int smaller = 0;
        int len = nums.size();
        for (int i = 0; i < len; i++)
            if (nums[i] < target)
                smaller++;
            else if (nums[i] == target)
                count++;

        vector<int> res = {};
        for (int i = 0; i < count; i++)
            res.push_back(smaller + i);

        return res;
    }
};

TEST_CASE("test1") {
    Solution solution;
    vector nums = {1, 2, 5, 2, 3};
    vector actual = solution.targetIndices(nums, 2);
    vector expected = {1, 2};
    CHECK_EQ(actual, expected);
}

TEST_CASE("test2") {
    Solution solution;
    vector nums = {1, 2, 5, 2, 3};
    vector actual = solution.targetIndices(nums, 3);
    vector expected = {3};
    CHECK_EQ(actual, expected);
}

TEST_CASE("test3") {
    Solution solution;
    vector nums = {1, 2, 5, 2, 3};
    vector actual = solution.targetIndices(nums, 5);
    vector expected = {4};
    CHECK_EQ(actual, expected);
}

TEST_CASE("test4") {
    Solution solution;
    vector nums = {1, 2, 5, 2, 3};
    vector actual = solution.targetIndices(nums, 6);
    vector<int> expected = {};
    CHECK_EQ(actual, expected);
}
