#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import unittest
from solution import Solution


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_1(self):
        actual = self.solution.solve([1, 2, 3])
        expected = 3
        self.assertEqual(expected, actual)


if __name__ == '__main__':
    unittest.main()
