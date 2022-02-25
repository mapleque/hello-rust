// work with this btree
//      a
//     / \
//    /   \
//   b     c
//  / \   / \
// d   e f   g
//
// f walk: d b e a f c g
// m walk: a b c d e f g
fn main() {
    let f = String::from("dbeafcg");
    let m = String::from("abcdefg");
    let tree = build_tree(f, m);
    println!("dfs: {}", dfs(&tree));
    println!("bfs: {}", bfs(&tree));
    println!("b walk: {}", b_walk(tree));
}

struct TreeNode {
    val: char,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// build with front and middle walk
fn build_tree(f: String, m: String) -> TreeNode {
    let fc: Vec<char> = f.chars().collect();
    if f.len() == 1 {
        return TreeNode {
            val: fc[0],
            left: None,
            right: None,
        };
    }
    if let Some((s, i)) = find_split(&f, &m) {
        let left = build_tree((&f[0..s]).into(), (&m[i..]).into());
        let right = build_tree((&f[s + 1..]).into(), (&m[i..]).into());
        return TreeNode {
            val: fc[s],
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
    } else {
        panic!("invalid input")
    }
}

fn find_split(f: &String, m: &String) -> Option<(usize, usize)> {
    for (i, e) in m.chars().enumerate() {
        if let Some(s) = find_index(f, e) {
            return Some((s, i));
        }
    }
    None
}

fn find_index(f: &String, n: char) -> Option<usize> {
    for (i, e) in f.chars().enumerate() {
        if e == n {
            return Some(i);
        }
    }
    None
}

fn dfs(tree: &TreeNode) -> String {
    let mut res = String::new();
    if let Some(node) = &tree.left {
        let s: String = dfs(&*node);
        res.push_str(&s[..]);
    };
    if let Some(node) = &tree.right {
        let s: String = dfs(&*node);
        res.push_str(&s[..]);
    };
    res.push(tree.val);
    res
}

fn bfs(tree: &TreeNode) -> String {
    let mut queue: Vec<&TreeNode> = Vec::new();
    queue.push(tree);
    bfs_inner(queue)
}

fn bfs_inner(queue: Vec<&TreeNode>) -> String {
    let mut res = String::new();
    if queue.len() == 0 {
        return res;
    }
    let mut next_level_queue: Vec<&TreeNode> = Vec::new();
    for tree in queue {
        res.push(tree.val);
        if let Some(node) = &tree.left {
            next_level_queue.push(&*node);
        }
        if let Some(node) = &tree.right {
            next_level_queue.push(&*node);
        }
    }
    let s = bfs_inner(next_level_queue);
    res.push_str(&s);
    res
}
fn b_walk(tree: TreeNode) -> String {
    let mut res = String::new();
    if let Some(node) = tree.right {
        let s = b_walk(*node);
        res.push_str(&s);
    }
    res.push(tree.val);
    if let Some(node) = tree.left {
        let s = b_walk(*node);
        res.push_str(&s);
    }
    res
}
