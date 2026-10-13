module Solution where

import Test.HUnit
import System.Exit (exitFailure, exitSuccess)

solve :: [Int] -> Int
solve nums = length nums

-- Unit Tests
tests :: Test
tests = TestList
    [ "test 1" ~: solve [1, 2, 3] ~?= 3
    ]

-- Main function to run tests
main :: IO ()
main = do
    counts <- runTestTT tests
    if failures counts > 0 || errors counts > 0
        then exitFailure
        else exitSuccess
