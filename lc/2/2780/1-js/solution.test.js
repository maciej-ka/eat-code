import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([1,2,2,2]);
  expect(actual).toEqual(2);
});

test('case 2', () => {
  const actual = solution([2,1,3,1,1,1,7,1,2,1]);
  expect(actual).toEqual(4);
});

test('case 3', () => {
  const actual = solution([3,3,3,3,7,2,2]);
  expect(actual).toEqual(-1);
});

test('case 4', () => {
  const actual = solution([3,3,3,7,2,2]);
  expect(actual).toEqual(-1);
});
