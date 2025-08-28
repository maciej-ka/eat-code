/**
 * @param {string} s
 * @return {number}
 */
var numberOfSubstrings = function(s) {
  // sliding variable window
  let lo = 0, hi = 0;

  // counts
  let a = 0, b = 0, c = 0;
  const getCount = (char) => {
    switch(char) {
      case 'a': return a;
      case 'b': return b;
      case 'c': return c;
    }
  }
  const enter = (char) => {
    switch(char) {
      case 'a': a++; break;
      case 'b': b++; break;
      case 'c': c++; break;
    }
  }
  const leave = (char) => {
    switch(char) {
      case 'a': a--; break;
      case 'b': b--; break;
      case 'c': c--; break;
    }
  }

  // initialize window to first substring
  while (true) {
    if (hi === s.length) { return 0 }
    enter(s[hi++])
    if (a && b && c) { break }
  }

  // minimal valid substrings count
  let minimal = 1, result = 1;

  while (true) {
    // prioritize identify of minimal valid substrings
    // shrink the window whenever its possible to have valid result after
    if (getCount(s[lo]) > 1) {
      leave(s[lo++])
      minimal++
      result++
      continue
    }

    // not possible to extend
    if (hi === s.length) {
      return result
    }

    // extend adds to all minimal substrings
    enter(s[hi++])
    result += minimal
  }
};

export default numberOfSubstrings
