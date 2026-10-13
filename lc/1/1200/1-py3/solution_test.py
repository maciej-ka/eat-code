#!/usr/bin/env python3

import unittest
from solution import Solution


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_1(self):
        actual = self.solution.minimumAbsDifference([4,2,1,3])
        expected = [[1,2],[2,3],[3,4]]
        self.assertEqual(expected, actual)

    def test_2(self):
        actual = self.solution.minimumAbsDifference([1,3,6,10,15])
        expected = [[1,3]]
        self.assertEqual(expected, actual)

    def test_3(self):
        actual = self.solution.minimumAbsDifference([3,8,-10,23,19,-4,-14,27])
        expected = [[-14,-10],[19,23],[23,27]]
        self.assertEqual(expected, actual)

if __name__ == '__main__':
    unittest.main()
