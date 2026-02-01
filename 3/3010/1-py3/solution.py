#!/usr/bin/env python3
# https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-i/submissions/1903698203/?envType=daily-question&envId=2026-01-31

from typing import List


class Solution:
    def minimumCost(self, nums: List[int]) -> int:
        num1 = min(nums[1], nums[2])
        num2 = max(nums[1], nums[2])
        for n in nums[3:]:
            if n < num1:
                num2 = num1
                num1 = n
            elif n < num2:
                num2 = n
        return nums[0] + num1 + num2
