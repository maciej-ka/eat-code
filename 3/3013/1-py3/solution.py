#!/usr/bin/env python3
# https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-ii/submissions/1906038988/?envType=daily-question&envId=2026-02-02

from typing import List
from heapq import *


class Solution:
    def minimumCost(self, nums: List[int], k: int, dist: int) -> int:
        sum = nums[0]
        # max heap
        top = []
        topset = set()
        # min heap
        rest = []

        init = sorted([(nums[i], i) for i in range(1, dist + 2)])

        for n in init[:k-1]:
            heappush_max(top, n)
            topset.add(n[1])
            sum += n[0]
        best = sum

        for n in init[k-1:]:
            heappush(rest, n)

        for i in range(2, len(nums) - dist):
            # leave
            if i-1 in topset:
                topset.remove(i-1)
                sum -= nums[i-1]

            # enter
            heappush(rest, (nums[i+dist], i+dist))

            # rebalance and clean
            while True:
                while top[0][1] < i:
                    heappop_max(top)

                if len(rest) == 0: break
                while rest[0][1] < i:
                    heappop(rest)

                if len(topset) == k-1 and top[0][0] <= rest[0][0]:
                    break

                # remove from top if worse
                if top[0][0] > rest[0][0]:
                    n = heappop_max(top)
                    sum -= n[0]
                    topset.remove(n[1])
                    heappush(rest, n)
                # add from rest
                n = heappop(rest)
                heappush_max(top, n)
                topset.add(n[1])
                sum += n[0]

            best = min(sum, best)
        return best
