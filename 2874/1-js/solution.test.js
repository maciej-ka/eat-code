import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([12,6,1,2,7]);
  expect(actual).toEqual(77);
});

test('case 2', () => {
  const actual = solution([1,10,3,4,19]);
  expect(actual).toEqual(133);
});

test('case 3', () => {
  const actual = solution([1,2,3]);
  expect(actual).toEqual(0);
});

test('case 4', () => {
  const actual = solution([1,10,19,3,4]);
  expect(actual).toEqual(64);
});
