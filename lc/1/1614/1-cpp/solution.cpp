// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/?envType=daily-question&envId=2026-09-28
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int maxDepth(string s) {
        int result = 0;
        int current = 0;
        for (int i = 0; i < s.length(); i++) {
            if (s[i] == '(') current++;
            if (s[i] == ')') current--;
            result = max(result, current);
        }
        return result;
    }
};

TEST_CASE("test1") {
    Solution solution;
    int actual = solution.maxDepth("(1+(2*3)+((8)/4))+1");
    int expected = 3;
    CHECK_EQ(actual, expected);
}
