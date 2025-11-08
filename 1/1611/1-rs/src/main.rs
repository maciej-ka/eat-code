struct Solution;

impl Solution {
    const POWERS: [i32; 31] = [
        1 << 0,
        1 << 1,
        1 << 2,
        1 << 3,
        1 << 4,
        1 << 5,
        1 << 6,
        1 << 7,
        1 << 8,
        1 << 9,
        1 << 10,
        1 << 11,
        1 << 12,
        1 << 13,
        1 << 14,
        1 << 15,
        1 << 16,
        1 << 17,
        1 << 18,
        1 << 19,
        1 << 20,
        1 << 21,
        1 << 22,
        1 << 23,
        1 << 24,
        1 << 25,
        1 << 26,
        1 << 27,
        1 << 28,
        1 << 29,
        1 << 30
    ];

    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        if n <= 1 { return n }

        let ihi = Self::ihi(n);
        let hi = Self::POWERS[ihi];

        // only one 1
        if hi == n { return Self::POWERS[0..=ihi].iter().sum() }

        // more than one 1
        let isec = Self::ihi(n ^ hi);
        let sec = Self::POWERS[isec];

        let mut res = 0;
        // clean all other
        res += Self::minimum_one_bit_operations(n ^ hi ^ sec);
        // move isec next to ihi
        res += Self::POWERS[(isec +1 )..ihi].iter().sum::<i32>();
        // change from 11000 to 1000
        res += 1;
        // zero what's left
        res += Self::minimum_one_bit_operations(hi >> 1);

        res
    }

    fn ihi(n: i32) -> usize {
        let mut lo = 0;
        let mut hi = Self::POWERS.len();
        let mut m;
        while lo < hi {
            m = (lo + hi + 1) >> 1;
            if Self::POWERS[m] <= n {
                lo = m;
            } else {
                hi = m - 1;
            }
        }
        lo
    }
}

#[test]
fn test_1() {
    let actual = Solution::minimum_one_bit_operations(3);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::minimum_one_bit_operations(6);
    let expected = 4;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::minimum_one_bit_operations(8);
    let expected = 15;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::minimum_one_bit_operations(2);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
// 11010
fn test_5() {
    let actual = Solution::minimum_one_bit_operations(26);
    let expected = 19;
    assert_eq!(actual, expected);
}

#[test]
// 10101
fn test_6() {
    let actual = Solution::minimum_one_bit_operations(21);
    let expected = 25;
    assert_eq!(actual, expected);
}

#[test]
// 1010
fn test_7() {
    let actual = Solution::minimum_one_bit_operations(10);
    let expected = 12;
    assert_eq!(actual, expected);
}

#[test]
fn test_8() {
    let actual = Solution::minimum_one_bit_operations(16);
    let expected = 31;
    assert_eq!(actual, expected);
}

#[test]
// 11111
fn test_9() {
    let actual = Solution::minimum_one_bit_operations(31);
    let expected = 21;
    assert_eq!(actual, expected);
}

