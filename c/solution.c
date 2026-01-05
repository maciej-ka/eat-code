#include <stdio.h>
#include <assert.h>

int solve(int* nums, int size) {
    return size;
}

void test1() {
    int nums[] = {1, 2, 3};
    int size = sizeof(nums) / sizeof(nums[0]);
    int result = solve(nums, size);
    assert(result == 3);
    printf("✓ test1 passed\n");
}

int main() {
    printf("Running tests...\n");
    test1();
    printf("All tests passed!\n");
    return 0;
}
