/**
 * @param {number[][]} questions
 * @return {number}
 */

// dynamic programming

// to reuse previous answers
// process questions from last to first

var mostPoints = function(questions) {
  // fill dp in place of questions
  for (let i = questions.length - 1; i >= 0; i--) {
    questions[i] = Math.max(
      // don't solve
      questions[i + 1] || 0,
      // solve
      questions[i][0] + (questions[i + 1 + questions[i][1]] || 0)
    )
  }

  return questions[0]
};

export default mostPoints
