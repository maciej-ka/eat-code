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
  let a = 0, e = 0, i = 0, o = 0, u = 0;
  let consonants = 0;

  // functions to operate on counts
  const change = (char, delta) => {
    switch (char) {
      case 'a': a += delta; break;
      case 'e': e += delta; break;
      case 'i': i += delta; break;
      case 'o': o += delta; break;
      case 'u': u += delta; break;
      default: consonants += delta;
    }
  }
  const getVowel = (char) => {
    switch (char) {
      case 'a': return a
      case 'e': return e
      case 'i': return i
      case 'o': return o
      case 'u': return u
    }
  }
  const enter = (char) => change(char, 1)
  const leave = (char) => change(char, -1)

  // functions to validate result
  const areVowelsValid = () => a && e && i && o && u
  const isValid = () => (consonants === k) && areVowelsValid()

  // init window to not overextend and try to be valid
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
    if (isValid() && getVowel(word[lo + 1]) > 1) {
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
