import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([0,1,1,1,0,0]);
  expect(actual).toEqual(3);
});

test('case 2', () => {
  const actual = solution([0,1,1,1]);
  expect(actual).toEqual(-1);
});

