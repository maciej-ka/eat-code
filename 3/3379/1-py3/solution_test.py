#!/usr/bin/env python3

import unittest
from solution import Solution


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_1(self):
        actual = self.solution.constructTransformedArray([3,-2,1,1])
        expected = [1,1,1,3]
        self.assertEqual(expected, actual)

    def test_2(self):
        actual = self.solution.constructTransformedArray([-1,4,-1])
        expected = [-1,-1,4]
        self.assertEqual(expected, actual)

if __name__ == '__main__':
    unittest.main()
