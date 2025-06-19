import { expect, test } from "vitest";
import solution from "./solution";

// 0,1,3
// 2... not possible
test("case 1", () => {
  const actual = solution([2,0,1,3], [0,1,2,3]);
  expect(actual).toEqual(1);
});

// 1,3,2 no, because reversed
// 3,2 is reversed, so it cannot be a part
//
// identify reversed
// 0,1 - 1,0
// 2,3 - 3,2
//
// reversed groups
// 0: 0,1
// 2: 2,3
// 4: 4
//
// only one triplet set possible from these
// ([0,1], [2,3], [4])
// m: how many elements can be outside of three = 0
// 2 ** m
// 2 ** 0
//
// multiply that set by count of elements
// 2*2*1
//
// which are
// 4,0,2
// 4,1,2
// 4,0,3
// 4,1,3
test("case 2", () => {
  const actual = solution([4,0,1,3,2], [4,1,0,2,3]);
  expect(actual).toEqual(0);
});

// 0,1,4
// 2,3
//
// identify islands of reversed elements
// [4,0,1]
// 
// if 4,1 is identified as group
// then any element inside also belongs to that group
// but only if it's inside in num1 and in num2
test("case 3", () => {
  const actual = solution([4,0,1,3,2], [1,0,4,2,3]);
  expect(actual).toEqual(0);
});

// hmmm, this one is different
// [[4,5] | [1,0]]  [2|3]
// we need both 4,5 or 1,0
//
// it's like reverse group but not of elements
// in this case, revers group consists of exclusive arrays
// [1,0]: [1,0], [4,5]
//
// 2 * 2
test("case 4", () => {
  const actual = solution([4,5,1,0,3,2], [1,0,4,5,2,3]);
  expect(actual).toEqual(4);
});

// [[4|5] | [1,0]], [2|3]
// enough length in: [1,0] [2|3]
test("case 5", () => {
  const actual = solution([4,5,1,0,3,2], [1,0,5,4,2,3]);
  expect(actual).toEqual(1);
});

test("case 6", () => {
  const actual = solution([1,0,2,5,3,6,4,7], [3,0,4,5,1,6,2,7]);
  expect(actual).toEqual(5);
});
