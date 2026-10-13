#!/usr/bin/env python3
# https://leetcode.com/problems/transformed-array/submissions/1909602954/?envType=daily-question&envId=2026-02-05
from typing import List


class Solution:
    def constructTransformedArray(self, nums: List[int]) -> List[int]:
        return [nums[(i + n) % len(nums)] for i, n in enumerate(nums)]
