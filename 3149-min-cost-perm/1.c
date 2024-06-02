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
}

void printArrayLn(int* arr, int size) {
  printArray(arr, size);
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
    if (perm[i] > perm[a] && perm[i] <= perm[b]) {
      b = i;
    }
  }

  // swap
  int temp = perm[a];
  perm[a] = perm[b];
  perm[b] = temp;

  // sort tail but only by reversing
  // works because: from first step we know numbers are ordered
  // and swap was done between two numbers so close to each other
  // that it doesn't ruin that ordering
  for (int i = 1; i <= (size - a) / 2; i++) {
    int temp = perm[size - i];
    perm[size - i] = perm[a + i];
    perm[a + i] = temp;
  }
  return 1;
}

void setFirstPerm(int* perm, int size) {
  for(int i=0;i<size;i++) {
    perm[i] = i;
  }
}

/**
 * Test usage.
 */
int main() {
  /* int nums[] = {1,0,2}; */
  /* int* returnSize = malloc(sizeof(int)); */
  /* int* res = findPermutation(nums, 3, returnSize); */
  /* printArrayLn(res, *returnSize); */

  /* int nums[] = {1,0,2}; */
  /* int perm[] = {0,1,2}; */
  /* printf("%d", calculateCost(nums, perm, 3)); */

  /* int perm[] = {0,2,1}; */
  /* nextPermutation(perm, 3); */
  /* printArrayLn(perm, 3); */

  /* int nums[] = {0,1,2,3,4}; */
  /* int size = 5; */
  /* printArrayLn(nums, size); */
  /* while (nextPermutation(nums, size)) { */
  /*   printArrayLn(nums, size); */
  /* } */

  // scan whole space for research
  int size = 5;
  int nums[] = {0,1,2,3,4};
  int perm[] = {0,1,2,3,4};
  int cost = 0;

  do {
    setFirstPerm(perm, size);
    puts("");
    printArrayLn(nums, size);
    puts("---------------");
    do {
      printArray(perm, size);
      cost = calculateCost(nums, perm, size);
      printf(" ");
      printf("%d", cost);
      puts("");
    } while (nextPermutation(perm, size));
  } while (nextPermutation(nums, size));
}
