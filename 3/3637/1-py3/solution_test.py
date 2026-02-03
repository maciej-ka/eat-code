#!/usr/bin/env python3

import unittest
from solution import Solution


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_1(self):
        actual = self.solution.isTrionic([1,3,5,4,2,6])
        expected = True
        self.assertEqual(expected, actual)

    def test_2(self):
        actual = self.solution.isTrionic([2,1,3])
        expected = False
        self.assertEqual(expected, actual)

if __name__ == '__main__':
    unittest.main()
