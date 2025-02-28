/**
 * @param {string} str1
 * @param {string} str2
 * @return {string}
 */
var shortestCommonSupersequence = function (str1, str2) {
  // encode decisions of dp
  // in two last bits of value
  // (rest is solution length)
  const STR1 = 1
  const STR2 = 2
  const BOTH = 3

  // mask used to get lenght
  const LEN = ~3
  // mask used to get direction
  const DIR = 3

  const solution = Array.from(
    new Array(str1.length + 1),
    () => new Array(str2.length + 1)
  );

  for (let len1 = 0; len1 <= str1.length; len1++) {
    for (let len2 = 0; len2 <= str2.length; len2++) {
      // console.log(len1, len2);
      let next = 0

      if (len1) {
        next = (LEN & solution[len1 - 1][len2]) + 4 + STR1;
        // console.log('str1');
      }

      if (len2) {
        const candidate = (LEN & solution[len1][len2 - 1]) + 4 + STR2;
        if (!next || candidate < next) {
          next = candidate;
          // console.log('str2');
        }
      }

      if (len1 && len2 && str1[len1 - 1] === str2[len2 - 1]) {
        const candidate = (LEN & solution[len1 - 1][len2 - 1]) + 4 + BOTH;
        if (!next || candidate < next) {
          next = candidate;
          // console.log('both');
        }
      }

      // console.log(next);
      solution[len1][len2] = next;
    }
  }

  // recreate solution string from direction bits
  let i1 = str1.length
  let i2 = str2.length
  let result = ""

  while (i1 >= 0 && i2 >= 0) {
    // console.log(i1, i2);
    switch(DIR & solution[i1][i2]) {
      case STR1:
        // console.log("str1");
        i1--
        result = (str1[i1] || "") + result
        break
      case STR2:
        // console.log("str2");
        i2--
        result = (str2[i2] || "") + result
        break
      default:
        // console.log("both");
        i1--
        i2--
        result = (str1[i1] || "") + result
    }
  }
  return result
};

export default shortestCommonSupersequence;
