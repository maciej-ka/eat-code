#!/usr/bin/env python3

import unittest
from solution import Solution


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_1(self):
        actual = self.solution.minBitwiseArray([2, 3, 5, 7])
        expected = [-1, 1, 4, 3]
        self.assertEqual(expected, actual)

    def test_2(self):
        actual = self.solution.minBitwiseArray([11, 13, 31])
        expected = [9, 12, 15]
        self.assertEqual(expected, actual)


if __name__ == '__main__':
    unittest.main()
