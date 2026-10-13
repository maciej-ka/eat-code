#!/usr/bin/env python3

import unittest
from solution import Solution


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_1(self):
        actual = self.solution.minPairSum([3,5,2,3])
        expected = 7
        self.assertEqual(expected, actual)

    def test_2(self):
        actual = self.solution.minPairSum([3,5,4,2,4,6])
        expected = 8
        self.assertEqual(expected, actual)

    def test_3(self):
        actual = self.solution.minPairSum([4,1,5,1,2,5,1,5,5,4])
        expected = 8
        self.assertEqual(expected, actual)


if __name__ == '__main__':
    unittest.main()
