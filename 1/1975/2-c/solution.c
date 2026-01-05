// https://leetcode.com/problems/maximum-matrix-sum/submissions/1875730528/

#include "unity/unity.h"
#include <limits.h>
#include <stdbool.h>
#include <stdlib.h>

long long maxMatrixSum(int** matrix, int matrixSize, int* matrixColSize) {
    (void) matrixColSize;
    long long sum = 0;
    int min = INT_MAX;
    bool even = true;

    for (int i = 0; i < matrixSize; i++) {
        for (int j = 0; j < matrixSize; j++) {
            int val = matrix[i][j];
            if (val < 0) { even = !even; }
            int absval = abs(val);
            sum += absval;
            if (absval < min) { min = absval; }
        }
    }

    return even
        ? sum
        : sum - (min << 1);
}

void test1(void) {
    int row1[] = {1, -1};
    int row2[] = {1, -1};
    long long actual = maxMatrixSum((int*[]){ row1, row2 }, 2, (int[]){2, 2});
    int expected = 4;
    TEST_ASSERT_EQUAL_INT(expected, actual);
}

void test2(void) {
    int row1[] = {1, 2, 3};
    int row2[] = {-1, -2, -3};
    int row3[] = {1, 2, 3};
    long long actual = maxMatrixSum((int*[]){ row1, row2, row3 }, 3, (int[]){3, 3, 3});
    int expected = 16;
    TEST_ASSERT_EQUAL_INT(expected, actual);
}

void setUp(void) {}
void tearDown(void) {}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test1);
    RUN_TEST(test2);
    return UNITY_END();
}

