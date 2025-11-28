// wrong, it's not a binary tree

struct Solution;

#[derive(Debug)]
struct Node {
    val: i32,
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
            parent: None,
        }
    }
}

impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        let mut nodes = Vec::<Node>::with_capacity(n as usize);
        for i in 0..values.len() {
            nodes.push(Node::new(values[i] % k));
        }

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let v_has_parent = nodes[v].parent != None;
            if !v_has_parent && nodes[u].left == None {
                nodes[u].left = Some(v);
                nodes[v].parent = Some(u);
            } else if !v_has_parent && nodes[u].right == None {
                nodes[u].right = Some(v);
                nodes[v].parent = Some(u);
            } else if nodes[v].left == None {
                nodes[v].left = Some(u);
                nodes[u].parent = Some(v);
            } else if nodes[v].right == None {
                nodes[v].right = Some(u);
                nodes[u].parent = Some(v);
            }
        }

        let mut root = 0;
        while let Some(i) = nodes[root].parent {
            root = i;
        }

        #[derive(Debug)]
        struct Result {
            sum: i32,
            cuts: i32,
        }

        fn visit(i: usize, nodes: &Vec<Node>, k: i32) -> Result {
            let mut cuts = 0;
            let mut sum = nodes[i].val;
            if let Some(left) = nodes[i].left {
                let l = visit(left, nodes, k);
                println!("");
                println!("i {:?}", left);
                println!("left {:?}", left);
                println!("res {:?}", l);
                sum += l.sum;
                cuts += l.cuts;
                if l.sum == 0 { cuts += 1 }
            }
            if let Some(right) = nodes[i].right {
                let r = visit(right, nodes, k);
                println!("");
                println!("i {:?}", right);
                println!("right {:?}", right);
                println!("res {:?}", r);
                sum += r.sum;
                cuts += r.cuts;
                if r.sum == 0 { cuts += 1 }
            }
            Result { cuts, sum: sum % k }
        }

        visit(root, &nodes, k).cuts + 1
    }
}

#[test]
fn test_1() {
    let actual = Solution::max_k_divisible_components(5, vec![vec![0,2],vec![1,2],vec![1,3],vec![2,4]], vec![1,8,1,4,4], 6);
    let expected = 2;
    assert_eq!(actual, expected);
}

#[test]
fn test_2() {
    let actual = Solution::max_k_divisible_components(7, vec![vec![0,1],vec![0,2],vec![1,3],vec![1,4],vec![2,5],vec![2,6]], vec![3,0,6,1,5,2,1], 3);
    let expected = 3;
    assert_eq!(actual, expected);
}

#[test]
fn test_3() {
    let actual = Solution::max_k_divisible_components(9, vec![vec![1,2],vec![1,7],vec![0,6],vec![0,8],vec![0,3],vec![3,4],vec![0,5],vec![2,5]], vec![1,4,4,0,2,1,1,6,2], 7);
    let expected = 2;
    assert_eq!(actual, expected);
}
