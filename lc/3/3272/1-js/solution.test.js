import { expect, test } from "vitest";
import solution from "./solution";

// palindroms with 3 digits
// cannot have leading 0
// -: means mirror
// (1..9)(0..9)-
//
// 5k-palindroms: divisible by 5
// 5(0..9)5
//
// good numbers: rearrangements
// 55(0..9)
// 5(0..9)5 - 1 duplicate
// (1..9)55 - 1 duplicate
// 10 + 10 + 9 - 2 = 27
test("case 1", () => {
  const actual = solution(3, 5);
  expect(actual).toEqual(27);
});

// palindroms with 1 digit
// (1..9)
//
// divisible by 4
// [4, 8]
//
// rearrangements
// [4, 8]
test("case 2", () => {
  const actual = solution(1, 4);
  expect(actual).toEqual(2);
});

// palindroms with 5 digits
// (1..9)(0..9)(0..9)--
test("case 3", () => {
  const actual = solution(5, 6);
  expect(actual).toEqual(2468);
});

// (1..9)-
test("case 4", () => {
  const actual = solution(2, 1);
  expect(actual).toEqual(9);
});

// (a: 1..9)(b: 0..9)-
//
// aab
// (a: 1..9)a(0..9)
// 110
// 111
// 112
// 113
// ...
// 220
// 221
// 222
// 223
// ...
// 9 * 10 = 90 options
//
// aba
// (a: 1..9)(0..9)a
// 101
// x 111
// 121
// 131
// ...
// 202
// 212
// x 222
// 232
// ...
// 9 * 10 = 90 options
// - 9 duplicates
//
// baa
// (0..9)(a: 1..9)a
// => (1..9)(a: 1..9)a
// 122
// 133
// 144
// ...
// 211
// x 222
// 233
// ...
// 9 * 9 = 81 options
// - 9 duplicates
//
// 90 + 90 + 81 - 9 - 9
// 261 - 18
test("case 5", () => {
  const actual = solution(3, 1);
  expect(actual).toEqual(243);
});

// (2,4,6,8)(0..9)-
//
// aab
// (a:2,4,6,8)a(0..9)
// 220
// 221
// 222
// 223
// ...
// 440
// 441
// 442
// 443
// 444
// ...
// 4 * 10 = 40
//
// aba
// (a:2,4,6,8)(0..9)a
// 202
// 212
// x 222
// 232
// 242
//
// 404
// 414
// 424
// 434
// x 444
// ...
// 4 * 10 = 40
// - 4 duplicates
//
// baa
// (0..9)(a:2,4,6,8)a
// => (1..9)(a:2,4,6,8)a
// 122
// 144
// 166
// ...
// x 222
// 244
// 266
// ...
// 322
// 344
// 366
// ...
// 422
// x 444
// 466
// ...
// 9 * 4 = 36
// - 4 duplicates
//
// 40 + 40 + 36 - 4 - 4
// 116 - 8
test("case 6", () => {
  const actual = solution(3, 2);
  expect(actual).toEqual(108);
});

// (a: 1..9)(b: 0..9)(c: 0..9)ba
//
// abcba
// (a: 1..9)(b: 0..9)(c: 0..9)ba
// 9 * 10 * 10 = 900
//
// a has to be different then b
// otherwise there will be duplicate
// abcab
//
//
test("case 7", () => {
  const actual = solution(5, 1);
  expect(actual).toEqual(10935);
});

test("case 8", () => {
  const actual = solution(6, 1);
  expect(actual).toEqual(10944);
});
