import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution(10, [[5,7],[1,3],[9,10]]);
  expect(actual).toEqual(2);
});

test('case 2', () => {
  const actual = solution(5, [[2,4],[1,3]]);
  expect(actual).toEqual(1);
});

test('case 3', () => {
  const actual = solution(6, [[1,6]]);
  expect(actual).toEqual(0);
});

test('case 4', () => {
  const actual = solution(20, [[1,2], [4,5], [7,8], [10,11], [13,14], [16,17], [19,20]]);
  expect(actual).toEqual(6);
});

test('case 5', () => {
  const actual = solution(33, [[3,9],[7,16],[21,23],[22,33],[5,13],[10,23],[1,15]]);
  expect(actual).toEqual(0);
});

test('case 6', () => {
  const actual = solution(16, [[11,14],[5,7],[3,10],[14,14],[5,13],[6,8]]);
  expect(actual).toEqual(4);
});

