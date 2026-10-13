#!/usr/bin/env python3
# https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/submissions/1880757331/?envType=daily-question&envId=2026-01-10


class Solution(object):
    def minimumDeleteSum(self, s1: str, s2: str) -> int:
        len1, len2 = len(s1), len(s2)
        dp = [[0] * (len2 + 1) for _ in range(len1 + 1)]

        for i in range(len1):
            dp[i + 1][0] = dp[i][0] + ord(s1[i])

        for j in range(len2):
            dp[0][j + 1] = dp[0][j] + ord(s2[j])

        for i in range(len1):
            for j in range(len2):
                if s1[i] == s2[j]:
                    dp[i + 1][j + 1] = dp[i][j]
                else:
                    dp[i + 1][j + 1] = min(
                            ord(s1[i]) + dp[i][j + 1],
                            ord(s2[j]) + dp[i + 1][j])

        return dp[len1][len2]
