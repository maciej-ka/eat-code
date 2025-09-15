// https://leetcode.com/problems/maximum-number-of-words-you-can-type/submissions/1771556507
/**
 * @param {string} text
 * @param {string} brokenLetters
 * @return {number}
 */
var canBeTypedWords = function(text, brokenLetters) {
  const set = new Set()
  for (let i = 0; i < brokenLetters.length; i++) {
    set.add(brokenLetters[i])
  }

  let words = 1
  let broken = 0
  let marked = false
  for (let char of text) {
    if (char === ' ') {
      marked = false
      words++
      continue
    }
    if (marked) { continue }
    if (set.has(char)) {
      broken++
      marked = true
    }
  }

  return words - broken
}

export default canBeTypedWords
