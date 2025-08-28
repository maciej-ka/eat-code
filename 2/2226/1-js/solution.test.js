import { expect, test } from 'vitest'
import solution from "./solution";

/**
 * 5 8 6
 * 5 5 3 5 1
 */
test('case 1', () => {
  const actual = solution([5,8,6], 3);
  expect(actual).toEqual(5);
});

/**
 * more candies than children
 */
test('case 2', () => {
  const actual = solution([2,5], 11);
  expect(actual).toEqual(0);
});

/**
 * 5 8 6
 * (4 1) (4 4) (4 2) four not possible
 * (3 2) (3 3 2) (3 3) three possible
 */
test('case 3', () => {
  const actual = solution([5,8,6], 5);
  expect(actual).toEqual(3);
});
