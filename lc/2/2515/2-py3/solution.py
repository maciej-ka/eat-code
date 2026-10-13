#!/usr/bin/env python3
# https://leetcode.com/problems/shortest-distance-to-target-string-in-a-circular-array/?envType=daily-question&envId=2026-04-15

from typing import List


class Solution:
    def closestTarget(self, words: List[str], target: str, startIndex: int) -> int:
        n = len(words)
        for i in range((n >> 1) + 1):
            i1 = (startIndex + i) % n
            i2 = (n + startIndex - i) % n
            if words[i1] == target or words[i2] == target:
                return i
        return -1
