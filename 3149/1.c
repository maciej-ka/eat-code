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

int calculateCost(int* nums, int* permutation, int size) {
  int cost = 0;
  for (int i=0;i<size;i++) {
    cost += abs(permutation[i] - nums[permutation[(i + 1) % size]]);
  }
  return cost;
}

void printArray(int* arr, int size) {
  for (int i=0;i<size;i++) {
    printf("%d", arr[i]);
  }
  puts("");
}

/**
 * Test usage.
 */
int main() {
  /* int nums[] = {1,0,2}; */
  /* int* returnSize = malloc(sizeof(int)); */
  /* int* res = findPermutation(nums, 3, returnSize); */
  /* printArray(res, *returnSize); */
  int nums[] = {1,0,2};
  int perm[] = {0,1,2};
  printf("%d", calculateCost(nums, perm, 3));
}
