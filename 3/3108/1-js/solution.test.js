import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution(5, [[0,1,7],[1,3,7],[1,2,1]], [[0,3],[3,4]]);
  expect(actual).toEqual([1, -1]);
});

test('case 2', () => {
  const actual = solution(3, [[0,2,7],[0,1,15],[1,2,6],[1,2,1]], [[1,2]]);
  expect(actual).toEqual([0]);
});

test('case 3', () => {
  const actual = solution(8, [[3,6,6],[6,1,0],[1,3,1]], [[5,4],[7,3]]);
  expect(actual).toEqual([-1, -1]);
});

test('case 4', () => {
  const actual = solution(7, [[6,0,0],[4,1,1],[5,4,0],[1,2,2],[3,0,2],[2,0,1],[0,4,2],[1,6,1],[1,3,1],[3,0,1]], [[5,4],[0,5],[4,0],[1,5],[1,3],[1,5]]);
  expect(actual).toEqual([0,0,0,0,0,0]);
});

