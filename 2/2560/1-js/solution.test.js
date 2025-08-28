import { expect, test } from 'vitest'
import solution from "./solution";

/**
 * miniminum 2 houses will be robbed (totally)
 * find minimum capability to do it
 *
 * a) 2 and 5
 * b) 2 and 9
 * c) 3 and 9
 *
 * a requires lowest capability of 5
 */
test('case 1', () => {
  const actual = solution([2,3,5,9], 2);
  expect(actual).toEqual(5);
});

test('case 2', () => {
  const actual = solution([2,7,9,3,1], 2);
  expect(actual).toEqual(2);
});

test('case 3', () => {
  const actual = solution([2,2,2,3,3], 2);
  expect(actual).toEqual(2);
});
