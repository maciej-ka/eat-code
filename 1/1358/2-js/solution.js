/**
 * @param {string} s
 * @return {number}
 */
var numberOfSubstrings = function(s) {
  // abc(abc)
  // any prefix before minimal valid substring
  // is a additional score to the result

  // remember last seen
  let a = -1, b = -1, c = -1;

  let result = 0
  for (let i = 0; i < s.length; i++) {
    switch (s[i]) {
      case 'a': a = i; break;
      case 'b': b = i; break;
      case 'c': c = i; break;
    }
    result += Math.min(a, b, c) + 1
  }

  return result
};

export default numberOfSubstrings
