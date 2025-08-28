import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([[2,4],[6,8]], 2);
  expect(actual).toEqual(4);
});

test('case 2', () => {
  const actual = solution([[1,5],[2,3]], 1);
  expect(actual).toEqual(5);
});

test('case 3', () => {
  const actual = solution([[1,2],[3,4]], 2);
  expect(actual).toEqual(-1);
});

test('case 4', () => {
  const actual = solution([[4,9],[9,24]], 5);
  expect(actual).toEqual(4);
});

test('case 5', () => {
  const actual = solution([[931,128],[639,712]], 73);
  expect(actual).toEqual(12);
});

test('case 6', () => {
  const actual = solution([[980,476,644,56],[644,140,812,308],[812,812,896,560],[728,476,56,812]], 84);
  expect(actual).toEqual(45);
});

