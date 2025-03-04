import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution(12);
  expect(actual).toEqual(true);
});

test('case 2', () => {
  const actual = solution(91);
  expect(actual).toEqual(true);
});

test('case 3', () => {
  const actual = solution(21);
  expect(actual).toEqual(false);
});

