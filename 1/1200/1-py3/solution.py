#!/usr/bin/env python3
# https://leetcode.com/problems/minimum-absolute-difference/submissions/1898117775/?envType=daily-question&envId=2026-01-26

from typing import List


class Solution:
    def minimumAbsDifference(self, arr: List[int]) -> List[List[int]]:
        arr = sorted(arr)
        min = arr[1] - arr[0]
        res = []
        for i in range(len(arr) - 1):
            diff = arr[i + 1] - arr[i]
            if diff < min:
                min = diff
                res = []
            if diff == min:
                res.append([arr[i], arr[i + 1]])
        return res
