#!/usr/bin/env python3
# wrong answer

from typing import List


class Solution(object):
    def minPairSum(self, nums: List[int]) -> int:
        min_num = nums[0]
        max_num = nums[0]
        for num in nums:
            min_num = min(min_num, num)
            max_num = max(max_num, num)
        return min_num + max_num
