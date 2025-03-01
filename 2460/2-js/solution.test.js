import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution([1,2,2,1,1,0]);
  expect(actual).toEqual([1,4,2,0,0,0]);
});

test('case 2', () => {
  const actual = solution([0,1]);
  expect(actual).toEqual([1,0]);
});

test('case 3', () => {
  const actual = solution([847,847,0,0,0,399,416,416,879,879,206,206,206,272]);
  expect(actual).toEqual([1694,399,832,1758,412,206,272,0,0,0,0,0,0,0]);
});

