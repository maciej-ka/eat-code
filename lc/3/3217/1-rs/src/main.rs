// https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/submissions/1817705118/

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut arr = vec![0; 1e5 as usize + 1];
        for n in nums { arr[n as usize] = 1 }

        let mut res = Box::new(ListNode::new(0));
        let mut tail = &mut res;

        let mut next = head;
        while let Some(node) = next {
            if arr[node.val as usize] == 0 {
                tail.next = Some(Box::new(ListNode::new(node.val)));
                tail = tail.next.as_mut().unwrap();
            }
            next = node.next;
        }
        res.next
    }
}

#[test]
fn test_1() {
    // list constructed backwards
    let list = Some(Box::new(ListNode{ val: 2, next: None }));
    let list = Some(Box::new(ListNode{ val: 1, next: list }));
    let list = Some(Box::new(ListNode{ val: 2, next: list }));
    let list = Some(Box::new(ListNode{ val: 1, next: list }));
    let list = Some(Box::new(ListNode{ val: 2, next: list }));
    let list = Some(Box::new(ListNode{ val: 1, next: list }));

    let actual = Solution::modified_list(vec![1], list);

    let expected = Some(Box::new(ListNode{ val: 2, next: None }));
    let expected = Some(Box::new(ListNode{ val: 2, next: expected }));
    let expected = Some(Box::new(ListNode{ val: 2, next: expected }));

    assert_eq!(actual, expected);
}
