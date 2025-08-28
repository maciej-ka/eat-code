void nextPermutation(int* nums, int numsSize) {
  int i;
  int temp;

  // find first decreasing value
  int a = numsSize - 2;
  for (; a >= 0; a--) {
    if (nums[a] < nums[a + 1]) {
      break;
    }
  }

  // if -1 then permutation was lexicographically last
  if (a >= 0) {

    // inside tail find value next to nums[a]
    int b = a + 1;
    for (i = a + 1; i < numsSize; i++) {
      if (nums[i] > nums[a] && nums[i] <= nums[b]) {
        b = i;
      }
    }

    // swap
    temp = nums[a];
    nums[a] = nums[b];
    nums[b] = temp;
  }

  // sort tail ... but only by reversing it
  // works because from first step we know numbers are ordered
  // and swaped numbers were so close in value
  // that swap didn't ruin ordering
  for (i = 1; i <= (numsSize - a) / 2; i++) {
    temp = nums[numsSize - i];
    nums[numsSize - i] = nums[a + i];
    nums[a + i] = temp;
  }
}
