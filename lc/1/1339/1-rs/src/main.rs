// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/submissions/1878065042

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution;

/*
 * solution
 */

use std::rc::Rc;
use std::cell::RefCell;

const MOD: i64 = 1_000_000_007;

fn sum(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  if node == None { return 0 }
  let rc = node.unwrap();
  let mut node = rc.borrow_mut();
  node.val += sum(node.left.clone());
  node.val += sum(node.right.clone());
  node.val
}

fn find_closest(node: Option<Rc<RefCell<TreeNode>>>, goal: i32) -> i32 {
  let rc = node.unwrap();
  let node = rc.borrow();
  if node.val < goal { return node.val }

  let mut best = node.val;

  if node.left != None {
    let res = find_closest(node.left.clone(), goal);
    if (goal - res).abs() < (goal - best).abs() {
      best = res;
    }
  }

  if node.right != None {
    let res = find_closest(node.right.clone(), goal);
    if (goal - res).abs() < (goal - best).abs() {
      best = res;
    }
  }

  best
}

impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
      sum(root.clone());
      let total = root.clone().unwrap().borrow().val;
      let found = find_closest(root.clone(), total >> 1) as i64;
      let ans = found * (total as i64 - found) % MOD;
      ans as i32
    }
}

/*
 * tests
 */
fn node(
  val: i32,
  left: Option<Rc<RefCell<TreeNode>>>,
  right: Option<Rc<RefCell<TreeNode>>>
) -> Option<Rc<RefCell<TreeNode>>> {
  let node = TreeNode { val, left, right };
  Some(Rc::new(RefCell::new(node)))
}

#[test]
fn test_1() {
  let root = node(
    1,
    node(
      2,
      node(4, None, None),
      node(5, None, None),
    ),
    node(
      3,
      node(6, None, None),
      None
    ),
  );
  let actual = Solution::max_product(root);
  let expected = 110;
  assert_eq!(actual, expected);
}

#[test]
fn test_2() {
  let root = node(
    1,
    None,
    node(
      2,
      node(3, None, None),
      node(
        4,
        node(5, None, None),
        node(6, None, None),
      )
    )
  );
  let actual = Solution::max_product(root);
  let expected = 90;
  assert_eq!(actual, expected);
}
