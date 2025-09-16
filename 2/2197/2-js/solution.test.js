import { expect, test } from "vitest";
import replaceNonCoprimes from "./solution";

test("test 1", () => {
  const actual = replaceNonCoprimes([6,4,3,2,7,6,2]);
  const expected = [12,7,6];
  expect(actual).toEqual(expected);
})

test("test 2", () => {
  const actual = replaceNonCoprimes([2,2,1,1,3,3,3]);
  const expected = [2,1,1,3];
  expect(actual).toEqual(expected);
})

test("test 3", () => {
  const actual = replaceNonCoprimes([287,41,49,287,899,23,23,20677,5,825]);
  const expected = [2009,20677,825];
  expect(actual).toEqual(expected);
})

