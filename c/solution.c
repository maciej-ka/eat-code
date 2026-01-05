#include <stdio.h>
#include <assert.h>
#include <stdlib.h>

int solve(int** nums, int size) {
    return size;
}

void test1() {
    int size = 3;
    int** nums = malloc(sizeof(int*) * size);
    for (int i = 0; i < size; i++) {
        nums[i] = malloc(sizeof(int));
        *nums[i] = i + 1;
    }

    int result = solve(nums, size);
    assert(result == 3);
    printf("✓ test1 passed\n");

    for (int i = 0; i < size; i++) {
        free(nums[i]);
    }
    free(nums);
}

int main() {
    printf("Running tests...\n");
    test1();
    printf("All tests passed!\n");
    return 0;
}
