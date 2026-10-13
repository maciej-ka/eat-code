/**
 * @param {string} str1
 * @param {string} str2
 * @return {string}
 */
var shortestCommonSupersequence = function (str1, str2) {
  const solutions = Array.from(
    new Array(str1.length + 1),
    () => new Array(str2.length + 1)
  );

  solutions[0][0] = ""
  for (let i = 0; i < str1.length; i++) {
    solutions[i+1][0] = solutions[i][0] + str1[i]
  }
  for (let i = 0; i < str2.length; i++) {
    solutions[0][i+1] = solutions[0][i] + str2[i]
  }

  for (let i1 = 0; i1 < str1.length; i1++) {
    for (let i2 = 0; i2 < str2.length; i2++) {
      let s = solutions[i1][i2 + 1] + str1[i1];
      let best = s.length;

      const candidate = solutions[i1 + 1][i2] + str2[i2];
      if (candidate.length < best) {
        s = candidate;
        best = candidate.length;
      }

      if (str1[i1] === str2[i2]) {
        const candidate = solutions[i1][i2] + str1[i1];
        if (candidate.length < best) {
          s = candidate;
          best = candidate.length;
        }
      }

      solutions[i1 + 1][i2 + 1] = s;
    }
  }

  return solutions[str1.length][str2.length];
};

export default shortestCommonSupersequence;
