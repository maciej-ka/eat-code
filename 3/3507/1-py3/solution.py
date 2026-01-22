#!/usr/bin/env python3
# https://leetcode.com/problems/minimum-pair-removal-to-sort-array-i/submissions/1893586558/?envType=daily-question&envId=2026-01-22

from typing import List


class Solution:
    def minimumPairRemoval(self, nums: List[int]) -> int:
        ans = 0
        while len(nums) > 1:
            sorted = True
            ibest = 0
            best = nums[0] + nums[1]
            for i in range(len(nums) - 1):
                if nums[i] > nums[i + 1]: sorted = False
                sum = nums[i] + nums[i + 1]
                if sum < best:
                    best = sum
                    ibest = i
            if sorted: break
            ans += 1
            nums[ibest] += nums[ibest + 1]
            del nums[ibest + 1]
        return ans
