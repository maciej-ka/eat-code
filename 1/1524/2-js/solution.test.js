import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([1,3,5]);
  expect(actual).toEqual(4);
});

test('case 2', () => {
  const actual = solution([2,4,6]);
  expect(actual).toEqual(0);
});

test('case 3', () => {
  const actual = solution([1,2,3,4,5,6,7]);
  expect(actual).toEqual(16);
});

test('case 4', () => {
  const actual = solution([2,4,1,2,4,6,3,2,4,5,2,4,3,2,3,5,2,3]);
  expect(actual).toEqual(90);
});
