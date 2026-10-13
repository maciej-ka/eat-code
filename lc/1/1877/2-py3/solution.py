#!/usr/bin/env python3
# https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/submissions/1895588137/?envType=daily-question&envId=2026-01-24

from typing import List


class Solution(object):
    def minPairSum(self, nums: List[int]) -> int:
        nums = sorted(nums)
        nlen = len(nums)
        best = nums[0] + nums[nlen - 1]
        for i in range(nlen // 2):
            best = max(best, nums[i] + nums[nlen - 1 - i])
        return best
