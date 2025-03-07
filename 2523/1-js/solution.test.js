import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution(10, 19);
  expect(actual).toEqual([11,13]);
});

test('case 2', () => {
  const actual = solution(4,5);
  expect(actual).toEqual([-1,-1]);
});

test('case 3', () => {
  const actual = solution(1,5);
  expect(actual).toEqual([1,2]);
});

test('case 4', () => {
  const actual = solution(2,5);
  expect(actual).toEqual([2,3]);
});

test('case 5', () => {
  const actual = solution(84084,407043);
  expect(actual).toEqual([84179,84181]);
});

