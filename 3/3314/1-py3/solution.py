#!/usr/bin/env python3
# https://leetcode.com/problems/construct-the-minimum-bitwise-array-i/submissions/1890919802/?envType=daily-question&envId=2026-01-20

from typing import List


class Solution:
    def minBitwiseArray(self, nums: List[int]) -> List[int]:
        for i, goal in enumerate(nums):
            nums[i] = -1
            for n in range(goal):
                if n | n + 1 == goal:
                    nums[i] = n
                    break
        return nums
