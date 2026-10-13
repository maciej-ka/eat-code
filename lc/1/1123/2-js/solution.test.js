import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([3,5,1,6,2,0,8,null,null,7,4]);
  expect(actual).toEqual([2,7,4]);
});

test('case 2', () => {
  const actual = solution([1]);
  expect(actual).toEqual([1]);
});

test('case 3', () => {
  const actual = solution([0,1,3,null,2]);
  expect(actual).toEqual([2]);
});
