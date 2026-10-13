#include "unity/unity.h"

int solve(int* nums, int size) {
    (void)nums;
    return size;
}

void test1(void) {
    int actual = solve((int[]){1, 2, 3}, 3);
    int expected = 3;
    TEST_ASSERT_EQUAL_INT(expected, actual);
}

void setUp(void) {}
void tearDown(void) {}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test1);
    return UNITY_END();
}

