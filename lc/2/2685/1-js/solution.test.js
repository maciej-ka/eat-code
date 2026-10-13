import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution(6, [[0,1],[0,2],[1,2],[3,4]]);
  expect(actual).toEqual(3);
});

test('case 2', () => {
  const actual = solution(6, [[0,1],[0,2],[1,2],[3,4],[3,5]]);
  expect(actual).toEqual(1);
});
