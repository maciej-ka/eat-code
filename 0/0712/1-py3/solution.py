#!/usr/bin/env python3
# https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/submissions/1880716461/?envType=daily-question&envId=2026-01-10
import math


class Solution(object):
    def minimumDeleteSum(self, s1: str, s2: str) -> int:
        len1 = len(s1)
        len2 = len(s2)
        dp = [[0 for j in range(len2 + 1)] for i in range(len1 + 1)]
        for i in range(len1 + 1):
            for j in range(len2 + 1):
                best = math.inf
                if i == 0 and j == 0:
                    best = 0
                if i > 0 and j > 0 and s1[i - 1] == s2[j - 1]:
                    best = dp[i - 1][j - 1]
                if i > 0:
                    best = min(best, ord(s1[i - 1]) + dp[i - 1][j])
                if j > 0:
                    best = min(best, ord(s2[j - 1]) + dp[i][j - 1])
                dp[i][j] = best
        return dp[len1][len2]
