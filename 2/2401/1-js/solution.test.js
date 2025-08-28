import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([1,3,8,48,10]);
  expect(actual).toEqual(3);
});

test('case 2', () => {
  const actual = solution([3,1,5,11,13]);
  expect(actual).toEqual(1);
});

test('case 3', () => {
  const actual = solution([744437702,379056602,145555074,392756761,560864007,934981918,113312475,1090,16384,33,217313281,117883195,978927664]);
  expect(actual).toEqual(3);
});
