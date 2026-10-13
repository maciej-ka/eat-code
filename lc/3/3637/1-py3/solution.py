#!/usr/bin/env python3
# https://leetcode.com/problems/trionic-array-i/submissions/1906503475/?envType=daily-question&envId=2026-02-02

from typing import List


class Solution:
    def isTrionic(self, nums: List[int]) -> bool:
        i = 0
        p = 0
        q = 0
        r = 0

        while i < len(nums) - 1:
            if nums[i] >= nums[i + 1]:
                p = i
                break
            i += 1

        while i < len(nums) - 1:
            if nums[i] <= nums[i + 1]:
                q = i
                break
            i += 1

        while True:
            if i == len(nums) - 1:
                r = len(nums) - 1
                break
            if nums[i] >= nums[i + 1]:
                r = i
                break
            i += 1

        return 0 < p and p < q and q < r and r == len(nums) - 1
