// too slow

struct Solution;

#[derive(Debug)]
struct Node {
    start: i32,
    end: i32,
    len: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(start: i32, end: i32) -> Self {
        Node{
            start: start,
            end: end,
            len: end - start,
            left: None,
            right: None
        }
    }

    fn divide(&mut self, at: i32) {
        if let (Some(left), Some(right)) = (&mut self.left, &mut self.right) {
            if at <= left.end {
                left.divide(at);
            } else {
                right.divide(at);
            }
            self.recalculate();
            return
        }

        self.left = Some(Box::new(Node::new(self.start, at)));
        self.right = Some(Box::new(Node::new(at, self.end)));
        self.recalculate();
    }

    fn recalculate(&mut self) {
        if let (Some(left), Some(right)) = (&self.left, &self.right) {
            self.len = left.len.max(right.len);
        }
    }

    fn find(&self, at: i32) -> &Self {
        if at == self.end { return &self }
        if let (Some(left), Some(right)) = (&self.left, &self.right) {
            if at <= left.end {
                return left.find(at)
            } else {
                return right.find(at)
            }
        }
        return &self
    }

    fn fits(&self, at: i32, size: i32) -> bool {
        if at == self.end {
            size <= self.len
        } else {
            size <= at - self.start
        }
    }
}

const MAX: i32 = 450000;

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut root = Node::new(0, MAX);
        let mut res = vec![];

        for query in queries {
            match query[0] {
                1 => root.divide(query[1]),
                _ => {
                    let mut at = query[1];
                    loop {
                        let node = root.find(at);
                        if node.fits(at, query[2]) {
                            res.push(true);
                            break
                        }
                        if node.start == 0 {
                            res.push(false);
                            break
                        }
                        at = node.start
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test_1() {
    let actual = Solution::get_results(vec![
        vec![1,2],
        vec![2,3,3],
        vec![2,3,1],
        vec![2,2,2]
    ]);
    let expected = vec![false, true, true];
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::get_results(vec![
        vec![1,7],
        vec![2,7,6],
        vec![1,2],
        vec![2,7,5],
        vec![2,7,6]
    ]);
    let expected = vec![true, true, false];
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::get_results(vec![
        vec![1,20],
        vec![1,10],
        vec![1,15],
        vec![1,2],
    ]);
    let expected = vec![];
    assert_eq!(actual, expected);
}
