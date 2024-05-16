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
  for (int i=0; i<size; i++) {
    cost += abs(permutation[i] - nums[permutation[(i + 1) % size]]);
  }
  return cost;
}

void printArray(int* arr, int size) {
  for (int i=0; i<size; i++) {
    printf("%d", arr[i]);
  }
  puts("");
}

/**
 * Find lexicographically next permutation.
 * Return 0 if argument permutation is last.
 */
int nextPermutation(int* perm, int size) {
  // find first decreasing value
  int a = size - 2;
  for (; a >= 0; a--) {
    if (perm[a] < perm[a + 1]) {
      break;
    }
  }

  // argument permutation was lexicographically last
  if (a < 0) {
    return 0;
  }

  // find next bigger
  int b = a + 1;
  for (int i = a + 1; i < size; i++) {
    if (perm[i] > perm[a] && perm[i] < perm[b]) {
      b = i;
    }
  }

  // swap
  int temp = perm[a];
  perm[a] = perm[b];
  perm[b] = temp;

  // sort ascending
  for (int i = a + 1; i < size; i++) {
    for (int k = i + 1; k < size; k++) {
      /* printf("%d", i); */
      /* printf("%d", k); */
      if (perm[k] < perm[i]) {
        int temp = perm[k];
        perm[k] = perm[i];
        perm[i] = temp;
      }
    }
  }
  return 1;
}

/**
 * Test usage.
 */
int main() {
  /* int nums[] = {1,0,2}; */
  /* int* returnSize = malloc(sizeof(int)); */
  /* int* res = findPermutation(nums, 3, returnSize); */
  /* printArray(res, *returnSize); */

  /* int nums[] = {1,0,2}; */
  /* int perm[] = {0,1,2}; */
  /* printf("%d", calculateCost(nums, perm, 3)); */

  /* int perm[] = {0,2,1}; */
  /* nextPermutation(perm, 3); */
  /* printArray(perm, 3); */

  int nums[] = {0,1,2,3};
  int size = 4;
  printArray(nums, size);
  while (nextPermutation(nums, size)) {
    printArray(nums, size);
  }

}
