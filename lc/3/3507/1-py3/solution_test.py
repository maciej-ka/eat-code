#!/usr/bin/env python3

import unittest
from solution import Solution


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_1(self):
        actual = self.solution.minimumPairRemoval([5,2,3,1])
        expected = 2
        self.assertEqual(expected, actual)


    def test_2(self):
        actual = self.solution.minimumPairRemoval([1,2,2])
        expected = 0
        self.assertEqual(expected, actual)


if __name__ == '__main__':
    unittest.main()
