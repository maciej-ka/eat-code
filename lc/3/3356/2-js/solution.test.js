import { expect, test } from 'vitest'
import solution from "./solution";

/**
 * 2 0 2
 *
 * q 0 2 1
 * decrement both by 1
 *
 * 1 0 1
 *
 * q 0 2 1
 * decrement both by 1
 *
 * 0 0 0
 * return 2
 */
test('case 1', () => {
  const actual = solution([2,0,2], [[0,2,1],[0,2,1],[1,1,3]]);
  expect(actual).toEqual(2);
});

/**
 * 4 3 2 1
 *
 * q 1 3 2
 * 4 1 0 0
 *
 * q 0 2 1
 * 3 0 0 0
 *
 * not possible
 * return -1
 */
test('case 2', () => {
  const actual = solution([4,3,2,1], [[1,3,2],[0,2,1]]);
  expect(actual).toEqual(-1);
});

test('case 3', () => {
  const actual = solution([0], [[0,0,2],[0,0,4],[0,0,4],[0,0,3],[0,0,5]]);
  expect(actual).toEqual(0);
});
