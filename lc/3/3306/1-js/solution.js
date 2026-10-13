/**
 * @param {string} word
 * @param {number} k
 * @return {number}
 */
var countOfSubstrings = function (word, k) {
  // scan trough word using moving window
  // defined by lower and higher index
  let lo = -1, hi = -1, result = 0;

  // when extending hi with a new char
  // it can be a part of many result substrings
  // and number of these substring is var multi
  let multi = 1

  // counts in current window
  const vowels = { a: 0, e: 0, i: 0, o: 0, u: 0 };
  let consonants = 0;

  // functions to change counts
  const isVowel = (char) => char === "a" || char === "e" || char === "i" || char === "o" || char === "u";
  const change = (char, delta) => isVowel(char) ? vowels[char] += delta : consonants += delta;
  const enter = (char) => change(char, 1)
  const leave = (char) => change(char, -1)

  // functions to validate result
  const areVowelsValid = () => vowels.a && vowels.e && vowels.i && vowels.o && vowels.u
  const isValid = () => (consonants === k) && areVowelsValid()

  // set window to first valid
  while (hi < word.length - 1) {
    enter(word[++hi])
    if (consonants > k) { break }
    if (isValid()) {
      result = 1
      break
    }
  }

  // not enough consonants
  if (consonants < k) { return 0 }

  while (true) {
    // overextended
    if (consonants > k) {
      leave(word[++lo])
      if (isValid()) { result++ }
      continue;
    }

    // try to shrink
    if (isValid() && vowels[word[lo + 1]] > 1) {
      leave(word[++lo])
      multi++
      result++
      continue
    }

    // impossilbe to shrink: extend
    if (hi >= word.length - 1) { break }
    enter(word[++hi])
    if (isValid()) { result += multi }
    else { multi = 1 }
  }

  return result;
};

export default countOfSubstrings;
