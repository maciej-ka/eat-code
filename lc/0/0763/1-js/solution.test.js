import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution("ababcbacadefegdehijhklij");
  expect(actual).toEqual([9,7,8]);
});

test('case 2', () => {
  const actual = solution("eccbbbbdec");
  expect(actual).toEqual([10]);
});

test('case 3', () => {
  const actual = solution("bbvemgjwruuwalp");
  expect(actual).toEqual([2,1,1,1,1,1,5,1,1,1]);
});
