import { expect, test } from "vitest";
import solution from "./solution";

test("case 1", () => {
  const actual = solution([5,2,5,4,5], 2);
  expect(actual).toEqual(2);
});

test("case 2", () => {
  const actual = solution([2,1,2], 2);
  expect(actual).toEqual(-1);
});

test("case 3", () => {
  const actual = solution([9,7,5,3], 1);
  expect(actual).toEqual(4);
});
