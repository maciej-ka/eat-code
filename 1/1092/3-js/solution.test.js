import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution("abac", "cab");
  expect(actual).toEqual("cabac");
});

test('case 2', () => {
  const actual = solution("aaaaaaaa", "aaaaaaaa");
  expect(actual).toEqual("aaaaaaaa");
});

test('case 3', () => {
  const actual = solution("abab", "acac");
  expect(actual).toEqual("acbacb");
});

