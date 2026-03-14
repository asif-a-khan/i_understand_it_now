#![allow(dead_code)]

use rand::Rng;

// ── Linked List (Vec-indexed) ──────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<usize>,
}

/// Build a linked list from a slice, returns (nodes_vec, head_index).
pub fn build_list(vals: &[i32]) -> (Vec<ListNode>, Option<usize>) {
    if vals.is_empty() {
        return (vec![], None);
    }
    let mut nodes: Vec<ListNode> = vals
        .iter()
        .enumerate()
        .map(|(i, &val)| ListNode {
            val,
            next: if i + 1 < vals.len() { Some(i + 1) } else { None },
        })
        .collect();
    let _ = &mut nodes; // suppress unused warning
    (nodes, Some(0))
}

/// Collect linked list values starting from head.
pub fn collect_list(nodes: &[ListNode], head: Option<usize>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut cur = head;
    while let Some(idx) = cur {
        if idx >= nodes.len() {
            break;
        }
        result.push(nodes[idx].val);
        cur = nodes[idx].next;
    }
    result
}

// ── Binary Tree (Vec-indexed arena) ────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

/// Build a binary tree from level-order representation (None = null node).
/// Returns (arena, root_index).
pub fn build_tree(vals: &[Option<i32>]) -> (Vec<TreeNode>, Option<usize>) {
    if vals.is_empty() || vals[0].is_none() {
        return (vec![], None);
    }
    let mut arena = Vec::new();
    let mut queue = std::collections::VecDeque::new();

    let root_val = vals[0].unwrap();
    arena.push(TreeNode {
        val: root_val,
        left: None,
        right: None,
    });
    queue.push_back(0usize);

    let mut i = 1;
    while let Some(parent_idx) = queue.pop_front() {
        // Left child
        if i < vals.len() {
            if let Some(v) = vals[i] {
                let child_idx = arena.len();
                arena.push(TreeNode {
                    val: v,
                    left: None,
                    right: None,
                });
                arena[parent_idx].left = Some(child_idx);
                queue.push_back(child_idx);
            }
            i += 1;
        }
        // Right child
        if i < vals.len() {
            if let Some(v) = vals[i] {
                let child_idx = arena.len();
                arena.push(TreeNode {
                    val: v,
                    left: None,
                    right: None,
                });
                arena[parent_idx].right = Some(child_idx);
                queue.push_back(child_idx);
            }
            i += 1;
        }
    }

    (arena, Some(0))
}

/// Collect tree values in level-order (BFS), including None for missing children
/// up to the last non-None value.
pub fn tree_to_level_order(arena: &[TreeNode], root: Option<usize>) -> Vec<Option<i32>> {
    let mut result = Vec::new();
    let Some(root_idx) = root else {
        return result;
    };
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(Some(root_idx));

    while let Some(node) = queue.pop_front() {
        match node {
            Some(idx) => {
                result.push(Some(arena[idx].val));
                queue.push_back(arena[idx].left);
                queue.push_back(arena[idx].right);
            }
            None => {
                result.push(None);
            }
        }
    }
    // Trim trailing Nones
    while result.last() == Some(&None) {
        result.pop();
    }
    result
}

/// Inorder traversal of tree.
pub fn inorder(arena: &[TreeNode], root: Option<usize>) -> Vec<i32> {
    let mut result = Vec::new();
    fn helper(arena: &[TreeNode], node: Option<usize>, result: &mut Vec<i32>) {
        if let Some(idx) = node {
            helper(arena, arena[idx].left, result);
            result.push(arena[idx].val);
            helper(arena, arena[idx].right, result);
        }
    }
    helper(arena, root, &mut result);
    result
}

// ── Graph (adjacency list) ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Graph {
    pub n: usize,
    /// adj[u] = vec of (v, weight)
    pub adj: Vec<Vec<(usize, i32)>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![vec![]; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, w: i32) {
        self.adj[u].push((v, w));
    }

    pub fn add_undirected_edge(&mut self, u: usize, v: usize, w: i32) {
        self.adj[u].push((v, w));
        self.adj[v].push((u, w));
    }
}

// ── Random generation utilities ────────────────────────────────────────

/// Generate a random Vec<i32> of given length with values in [lo, hi].
pub fn random_vec(rng: &mut impl Rng, len: usize, lo: i32, hi: i32) -> Vec<i32> {
    (0..len).map(|_| rng.random_range(lo..=hi)).collect()
}

/// Generate a sorted random Vec<i32>.
pub fn random_sorted_vec(rng: &mut impl Rng, len: usize, lo: i32, hi: i32) -> Vec<i32> {
    let mut v = random_vec(rng, len, lo, hi);
    v.sort();
    v
}

