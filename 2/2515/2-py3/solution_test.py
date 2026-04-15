#!/usr/bin/env python3

import unittest
from solution import Solution

class TestSolution(unittest.TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_1(self):
        actual = self.solution.closestTarget(["hello","i","am","leetcode","hello"], "hello", 1)
        expected = 1
        self.assertEqual(expected, actual)

    def test_2(self):
        actual = self.solution.closestTarget(["a","b","leetcode"], "leetcode", 0)
        expected = 1
        self.assertEqual(expected, actual)

    def test_3(self):
        actual = self.solution.closestTarget(["i","eat","leetcode"], "ate", 0)
        expected = -1
        self.assertEqual(expected, actual)

if __name__ == '__main__':
    unittest.main()
