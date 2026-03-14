use rand::Rng;

use crate::problems::helpers::{self, TreeNode, build_tree, tree_to_level_order, inorder, random_bst, random_tree};
use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part3_trees::balanced_bst as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(IsBalanced),
        Box::new(SortedArrayToBST),
        Box::new(MinDepth),
        Box::new(TreeHeight),
        Box::new(CountNodes),
        Box::new(BalanceBST),
        Box::new(ConvertSortedList),
        Box::new(ClosestValue),
        Box::new(KthSmallest),
        Box::new(AllElementsTwoBST),
        Box::new(LargestBSTSubtree),
        Box::new(VerifyPreorder),
        Box::new(CountRange),
        Box::new(MedianBST),
        Box::new(RankFromStream),
    ]
}

// ── helpers ──────────────────────────────────────────────────────────────

fn is_balanced_tree(arena: &[TreeNode], root: Option<usize>) -> bool {
    fn height(arena: &[TreeNode], node: Option<usize>) -> i32 {
        match node {
            None => 0,
            Some(idx) => {
                let lh = height(arena, arena[idx].left);
                let rh = height(arena, arena[idx].right);
                if lh == -1 || rh == -1 || (lh - rh).abs() > 1 {
                    return -1;
                }
                1 + lh.max(rh)
            }
        }
    }
    height(arena, root) != -1
}

fn tree_height(arena: &[TreeNode], root: Option<usize>) -> i32 {
    match root {
        None => 0,
        Some(idx) => {
            let lh = tree_height(arena, arena[idx].left);
            let rh = tree_height(arena, arena[idx].right);
            1 + lh.max(rh)
        }
    }
}

fn is_bst(arena: &[TreeNode], root: Option<usize>) -> bool {
    fn check(arena: &[TreeNode], node: Option<usize>, min: i64, max: i64) -> bool {
        match node {
            None => true,
            Some(idx) => {
                let val = arena[idx].val as i64;
                if val <= min || val >= max { return false; }
                check(arena, arena[idx].left, min, val)
                    && check(arena, arena[idx].right, val, max)
            }
        }
    }
    check(arena, root, i64::MIN, i64::MAX)
}

fn count_nodes(arena: &[TreeNode], root: Option<usize>) -> i32 {
    match root {
        None => 0,
        Some(idx) => 1 + count_nodes(arena, arena[idx].left) + count_nodes(arena, arena[idx].right),
    }
}

/// Generate a random unbalanced BST by inserting random values.
fn random_unbalanced_bst(rng: &mut impl Rng, size: usize, lo: i32, hi: i32) -> Vec<Option<i32>> {
    let vals = helpers::random_unique_vec(rng, size, lo, hi);
    let mut arena: Vec<TreeNode> = Vec::new();
    let mut root: Option<usize> = None;

    for v in vals {
        root = Some(bst_insert(&mut arena, root, v));
    }
    tree_to_level_order(&arena, root)
}

fn bst_insert(arena: &mut Vec<TreeNode>, root: Option<usize>, val: i32) -> usize {
    match root {
        None => {
            let idx = arena.len();
            arena.push(TreeNode { val, left: None, right: None });
            idx
        }
        Some(idx) => {
            if val < arena[idx].val {
                let left = bst_insert(arena, arena[idx].left, val);
                arena[idx].left = Some(left);
            } else {
                let right = bst_insert(arena, arena[idx].right, val);
                arena[idx].right = Some(right);
            }
            idx
        }
    }
}

// ── Easy 1: Is Balanced ─────────────────────────────────────────────────

struct IsBalanced;
struct IsBalancedTest { tree: Vec<Option<i32>> }

