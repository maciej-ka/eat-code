import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution(10, [[5,7],[1,3],[9,10]]);
  expect(actual).toEqual();
});

test('case 2', () => {
  const actual = solution(5, [[2,4],[1,3]]);
  expect(actual).toEqual();
});

test('case 3', () => {
  const actual = solution(6, [[1,6]]);
  expect(actual).toEqual();
});

test.only('case 4', () => {
  const actual = solution(20, [[1,2], [4,5], [7,8], [10,11], [13,14], [16,17], [19,20]]);
  expect(actual).toEqual();
});

