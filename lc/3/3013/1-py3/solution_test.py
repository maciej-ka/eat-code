#!/usr/bin/env python3

import unittest
from solution import Solution


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_1(self):
        actual = self.solution.minimumCost([1,3,2,6,4,2], 3, 3)
        expected = 5
        self.assertEqual(expected, actual)

    def test_2(self):
        actual = self.solution.minimumCost([10,1,2,2,2,1], 4, 3)
        expected = 15
        self.assertEqual(expected, actual)

    def test_3(self):
        actual = self.solution.minimumCost([10,8,18,9], 3, 1)
        expected = 36
        self.assertEqual(expected, actual)

if __name__ == '__main__':
    unittest.main()
