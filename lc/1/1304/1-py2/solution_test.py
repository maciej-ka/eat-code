#!/usr/bin/env python2
# -*- coding: utf-8 -*-

import unittest
from solution import Solution


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_1(self):
        actual = self.solution.sumZero(1)
        expected = [0]
        self.assertEqual(expected, actual)

    def test_2(self):
        actual = self.solution.sumZero(2)
        expected = [-1, 1]
        self.assertEqual(expected, actual)

    def test_3(self):
        actual = self.solution.sumZero(3)
        expected = [-1, 1, 0]
        self.assertEqual(expected, actual)


if __name__ == '__main__':
    unittest.main()
