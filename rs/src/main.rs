fn solve(arr: &[i32]) -> usize {
    arr.len()
}

#[test]
fn test_1() {
    let arr = [1, 2, 3];
    assert_eq!(solve(&arr), 3);
}
