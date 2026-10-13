import { expect, test } from "vitest";
import solution from "./solution";

test("case 1", () => {
  const actual = solution([3,0,1,1,9,7], 7, 2, 3);
  expect(actual).toEqual(4);
});

test("case 2", () => {
  const actual = solution([1,1,2,2,3], 0, 0, 1);
  expect(actual).toEqual(0);
});
