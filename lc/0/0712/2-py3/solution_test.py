#!/usr/bin/env python3

import unittest
from solution import Solution


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_1(self):
        actual = self.solution.minimumDeleteSum("sea", "eat")
        expected = 231
        self.assertEqual(expected, actual)

    def test_2(self):
        actual = self.solution.minimumDeleteSum("delete", "leet")
        expected = 403
        self.assertEqual(expected, actual)

    def test_3(self):
        actual = self.solution.minimumDeleteSum("ccaccjp", "fwosarcwge")
        expected = 1399
        self.assertEqual(expected, actual)


if __name__ == '__main__':
    unittest.main()
