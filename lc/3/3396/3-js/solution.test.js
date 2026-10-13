import { expect, test } from "vitest";
import solution from "./solution";

test("case 1", () => {
  const actual = solution([1,2,3,4,2,3,3,5,7]);
  expect(actual).toEqual(2);
});

test("case 2", () => {
  const actual = solution([4,5,6,4,4]);
  expect(actual).toEqual(2);
});

test("case 3", () => {
  const actual = solution([6,7,8,9]);
  expect(actual).toEqual(0);
});
