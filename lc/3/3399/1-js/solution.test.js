import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution("000001", 1);
  expect(actual).toEqual(2);
});

test('case 2', () => {
  const actual = solution("0000", 2);
  expect(actual).toEqual(1);
});

test('case 3', () => {
  const actual = solution("0101", 0);
  expect(actual).toEqual(1);
});

test('case 4', () => {
  const actual = solution("011", 0);
  expect(actual).toEqual(2);
});

test('case 5', () => {
  const actual = solution("00", 2);
  expect(actual).toEqual(1);
});

test('case 6', () => {
  const actual = solution("000", 0);
  expect(actual).toEqual(3);
});
