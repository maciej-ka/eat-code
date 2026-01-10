#!/usr/bin/env python2
# -*- coding: utf-8 -*-
# https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/submissions/1762498102/?envType=daily-question&envId=2025-09-07


class Solution(object):
    def sumZero(self, n):
        """
        :type n: int
        :rtype: List[int]
        """
        result = []
        for i in range(0, n / 2):
            result.append(-i - 1)
            result.append(i + 1)
        if n % 2:
            result.append(0)
        return result
