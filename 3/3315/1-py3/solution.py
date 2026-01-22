#!/usr/bin/env python3
# https://leetcode.com/problems/construct-the-minimum-bitwise-array-ii/submissions/1892296879/?envType=daily-question&envId=2026-01-21

from typing import List


class Solution:
    def minBitwiseArray(self, nums: List[int]) -> List[int]:
        ans = [-1] * len(nums)
        for i, num in enumerate(nums):
            if num % 2:
                # b is set after initial 111 train
                b = (num + 1) & -(num + 1)
                ans[i] = num - (b >> 1)
        return ans
