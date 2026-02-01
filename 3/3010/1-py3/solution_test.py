#!/usr/bin/env python3

import unittest
from solution import Solution


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_1(self):
        actual = self.solution.minimumCost([1,2,3,12])
        expected = 6
        self.assertEqual(expected, actual)

    def test_2(self):
        actual = self.solution.minimumCost([5,4,3])
        expected = 12
        self.assertEqual(expected, actual)

    def test_3(self):
        actual = self.solution.minimumCost([10,3,1,1])
        expected = 12
        self.assertEqual(expected, actual)

if __name__ == '__main__':
    unittest.main()
