import { expect, test, vitest } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution("aeioqq", 1);
  expect(actual).toEqual(0);
});

test('case 2', () => {
  const actual = solution("aeiou", 0);
  expect(actual).toEqual(1);
});

test('case 3', () => {
  const actual = solution("ieaouqqieaouqq", 1);
  expect(actual).toEqual(3);
});

test('case 4', () => {
  const actual = solution("aeiouw", 1);
  expect(actual).toEqual(1);
});

// aaeiouk
// aeiouk

// adding "a"
// aaeiouka
// aeiouka
// eiouka
test('case 5', () => {
  const actual = solution("aaeiouka", 1);
  expect(actual).toEqual(5);
});

// aaeiouk
// aeiouk

// aaeiouka
// aeiouka
// eiouka

// aaeioukaa
// aeioukaa
// eioukaa
test('case 6', () => {
  const actual = solution("aaeioukaa", 1);
  expect(actual).toEqual(8);
});

test('case 7', () => {
  const actual = solution("buoeia", 0);
  expect(actual).toEqual(1);
});

test('case 8', () => {
  const actual = solution("euaoei", 1);
  expect(actual).toEqual(0);
});

test('case 9', () => {
  const actual = solution("eufiahio", 1);
  expect(actual).toEqual(0);
});

test('case 10', () => {
  const actual = solution("qaouieun", 0);
  expect(actual).toEqual(2);
});
