import { expect, test } from 'vitest'
import solution from "./solution";

/**
 * nums length is odd
 * [3, 3]
 * [2, 2], [2, 2]
 */
test('case 1', () => {
  const actual = solution([3,2,3,2,2,2]);
  expect(actual).toEqual(true);
});

test('case 2', () => {
  const actual = solution([1,2,3,4]);
  expect(actual).toEqual(false);
});

test('case 3', () => {
  const actual = solution([1,1]);
  expect(actual).toEqual(true);
});