/// Generate a random Vec<i32> with unique values.
pub fn random_unique_vec(rng: &mut impl Rng, len: usize, lo: i32, hi: i32) -> Vec<i32> {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    while set.len() < len {
        set.insert(rng.random_range(lo..=hi));
    }
    let mut v: Vec<i32> = set.into_iter().collect();
    // Shuffle to randomize order
    for i in (1..v.len()).rev() {
        let j = rng.random_range(0..=i);
        v.swap(i, j);
    }
    v
}

/// Generate a random string of lowercase ASCII letters.
pub fn random_string(rng: &mut impl Rng, len: usize) -> String {
    (0..len)
        .map(|_| (b'a' + rng.random_range(0..26u8)) as char)
        .collect()
}

/// Generate a random string from a specific alphabet.
pub fn random_string_from(rng: &mut impl Rng, len: usize, alphabet: &[u8]) -> String {
    (0..len)
        .map(|_| alphabet[rng.random_range(0..alphabet.len())] as char)
        .collect()
}

/// Generate a random connected undirected graph.
pub fn random_connected_graph(rng: &mut impl Rng, n: usize, extra_edges: usize) -> Graph {
    let mut g = Graph::new(n);
    // Spanning tree: connect each node to a random earlier node
    for i in 1..n {
        let j = rng.random_range(0..i);
        let w = rng.random_range(1..=100);
        g.add_undirected_edge(i, j, w);
    }
    // Extra edges
    for _ in 0..extra_edges {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u != v {
            let w = rng.random_range(1..=100);
            g.add_undirected_edge(u, v, w);
        }
    }
    g
}

/// Generate a random DAG (directed acyclic graph) with n nodes.
pub fn random_dag(rng: &mut impl Rng, n: usize, edge_count: usize) -> Graph {
    let mut g = Graph::new(n);
    for _ in 0..edge_count {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u < v {
            g.add_edge(u, v, 1);
        }
    }
    g
}

/// Generate a random binary tree as level-order Option<i32> array.
pub fn random_tree(rng: &mut impl Rng, size: usize, lo: i32, hi: i32) -> Vec<Option<i32>> {
    if size == 0 {
        return vec![];
    }
    let mut vals = vec![None; size * 2 + 1];
    vals[0] = Some(rng.random_range(lo..=hi));
    let mut count = 1;
    for i in 0..vals.len() {
        if count >= size {
            break;
        }
        if vals[i].is_some() {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            if left < vals.len() && count < size && rng.random_range(0..3) != 0 {
                vals[left] = Some(rng.random_range(lo..=hi));
                count += 1;
            }
            if right < vals.len() && count < size && rng.random_range(0..3) != 0 {
                vals[right] = Some(rng.random_range(lo..=hi));
                count += 1;
            }
        }
    }
    // Trim trailing Nones
    while vals.last() == Some(&None) {
        vals.pop();
    }
    vals
}

/// Generate a random BST (returns level-order).
pub fn random_bst(rng: &mut impl Rng, size: usize, lo: i32, hi: i32) -> Vec<Option<i32>> {
    let mut vals = random_unique_vec(rng, size, lo, hi);
    vals.sort();
    bst_from_sorted(&vals)
}

/// Build a balanced BST from sorted values, returns level-order.
fn bst_from_sorted(sorted: &[i32]) -> Vec<Option<i32>> {
    if sorted.is_empty() {
        return vec![];
    }
    let mut arena = Vec::new();
    fn build(sorted: &[i32], lo: usize, hi: usize, arena: &mut Vec<TreeNode>) -> Option<usize> {
        if lo > hi {
            return None;
        }
        let mid = lo + (hi - lo) / 2;
        let idx = arena.len();
        arena.push(TreeNode {
            val: sorted[mid],
            left: None,
            right: None,
        });
        let left = if mid > lo {
            build(sorted, lo, mid - 1, arena)
        } else {
            None
        };
        let right = if mid < hi {
            build(sorted, mid + 1, hi, arena)
        } else {
            None
        };
        arena[idx].left = left;
        arena[idx].right = right;
        Some(idx)
    }
    let root = build(sorted, 0, sorted.len() - 1, &mut arena);
    tree_to_level_order(&arena, root)
}

/// Generate a 2D grid of given dimensions filled with random values.
pub fn random_grid(rng: &mut impl Rng, rows: usize, cols: usize, lo: i32, hi: i32) -> Vec<Vec<i32>> {
    (0..rows)
        .map(|_| (0..cols).map(|_| rng.random_range(lo..=hi)).collect())
        .collect()
}
