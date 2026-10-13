// https://leetcode.com/problems/create-binary-tree-from-descriptions/submissions/2025380060/?envType=daily-question&envId=2026-06-07

struct Solution;

// Definition for a binary tree node.
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

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = HashMap::<i32, Rc<RefCell<TreeNode>>>::with_capacity(descriptions.len() + 1);
        let mut parents = HashMap::<i32, i32>::with_capacity(descriptions.len() + 1);

        for d in &descriptions {
            parents.insert(d[1], d[0]);
            let parent = nodes.entry(d[0]).or_insert(Rc::new(RefCell::new(TreeNode::new(d[0])))).clone();
            let child = nodes.entry(d[1]).or_insert(Rc::new(RefCell::new(TreeNode::new(d[1])))).clone();
            if d[2] == 1 {
                parent.borrow_mut().left = Some(child);
            } else {
                parent.borrow_mut().right = Some(child);
            }
        }

        let mut val = &descriptions[0][0];
        while let Some(parent) = parents.get(val) {
            val = parent;
        }
        Some((*nodes.get(val).unwrap()).clone())
    }
}

#[test]
fn test_1() {
    let actual = Solution::create_binary_tree(vec![vec![20,15,1],vec![20,17,0],vec![50,20,1],vec![50,80,0],vec![80,19,1]]);

    let ll = TreeNode::new(15);
    let lr = TreeNode::new(17);
    let rl = TreeNode::new(19);
    let mut l = TreeNode::new(20);
    let mut r = TreeNode::new(80);
    let mut root = TreeNode::new(50);

    l.left = Some(Rc::new(RefCell::new(ll)));
    l.right = Some(Rc::new(RefCell::new(lr)));
    r.left = Some(Rc::new(RefCell::new(rl)));
    root.left = Some(Rc::new(RefCell::new(l)));
    root.right = Some(Rc::new(RefCell::new(r)));

    let expected = Some(Rc::new(RefCell::new(root)));
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::create_binary_tree(vec![vec![1,2,1],vec![2,3,0],vec![3,4,1]]);

    let lrl = TreeNode::new(4);
    let mut lr = TreeNode::new(3);
    let mut l = TreeNode::new(2);
    let mut root = TreeNode::new(1);

    lr.left = Some(Rc::new(RefCell::new(lrl)));
    l.right = Some(Rc::new(RefCell::new(lr)));
    root.left = Some(Rc::new(RefCell::new(l)));

    let expected = Some(Rc::new(RefCell::new(root)));
    assert_eq!(actual, expected);
}
