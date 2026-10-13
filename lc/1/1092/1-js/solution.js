/**
 * @param {string} str1
 * @param {string} str2
 * @return {string}
 */
var shortestCommonSupersequence = function (str1, str2) {
  // str1: "abac"
  // str2: "cab"
  const solutions = Array.from(
    new Array(str1.length + 1),
    () => new Array(str2.length + 1)
  );

  for (let len1 = 0; len1 <= str1.length; len1++) {
    for (let len2 = 0; len2 <= str2.length; len2++) {
      // len1: 2, "ab"
      // len2: 1, "c"
      // compare three ways of arriving here
      // and store the best
      let s = "",
        best;

      if (len1) {
        s = solutions[len1 - 1][len2] + str1[len1 - 1];
        best = s.length;
      }

      if (len2) {
        const candidate = solutions[len1][len2 - 1] + str2[len2 - 1];
        if (!best || candidate.length < best) {
          s = candidate;
          best = candidate.length;
        }
      }

      if (len1 && len2 && str1[len1 - 1] === str2[len2 - 1]) {
        const candidate = solutions[len1 - 1][len2 - 1] + str1[len1 - 1];
        if (!best || candidate.length < best) {
          s = candidate;
          best = candidate.length;
        }
      }

      solutions[len1][len2] = s;
    }
  }

  return solutions[str1.length][str2.length];
};

export default shortestCommonSupersequence;
