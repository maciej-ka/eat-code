// https://leetcode.com/problems/next-greater-numerically-balanced-number/submissions/1810619964/?envType=daily-question&envId=2025-10-24

struct Solution;

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        let magical = [ 1, 22, 122, 212, 221, 333, 1333, 3133, 3313, 3331, 4444, 14444, 22333, 23233, 23323, 23332, 32233, 32323, 32332, 33223, 33232 , 33322, 41444, 44144, 44414, 44441, 55555, 122333, 123233, 123323, 123332, 132233, 132323, 132332, 133223, 133232, 133322, 155555, 212333, 213233, 213323, 213332, 221333, 223133, 223313, 223331, 224444, 231233, 231323, 231332, 232133, 232313, 232331, 233123, 233132, 233213, 233231, 233312, 233321, 242444, 244244, 244424, 244442, 312233, 312323, 312332, 313223, 313232, 313322, 321233, 321323, 321332, 322133, 322313, 322331, 323123, 323132, 323213, 323231, 323312, 323321, 331223, 331232, 331322, 332123, 332132, 332213, 332231, 332312, 332321, 333122, 333212, 333221, 422444, 424244, 424424, 424442, 442244, 442424, 442442, 444224, 444242, 444422, 515555, 551555, 555155, 555515, 555551, 666666, 1224444 ];
        let mut lo = 0;
        let mut hi = magical.len();

        while lo < hi {
            let m = lo + hi >> 1;
            if n >= magical[m] {
                lo = m + 1;
            } else {
                hi = m;
            }
        }

       magical[lo]
    }

    fn is_beatuful(mut n: i32) -> bool {
        let mut arr = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        while n > 0 {
            arr[(n % 10) as usize] += 1;
            n = n / 10;
        }
        if arr[0] > 0 { return false }
        for i in 1..10 {
            if arr[i] > 0 && arr[i] != i { return false }
        }
        true
    }

    fn generate() {
        for n in 1..=1000_000 {
            if Solution::is_beatuful(n) { println!("{n}") }
        }
    }
}

#[test]
fn test_1() {
    let actual = Solution::next_beautiful_number(1);
    let expected = 22;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::next_beautiful_number(1000);
    let expected = 1333;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::next_beautiful_number(3000);
    let expected = 3133;
    assert_eq!(actual, expected);
}

#[test]
fn test_4() {
    let actual = Solution::next_beautiful_number(99999);
    let expected = 122333;
    assert_eq!(actual, expected);
}

#[test]
fn test_6() {
    let actual = Solution::next_beautiful_number(100000);
    let expected = 122333;
    assert_eq!(actual, expected);
}

#[test]
fn test_7() {
    let actual = Solution::next_beautiful_number(0);
    let expected = 1;
    assert_eq!(actual, expected);
}

#[test]
fn test_8() {
    let actual = Solution::next_beautiful_number(67311);
    let expected = 122333;
    assert_eq!(actual, expected);
}

#[test]
fn test_9() {
    let actual = Solution::next_beautiful_number(3017);
    let expected = 3133;
    assert_eq!(actual, expected);
}

#[test]
fn test_10() {
    let actual = Solution::next_beautiful_number(3332);
    let expected = 4444;
    assert_eq!(actual, expected);
}

#[test]
fn test_11() {
    let actual = Solution::is_beatuful(33322);
    let expected = true;
    assert_eq!(actual, expected);
}

#[test]
fn test_12() {
    let actual = Solution::is_beatuful(33320);
    let expected = false;
    assert_eq!(actual, expected);
}

// #[test]
// fn test_12() {
//     let actual = Solution::generate();
//     let expected = ();
//     assert_eq!(actual, expected);
// }
