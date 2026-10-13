// https://leetcode.com/problems/maximum-number-of-words-you-can-type/submissions/1771551358/
/**
 * @param {string} text
 * @param {string} brokenLetters
 * @return {number}
 */
var canBeTypedWords = function(text, brokenLetters) {
  const lookup = {}
  for (let i = 0; i < brokenLetters.length; i++) {
    lookup[brokenLetters[i]] = true
  }

  let words = 1
  let broken = 0
  let marked = false
  for (let i = 0; i < text.length; i++) {
    if (text[i] === ' ') {
      marked = false
      words++
      continue
    }
    if (marked) { continue }
    if (lookup[text[i]]) {
      broken++
      marked = true
    }
  }

  return words - broken
};

export default canBeTypedWords
