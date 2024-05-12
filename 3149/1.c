#include <stdlib.h>
#include <stdio.h>

/**
 * My first ever malloc usage.
 * The returned array must be malloced, assume caller calls free().
 */
int* findPermutation(int* nums, int numsSize, int* returnSize) {
  int* res = malloc(numsSize * sizeof(int));
  *returnSize = numsSize;
  res[0] = 5;
  res[1] = 6;
  res[2] = 7;
  return res;
}

/**
 * Test usage.
 */
int main() {
  int nums[] = {1,0,2};
  int* returnSize = malloc(sizeof(int));
  int* res = findPermutation(nums, 3, returnSize);

  for (int i=0;i<*returnSize;i++) {
    printf("%d", res[i]);
  }
  puts("");
}
