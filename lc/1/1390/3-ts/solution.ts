// https://leetcode.com/problems/four-divisors/submissions/1874358874

function sumFourDivisors(nums: number[]): number {
  let res = 0;

  nums.forEach((n) => {
    let sum = 1 + n;
    let count = 2;
    const root = Math.floor(Math.sqrt(n));
    for (let d = 2; d <= root; d++) {
      if (n % d) continue;
      const other = n / d;
      if (d == other) {
        sum += d
        count++
      } else {
        sum += d + other
        count += 2
      }
      if (count > 4) break
    }
    if (count == 4) {
      res += sum
    }
  })

  return res;
};

export default sumFourDivisors;
