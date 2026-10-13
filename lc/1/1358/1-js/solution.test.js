import { expect, test, vitest } from 'vitest'
import solution from "./solution";

/*
 * ending on 3th:
 * abc
 *
 * ending on 4th
 * abca
 *  bca
 *
 * abcab
 *  bcab
 *   cab
 *
 * abcabc
 *  bcabc
 *   cabc
 *    abc
 */
test('case 1', () => {
  const actual = solution("abcabc");
  expect(actual).toEqual(10);
});

/**
 * aaacb
 *  aacb
 *   acb
 */
test('case 2', () => {
  const actual = solution("aaacb");
  expect(actual).toEqual(3);
});

test('case 3', () => {
  const actual = solution("abc");
  expect(actual).toEqual(1);
});

test('case 4', () => {
  const actual = solution("aaa");
  expect(actual).toEqual(0);
});