impl Problem for IsBalanced {
    fn id(&self) -> &str { "balanced_bst_is_balanced" }
    fn name(&self) -> &str { "Is Balanced" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a binary tree (level-order representation), determine if it is \
         height-balanced. A tree is height-balanced if for every node, the depth \
         of the two subtrees differs by at most 1.\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(0..=15);
            let tree = if rng.random_range(0..2) == 0 {
                random_bst(&mut rng, n, -100, 100)
            } else {
                random_tree(&mut rng, n, -100, 100)
            };
            TestCase { data: Box::new(IsBalancedTest { tree }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IsBalancedTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let expected = is_balanced_tree(&arena, root);
        let actual = solutions::is_balanced(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 2: Sorted Array to BST ─────────────────────────────────────────

struct SortedArrayToBST;
struct SortedArrayToBSTTest { nums: Vec<i32> }

impl Problem for SortedArrayToBST {
    fn id(&self) -> &str { "balanced_bst_sorted_array_to_bst" }
    fn name(&self) -> &str { "Sorted Array to BST" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a sorted (ascending) integer array with unique values, convert it to a \
         height-balanced BST. Return the level-order representation.\n\n\
         The resulting tree must be a valid BST and height-balanced.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(0..=20);
            let nums = helpers::random_unique_vec(&mut rng, n, -100, 100);
            let mut nums = nums;
            nums.sort();
            TestCase { data: Box::new(SortedArrayToBSTTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortedArrayToBSTTest>().unwrap();
        let actual = solutions::sorted_array_to_bst(&t.nums);
        let (arena, root) = build_tree(&actual);
        let in_order = inorder(&arena, root);
        let balanced = is_balanced_tree(&arena, root);
        let valid_bst = is_bst(&arena, root);
        let is_correct = in_order == t.nums && balanced && valid_bst;
        SolutionResult {
            is_correct,
            input_description: format!("nums={:?}", t.nums),
            expected: "valid balanced BST with same elements".to_string(),
            actual: format!("tree={:?}, balanced={}, bst={}, inorder={:?}", actual, balanced, valid_bst, in_order),
        }
    }
}

// ── Easy 3: Min Depth ───────────────────────────────────────────────────

struct MinDepth;
struct MinDepthTest { tree: Vec<Option<i32>> }

impl Problem for MinDepth {
    fn id(&self) -> &str { "balanced_bst_min_depth" }
    fn name(&self) -> &str { "Minimum Depth of Binary Tree" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a binary tree, find its minimum depth. The minimum depth is the number of \
         nodes along the shortest path from the root node down to the nearest leaf node.\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 1000\n\
         - Return 0 for an empty tree"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(0..=15);
            let tree = random_tree(&mut rng, n, -100, 100);
            TestCase { data: Box::new(MinDepthTest { tree }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MinDepthTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let expected = ref_min_depth(&arena, root);
        let actual = solutions::min_depth(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_min_depth(arena: &[TreeNode], root: Option<usize>) -> i32 {
    match root {
        None => 0,
        Some(idx) => {
            let left = arena[idx].left;
            let right = arena[idx].right;
            if left.is_none() && right.is_none() { return 1; }
            if left.is_none() { return 1 + ref_min_depth(arena, right); }
            if right.is_none() { return 1 + ref_min_depth(arena, left); }
            1 + ref_min_depth(arena, left).min(ref_min_depth(arena, right))
        }
    }
}

// ── Easy 4: Tree Height ─────────────────────────────────────────────────

struct TreeHeight;
struct TreeHeightTest { tree: Vec<Option<i32>> }

impl Problem for TreeHeight {
    fn id(&self) -> &str { "balanced_bst_height" }
    fn name(&self) -> &str { "Height of Binary Tree" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Calculate the height of a binary tree. The height is the number of nodes along \
         the longest path from the root to any leaf.\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 1000\n\
         - Return 0 for an empty tree"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(0..=15);
            let tree = random_tree(&mut rng, n, -100, 100);
            TestCase { data: Box::new(TreeHeightTest { tree }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TreeHeightTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let expected = tree_height(&arena, root);
        let actual = solutions::height(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 5: Count Nodes in Complete Binary Tree ─────────────────────────

struct CountNodes;
struct CountNodesTest { tree: Vec<Option<i32>> }

impl Problem for CountNodes {
    fn id(&self) -> &str { "balanced_bst_count_nodes" }
    fn name(&self) -> &str { "Count Complete Tree Nodes" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a complete binary tree, return the number of nodes.\n\n\
         A complete binary tree is one where every level, except possibly the last, is \
         completely filled, and all nodes in the last level are as far left as possible.\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 1000\n\
         - Try to achieve better than O(n) time"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(0..=20);
            // Build a complete binary tree
            let tree: Vec<Option<i32>> = (0..n).map(|_| Some(rng.random_range(-100..=100))).collect();
            TestCase { data: Box::new(CountNodesTest { tree }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountNodesTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let expected = count_nodes(&arena, root);
        let actual = solutions::count_nodes(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 1: Balance a BST ─────────────────────────────────────────────

struct BalanceBST;
struct BalanceBSTTest { tree: Vec<Option<i32>> }

impl Problem for BalanceBST {
    fn id(&self) -> &str { "balanced_bst_balance" }
    fn name(&self) -> &str { "Balance a BST" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given the root of a BST, return a balanced BST with the same node values.\n\n\
         The result must be a valid BST and height-balanced.\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=15);
            let tree = random_unbalanced_bst(&mut rng, n, -200, 200);
            TestCase { data: Box::new(BalanceBSTTest { tree }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BalanceBSTTest>().unwrap();
        let (orig_arena, orig_root) = build_tree(&t.tree);
        let orig_inorder = inorder(&orig_arena, orig_root);

        let actual = solutions::balance_bst(&t.tree);
        let (act_arena, act_root) = build_tree(&actual);
        let act_inorder = inorder(&act_arena, act_root);
        let balanced = is_balanced_tree(&act_arena, act_root);
        let valid_bst = is_bst(&act_arena, act_root);

        let is_correct = act_inorder == orig_inorder && balanced && valid_bst;
        SolutionResult {
            is_correct,
            input_description: format!("tree={:?}", t.tree),
            expected: "balanced BST with same elements".to_string(),
            actual: format!("tree={:?}, balanced={}, bst={}", actual, balanced, valid_bst),
        }
    }
}

// ── Medium 2: Convert Sorted List to BST ────────────────────────────────

struct ConvertSortedList;
struct ConvertSortedListTest { nums: Vec<i32> }

impl Problem for ConvertSortedList {
    fn id(&self) -> &str { "balanced_bst_convert_sorted_list" }
    fn name(&self) -> &str { "Convert Sorted List to BST" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a sorted (ascending) list of integers, convert it to a height-balanced BST. \
         Return the level-order representation.\n\n\
         Similar to sorted array to BST but conceptually treats the input as a linked list.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(0..=20);
            let nums = helpers::random_sorted_vec(&mut rng, n, -100, 100);
            // Deduplicate for BST property
            let mut nums = nums;
            nums.dedup();
            TestCase { data: Box::new(ConvertSortedListTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ConvertSortedListTest>().unwrap();
        let actual = solutions::sorted_list_to_bst(&t.nums);
        let (arena, root) = build_tree(&actual);
        let in_order = inorder(&arena, root);
        let balanced = is_balanced_tree(&arena, root);
        let valid_bst = is_bst(&arena, root);
        let is_correct = in_order == t.nums && balanced && valid_bst;
        SolutionResult {
            is_correct,
            input_description: format!("nums={:?}", t.nums),
            expected: "valid balanced BST with same elements".to_string(),
            actual: format!("tree={:?}, balanced={}, bst={}, inorder={:?}", actual, balanced, valid_bst, in_order),
        }
    }
}

// ── Medium 3: Closest Value in BST ──────────────────────────────────────

struct ClosestValue;
struct ClosestValueTest { tree: Vec<Option<i32>>, target: f64 }

impl Problem for ClosestValue {
    fn id(&self) -> &str { "balanced_bst_closest_value" }
    fn name(&self) -> &str { "Closest Value in BST" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a BST and a target floating-point value, find the value in the BST that is \
         closest to the target.\n\n\
         If two values are equally close, return the smaller one.\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 1000\n\
         - The tree is a valid BST"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=15);
            let tree = random_bst(&mut rng, n, -100, 100);
            let target = rng.random_range(-120.0..=120.0_f64);
            // Round to 1 decimal for cleaner display
            let target = (target * 10.0).round() / 10.0;
            TestCase { data: Box::new(ClosestValueTest { tree, target }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ClosestValueTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let expected = ref_closest_value(&arena, root, t.target);
        let actual = solutions::closest_value(&t.tree, t.target);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}, target={}", t.tree, t.target),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_closest_value(arena: &[TreeNode], root: Option<usize>, target: f64) -> i32 {
    let mut closest = arena[root.unwrap()].val;
    let mut node = root;
    while let Some(idx) = node {
        let val = arena[idx].val;
        let diff = (val as f64 - target).abs();
        let best_diff = (closest as f64 - target).abs();
        if diff < best_diff || (diff == best_diff && val < closest) {
            closest = val;
        }
        node = if target < val as f64 { arena[idx].left } else { arena[idx].right };
    }
    closest
}

// ── Medium 4: Kth Smallest in BST ───────────────────────────────────────

struct KthSmallest;
struct KthSmallestTest { tree: Vec<Option<i32>>, k: usize }

impl Problem for KthSmallest {
    fn id(&self) -> &str { "balanced_bst_kth_smallest" }
    fn name(&self) -> &str { "Kth Smallest Element in BST" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given the root of a BST and an integer `k`, return the kth smallest value \
         (1-indexed) in the tree.\n\n\
         Constraints:\n\
         - 1 <= k <= number of nodes <= 1000\n\
         - The tree is a valid BST with unique values"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=20);
            let tree = random_bst(&mut rng, n, -200, 200);
            let k = rng.random_range(1..=n);
            TestCase { data: Box::new(KthSmallestTest { tree, k }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<KthSmallestTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let in_order = inorder(&arena, root);
        let expected = in_order[t.k - 1];
        let actual = solutions::kth_smallest(&t.tree, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}, k={}", t.tree, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 5: All Elements in Two BSTs ──────────────────────────────────

struct AllElementsTwoBST;
struct AllElementsTest { tree1: Vec<Option<i32>>, tree2: Vec<Option<i32>> }

impl Problem for AllElementsTwoBST {
    fn id(&self) -> &str { "balanced_bst_all_elements_two_bst" }
    fn name(&self) -> &str { "All Elements in Two BSTs" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given two BSTs, return a sorted list of all elements from both trees.\n\n\
         Constraints:\n\
         - 0 <= number of nodes in each tree <= 500"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n1 = rng.random_range(0..=10);
            let n2 = rng.random_range(0..=10);
            let tree1 = random_bst(&mut rng, n1, -100, 100);
            let tree2 = random_bst(&mut rng, n2, -100, 100);
            TestCase { data: Box::new(AllElementsTest { tree1, tree2 }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AllElementsTest>().unwrap();
        let (a1, r1) = build_tree(&t.tree1);
        let (a2, r2) = build_tree(&t.tree2);
        let mut expected = inorder(&a1, r1);
        expected.extend(inorder(&a2, r2));
        expected.sort();
        let actual = solutions::all_elements_two_bst(&t.tree1, &t.tree2);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree1={:?}, tree2={:?}", t.tree1, t.tree2),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 1: Largest BST Subtree ─────────────────────────────────────────

struct LargestBSTSubtree;
struct LargestBSTTest { tree: Vec<Option<i32>> }

impl Problem for LargestBSTSubtree {
    fn id(&self) -> &str { "balanced_bst_largest_bst_subtree" }
    fn name(&self) -> &str { "Largest BST Subtree" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given a binary tree, find the largest subtree that is also a valid BST. \
         Return the number of nodes in that subtree.\n\n\
         A subtree must include all of its descendants.\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(0..=15);
            let tree = random_tree(&mut rng, n, -50, 50);
            TestCase { data: Box::new(LargestBSTTest { tree }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LargestBSTTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let expected = ref_largest_bst_subtree(&arena, root);
        let actual = solutions::largest_bst_subtree(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_largest_bst_subtree(arena: &[TreeNode], root: Option<usize>) -> i32 {
    // Returns (is_bst, min_val, max_val, size, largest_bst_size)
    fn helper(arena: &[TreeNode], node: Option<usize>) -> (bool, i64, i64, i32, i32) {
        match node {
            None => (true, i64::MAX, i64::MIN, 0, 0),
            Some(idx) => {
                let (l_bst, l_min, l_max, l_size, l_largest) = helper(arena, arena[idx].left);
                let (r_bst, r_min, r_max, r_size, r_largest) = helper(arena, arena[idx].right);
                let val = arena[idx].val as i64;
                if l_bst && r_bst && l_max < val && val < r_min {
                    let size = l_size + r_size + 1;
                    (true, l_min.min(val), r_max.max(val), size, size)
                } else {
                    (false, 0, 0, 0, l_largest.max(r_largest))
                }
            }
        }
    }
    let (_, _, _, _, largest) = helper(arena, root);
    largest
}

// ── Hard 2: Verify Preorder of BST ──────────────────────────────────────

struct VerifyPreorder;
struct VerifyPreorderTest { preorder: Vec<i32> }

impl Problem for VerifyPreorder {
    fn id(&self) -> &str { "balanced_bst_verify_preorder" }
    fn name(&self) -> &str { "Verify Preorder Sequence of BST" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given an array of unique integers, determine if it is a valid preorder traversal \
         sequence of a BST.\n\n\
         Constraints:\n\
         - 1 <= preorder.len() <= 1000\n\
         - All values are unique"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=15);
            let preorder = if rng.random_range(0..2) == 0 {
                // Generate valid preorder from a BST
                let tree = random_bst(&mut rng, n, -200, 200);
                let (arena, root) = build_tree(&tree);
                ref_preorder(&arena, root)
            } else {
                // Random permutation (may or may not be valid)
                helpers::random_unique_vec(&mut rng, n, -200, 200)
            };
            TestCase { data: Box::new(VerifyPreorderTest { preorder }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<VerifyPreorderTest>().unwrap();
        let expected = ref_verify_preorder(&t.preorder);
        let actual = solutions::verify_preorder(&t.preorder);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("preorder={:?}", t.preorder),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_preorder(arena: &[TreeNode], root: Option<usize>) -> Vec<i32> {
    let mut result = Vec::new();
    fn helper(arena: &[TreeNode], node: Option<usize>, result: &mut Vec<i32>) {
        if let Some(idx) = node {
            result.push(arena[idx].val);
            helper(arena, arena[idx].left, result);
            helper(arena, arena[idx].right, result);
        }
    }
    helper(arena, root, &mut result);
    result
}

fn ref_verify_preorder(preorder: &[i32]) -> bool {
    let mut stack: Vec<i32> = Vec::new();
    let mut low = i64::MIN;
    for &val in preorder {
        if (val as i64) < low { return false; }
        while let Some(&top) = stack.last() {
            if top < val {
                low = stack.pop().unwrap() as i64;
            } else {
                break;
            }
        }
        stack.push(val);
    }
    true
}

// ── Hard 3: Count Nodes in Range ────────────────────────────────────────

struct CountRange;
struct CountRangeTest { tree: Vec<Option<i32>>, lo: i32, hi: i32 }

impl Problem for CountRange {
    fn id(&self) -> &str { "balanced_bst_count_range" }
    fn name(&self) -> &str { "Count Nodes in Range" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given a BST and a range [lo, hi], count the number of nodes with values \
         in the inclusive range [lo, hi].\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 1000\n\
         - The tree is a valid BST"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(0..=20);
            let tree = random_bst(&mut rng, n, -100, 100);
            let a = rng.random_range(-120..=120);
            let b = rng.random_range(-120..=120);
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            TestCase { data: Box::new(CountRangeTest { tree, lo, hi }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountRangeTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let in_order = inorder(&arena, root);
        let expected = in_order.iter().filter(|&&v| v >= t.lo && v <= t.hi).count() as i32;
        let actual = solutions::count_range(&t.tree, t.lo, t.hi);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}, lo={}, hi={}", t.tree, t.lo, t.hi),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 4: Median of BST ──────────────────────────────────────────────

struct MedianBST;
struct MedianBSTTest { tree: Vec<Option<i32>> }

impl Problem for MedianBST {
    fn id(&self) -> &str { "balanced_bst_median_bst" }
    fn name(&self) -> &str { "Median of BST" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given a BST, find the median value. If the tree has an even number of nodes, \
         return the average of the two middle values.\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 1000\n\
         - The tree is a valid BST"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=20);
            let tree = random_bst(&mut rng, n, -200, 200);
            TestCase { data: Box::new(MedianBSTTest { tree }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MedianBSTTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let in_order = inorder(&arena, root);
        let n = in_order.len();
        let expected = if n % 2 == 1 {
            in_order[n / 2] as f64
        } else {
            (in_order[n / 2 - 1] as f64 + in_order[n / 2] as f64) / 2.0
        };
        let actual = solutions::median_bst(&t.tree);
        let is_correct = (expected - actual).abs() < 1e-5;
        SolutionResult {
            is_correct,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected:.5}"),
            actual: format!("{actual:.5}"),
        }
    }
}

// ── Hard 5: Rank from Stream ────────────────────────────────────────────

struct RankFromStream;
struct RankStreamTest { stream: Vec<i32>, queries: Vec<i32> }

impl Problem for RankFromStream {
    fn id(&self) -> &str { "balanced_bst_rank_from_stream" }
    fn name(&self) -> &str { "Rank from Stream" }
    fn topic(&self) -> &str { "balanced_bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "You are reading integers from a data stream. After reading all integers, \
         answer queries: for each query value x, return the rank of x, which is the \
         number of elements in the stream that are less than or equal to x.\n\n\
         Constraints:\n\
         - 1 <= stream.len() <= 1000\n\
         - 1 <= queries.len() <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let stream: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let q = rng.random_range(1..=10);
            let queries: Vec<i32> = (0..q).map(|_| rng.random_range(-120..=120)).collect();
            TestCase { data: Box::new(RankStreamTest { stream, queries }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RankStreamTest>().unwrap();
        let expected: Vec<i32> = t.queries.iter().map(|&q| {
            t.stream.iter().filter(|&&v| v <= q).count() as i32
        }).collect();
        let actual = solutions::rank_from_stream(&t.stream, &t.queries);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("stream={:?}, queries={:?}", t.stream, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}
