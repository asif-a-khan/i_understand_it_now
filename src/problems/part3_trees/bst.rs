use rand::Rng;

use crate::problems::helpers::{
    build_tree, inorder, random_bst, random_unique_vec, tree_to_level_order, TreeNode,
};
use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part3_trees::bst as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy
        Box::new(BstSearch),
        Box::new(BstMinimum),
        Box::new(BstIsValid),
        Box::new(BstRangeSum),
        Box::new(BstSortedArrayToBst),
        // Medium
        Box::new(BstInsert),
        Box::new(BstDelete),
        Box::new(BstKthSmallest),
        Box::new(BstInorderSuccessor),
        Box::new(BstLca),
        // Hard
        Box::new(BstRecover),
        Box::new(BstCountNodesInRange),
        Box::new(BstFromPreorder),
        Box::new(BstIterator),
        Box::new(BstMergeTwo),
    ]
}

// ── Easy 1: Search in BST ─────────────────────────────────────────────

struct BstSearch;
struct BstSearchTest {
    tree: Vec<Option<i32>>,
    target: i32,
}

impl Problem for BstSearch {
    fn id(&self) -> &str { "bst_search" }
    fn name(&self) -> &str { "Search in a Binary Search Tree" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a BST (level-order) and a target value, return true if the value \
         exists in the BST, false otherwise.\n\n\
         Example: tree=[4,2,7,1,3], target=2 -> true\n\
         Example: tree=[4,2,7,1,3], target=5 -> false\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 100\n\
         - All values are unique."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = Vec::new();
        // Tests where target exists
        for _ in 0..5 {
            let size = rng.random_range(1..=20);
            let tree = random_bst(&mut rng, size, -100, 100);
            let (arena, root) = build_tree(&tree);
            let vals = inorder(&arena, root);
            let target = vals[rng.random_range(0..vals.len())];
            tests.push(TestCase { data: Box::new(BstSearchTest { tree, target }) });
        }
        // Tests where target does not exist
        for _ in 0..5 {
            let size = rng.random_range(1..=20);
            let tree = random_bst(&mut rng, size, -100, 100);
            let (arena, root) = build_tree(&tree);
            let vals: std::collections::HashSet<i32> = inorder(&arena, root).into_iter().collect();
            let mut target = rng.random_range(-150..=150);
            while vals.contains(&target) { target = rng.random_range(-150..=150); }
            tests.push(TestCase { data: Box::new(BstSearchTest { tree, target }) });
        }
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstSearchTest>().unwrap();
        let expected = ref_bst_search(&t.tree, t.target);
        let actual = solutions::bst_search(&t.tree, t.target);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}, target={}", t.tree, t.target),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_bst_search(tree: &[Option<i32>], target: i32) -> bool {
    let (arena, root) = build_tree(tree);
    fn search(arena: &[TreeNode], node: Option<usize>, target: i32) -> bool {
        let Some(idx) = node else { return false };
        if arena[idx].val == target { return true; }
        if target < arena[idx].val {
            search(arena, arena[idx].left, target)
        } else {
            search(arena, arena[idx].right, target)
        }
    }
    search(&arena, root, target)
}

// ── Easy 2: BST Minimum ──────────────────────────────────────────────

struct BstMinimum;
struct BstMinimumTest {
    tree: Vec<Option<i32>>,
}

impl Problem for BstMinimum {
    fn id(&self) -> &str { "bst_minimum" }
    fn name(&self) -> &str { "Minimum Value in BST" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a BST (level-order), return the minimum value in the tree.\n\n\
         The minimum value is always the leftmost node.\n\n\
         Example: tree=[4,2,7,1,3] -> 1\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 100\n\
         - All values are unique."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let size = rng.random_range(1..=25);
            let tree = random_bst(&mut rng, size, -200, 200);
            TestCase { data: Box::new(BstMinimumTest { tree }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstMinimumTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let vals = inorder(&arena, root);
        let expected = vals[0];
        let actual = solutions::bst_minimum(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 3: Validate BST ─────────────────────────────────────────────

struct BstIsValid;
struct BstIsValidTest {
    tree: Vec<Option<i32>>,
}

impl Problem for BstIsValid {
    fn id(&self) -> &str { "bst_is_valid" }
    fn name(&self) -> &str { "Validate Binary Search Tree" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a binary tree (level-order), determine if it is a valid BST.\n\n\
         A valid BST has:\n\
         - Left subtree values strictly less than the node.\n\
         - Right subtree values strictly greater than the node.\n\
         - Both subtrees are also valid BSTs.\n\n\
         Example: [2,1,3] -> true\n\
         Example: [5,1,4,None,None,3,6] -> false (4 < 5 but is on the right)\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = Vec::new();
        // Valid BSTs
        for _ in 0..5 {
            let size = rng.random_range(1..=20);
            let tree = random_bst(&mut rng, size, -100, 100);
            tests.push(TestCase { data: Box::new(BstIsValidTest { tree }) });
        }
        // Invalid BSTs (random trees that are likely not BSTs)
        for _ in 0..3 {
            let size = rng.random_range(2..=15);
            let tree = crate::problems::helpers::random_tree(&mut rng, size, -20, 20);
            tests.push(TestCase { data: Box::new(BstIsValidTest { tree }) });
        }
        // Tricky invalid: [5,4,6,None,None,3,7] -- 3 < 5 but is in right subtree
        tests.push(TestCase { data: Box::new(BstIsValidTest {
            tree: vec![Some(5), Some(4), Some(6), None, None, Some(3), Some(7)],
        }) });
        // Single node (valid)
        tests.push(TestCase { data: Box::new(BstIsValidTest { tree: vec![Some(1)] }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstIsValidTest>().unwrap();
        let expected = ref_is_valid_bst(&t.tree);
        let actual = solutions::bst_is_valid(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_is_valid_bst(tree: &[Option<i32>]) -> bool {
    let (arena, root) = build_tree(tree);
    fn validate(arena: &[TreeNode], node: Option<usize>, min: i64, max: i64) -> bool {
        let Some(idx) = node else { return true };
        let val = arena[idx].val as i64;
        if val <= min || val >= max { return false; }
        validate(arena, arena[idx].left, min, val) && validate(arena, arena[idx].right, val, max)
    }
    validate(&arena, root, i64::MIN, i64::MAX)
}

// ── Easy 4: Range Sum of BST ─────────────────────────────────────────

struct BstRangeSum;
struct BstRangeSumTest {
    tree: Vec<Option<i32>>,
    low: i32,
    high: i32,
}

impl Problem for BstRangeSum {
    fn id(&self) -> &str { "bst_range_sum" }
    fn name(&self) -> &str { "Range Sum of BST" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a BST and two integers `low` and `high`, return the sum of values of \
         all nodes with values in the inclusive range [low, high].\n\n\
         Example: tree=[10,5,15,3,7,None,18], low=7, high=15 -> 32 (7+10+15)\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 100\n\
         - All values are unique.\n\
         - low <= high"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let size = rng.random_range(1..=25);
            let tree = random_bst(&mut rng, size, -100, 100);
            let a = rng.random_range(-120..=120);
            let b = rng.random_range(-120..=120);
            let (low, high) = if a <= b { (a, b) } else { (b, a) };
            TestCase { data: Box::new(BstRangeSumTest { tree, low, high }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstRangeSumTest>().unwrap();
        let expected = ref_range_sum(&t.tree, t.low, t.high);
        let actual = solutions::bst_range_sum(&t.tree, t.low, t.high);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}, low={}, high={}", t.tree, t.low, t.high),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_range_sum(tree: &[Option<i32>], low: i32, high: i32) -> i32 {
    let (arena, root) = build_tree(tree);
    fn helper(arena: &[TreeNode], node: Option<usize>, low: i32, high: i32) -> i32 {
        let Some(idx) = node else { return 0 };
        let val = arena[idx].val;
        let mut sum = 0;
        if val >= low && val <= high { sum += val; }
        if val > low { sum += helper(arena, arena[idx].left, low, high); }
        if val < high { sum += helper(arena, arena[idx].right, low, high); }
        sum
    }
    helper(&arena, root, low, high)
}

// ── Easy 5: Sorted Array to BST ──────────────────────────────────────

struct BstSortedArrayToBst;
struct BstSortedArrayToBstTest {
    nums: Vec<i32>,
}

impl Problem for BstSortedArrayToBst {
    fn id(&self) -> &str { "bst_sorted_array_to_bst" }
    fn name(&self) -> &str { "Convert Sorted Array to Binary Search Tree" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a sorted array of unique integers, convert it to a height-balanced BST.\n\n\
         A height-balanced BST is one where the depth of the two subtrees of every node \
         never differs by more than one.\n\n\
         Return the tree as level-order Vec<Option<i32>>.\n\n\
         Example: [-10,-3,0,5,9] -> [0,-3,9,-10,None,5] (one valid answer)\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 100\n\
         - Values are sorted in strictly increasing order."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..9).map(|_| {
            let size = rng.random_range(1..=25);
            let mut nums = random_unique_vec(&mut rng, size, -200, 200);
            nums.sort();
            TestCase { data: Box::new(BstSortedArrayToBstTest { nums }) }
        }).collect();
        tests.push(TestCase { data: Box::new(BstSortedArrayToBstTest { nums: vec![0] }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstSortedArrayToBstTest>().unwrap();
        let actual = solutions::bst_sorted_array_to_bst(&t.nums);
        // Validate: (1) inorder matches sorted input, (2) is valid BST, (3) is balanced
        let (arena, root) = build_tree(&actual);
        let in_vals = inorder(&arena, root);
        let is_valid = ref_is_valid_bst(&actual);
        let is_balanced = ref_is_balanced(&arena, root);
        let correct = in_vals == t.nums && is_valid && is_balanced;
        SolutionResult {
            is_correct: correct,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("valid balanced BST with inorder={:?}", t.nums),
            actual: format!("tree={actual:?}, inorder={in_vals:?}, valid={is_valid}, balanced={is_balanced}"),
        }
    }
}

fn ref_is_balanced(arena: &[TreeNode], root: Option<usize>) -> bool {
    fn height(arena: &[TreeNode], node: Option<usize>) -> i32 {
        let Some(idx) = node else { return 0 };
        let l = height(arena, arena[idx].left);
        let r = height(arena, arena[idx].right);
        if l == -1 || r == -1 || (l - r).abs() > 1 { return -1; }
        1 + l.max(r)
    }
    height(arena, root) != -1
}

// ── Medium 1: Insert into BST ─────────────────────────────────────────

struct BstInsert;
struct BstInsertTest {
    tree: Vec<Option<i32>>,
    val: i32,
}

impl Problem for BstInsert {
    fn id(&self) -> &str { "bst_insert" }
    fn name(&self) -> &str { "Insert into a Binary Search Tree" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a BST and a value to insert, insert the value into the BST and return \
         the updated tree as level-order. The value does not already exist in the tree.\n\n\
         Any valid BST after insertion is accepted.\n\n\
         Example: tree=[4,2,7,1,3], val=5 -> [4,2,7,1,3,5]\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 100\n\
         - All values are unique.\n\
         - val does not exist in the tree."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let size = rng.random_range(1..=20);
            let tree = random_bst(&mut rng, size, -100, 100);
            let (arena, root) = build_tree(&tree);
            let existing: std::collections::HashSet<i32> = inorder(&arena, root).into_iter().collect();
            let mut val = rng.random_range(-120..=120);
            while existing.contains(&val) { val = rng.random_range(-120..=120); }
            TestCase { data: Box::new(BstInsertTest { tree, val }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstInsertTest>().unwrap();
        let actual = solutions::bst_insert(&t.tree, t.val);
        // Validate: (1) inorder contains all original values + new value, (2) is valid BST
        let (arena_orig, root_orig) = build_tree(&t.tree);
        let mut expected_inorder = inorder(&arena_orig, root_orig);
        expected_inorder.push(t.val);
        expected_inorder.sort();

        let (arena_actual, root_actual) = build_tree(&actual);
        let actual_inorder = inorder(&arena_actual, root_actual);
        let is_valid = ref_is_valid_bst(&actual);
        let correct = actual_inorder == expected_inorder && is_valid;
        SolutionResult {
            is_correct: correct,
            input_description: format!("tree={:?}, val={}", t.tree, t.val),
            expected: format!("valid BST with inorder={expected_inorder:?}"),
            actual: format!("tree={actual:?}, inorder={actual_inorder:?}, valid={is_valid}"),
        }
    }
}

// ── Medium 2: Delete from BST ─────────────────────────────────────────

struct BstDelete;
struct BstDeleteTest {
    tree: Vec<Option<i32>>,
    val: i32,
}

impl Problem for BstDelete {
    fn id(&self) -> &str { "bst_delete" }
    fn name(&self) -> &str { "Delete Node in a BST" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a BST and a value, delete the node with that value and return the \
         resulting BST as level-order. If the value does not exist, return the tree unchanged.\n\n\
         When deleting a node with two children, replace with the inorder successor.\n\n\
         Example: tree=[5,3,6,2,4,None,7], val=3 -> [5,4,6,2,None,None,7]\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 100\n\
         - All values are unique."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = Vec::new();
        // Delete existing values
        for _ in 0..7 {
            let size = rng.random_range(1..=20);
            let tree = random_bst(&mut rng, size, -100, 100);
            let (arena, root) = build_tree(&tree);
            let vals = inorder(&arena, root);
            let val = vals[rng.random_range(0..vals.len())];
            tests.push(TestCase { data: Box::new(BstDeleteTest { tree, val }) });
        }
        // Delete non-existing value
        for _ in 0..3 {
            let size = rng.random_range(1..=15);
            let tree = random_bst(&mut rng, size, -100, 100);
            let (arena, root) = build_tree(&tree);
            let existing: std::collections::HashSet<i32> = inorder(&arena, root).into_iter().collect();
            let mut val = rng.random_range(-150..=150);
            while existing.contains(&val) { val = rng.random_range(-150..=150); }
            tests.push(TestCase { data: Box::new(BstDeleteTest { tree, val }) });
        }
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstDeleteTest>().unwrap();
        let actual = solutions::bst_delete(&t.tree, t.val);
        // Validate: (1) inorder is original minus deleted val, (2) valid BST
        let (arena_orig, root_orig) = build_tree(&t.tree);
        let orig_inorder = inorder(&arena_orig, root_orig);
        let expected_inorder: Vec<i32> = orig_inorder.into_iter().filter(|&v| v != t.val).collect();

        let (arena_actual, root_actual) = build_tree(&actual);
        let actual_inorder = inorder(&arena_actual, root_actual);
        let is_valid = ref_is_valid_bst(&actual);
        let correct = actual_inorder == expected_inorder && is_valid;
        SolutionResult {
            is_correct: correct,
            input_description: format!("tree={:?}, val={}", t.tree, t.val),
            expected: format!("valid BST with inorder={expected_inorder:?}"),
            actual: format!("tree={actual:?}, inorder={actual_inorder:?}, valid={is_valid}"),
        }
    }
}

// ── Medium 3: Kth Smallest Element in BST ─────────────────────────────

struct BstKthSmallest;
struct BstKthSmallestTest {
    tree: Vec<Option<i32>>,
    k: usize,
}

impl Problem for BstKthSmallest {
    fn id(&self) -> &str { "bst_kth_smallest" }
    fn name(&self) -> &str { "Kth Smallest Element in a BST" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a BST and an integer k, return the kth smallest value in the tree \
         (1-indexed).\n\n\
         Example: tree=[3,1,4,None,2], k=1 -> 1\n\
         Example: tree=[5,3,6,2,4,None,None,1], k=3 -> 3\n\n\
         Constraints:\n\
         - 1 <= k <= number of nodes <= 100\n\
         - All values are unique."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let size = rng.random_range(1..=25);
            let tree = random_bst(&mut rng, size, -200, 200);
            let k = rng.random_range(1..=size);
            TestCase { data: Box::new(BstKthSmallestTest { tree, k }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstKthSmallestTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let vals = inorder(&arena, root);
        let expected = vals[t.k - 1];
        let actual = solutions::bst_kth_smallest(&t.tree, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}, k={}", t.tree, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 4: Inorder Successor in BST ────────────────────────────────

struct BstInorderSuccessor;
struct BstInorderSuccessorTest {
    tree: Vec<Option<i32>>,
    val: i32,
}

impl Problem for BstInorderSuccessor {
    fn id(&self) -> &str { "bst_inorder_successor" }
    fn name(&self) -> &str { "Inorder Successor in BST" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a BST and a node value, find the inorder successor of that node \
         (the node with the smallest value greater than the given value).\n\n\
         Return Some(value) if a successor exists, or None if the node is the largest.\n\n\
         Example: tree=[5,3,6,2,4,None,None,1], val=4 -> Some(5)\n\
         Example: tree=[2,1,3], val=3 -> None\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 100\n\
         - All values are unique.\n\
         - val exists in the tree."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let size = rng.random_range(1..=25);
            let tree = random_bst(&mut rng, size, -200, 200);
            let (arena, root) = build_tree(&tree);
            let vals = inorder(&arena, root);
            let val = vals[rng.random_range(0..vals.len())];
            TestCase { data: Box::new(BstInorderSuccessorTest { tree, val }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstInorderSuccessorTest>().unwrap();
        let expected = ref_inorder_successor(&t.tree, t.val);
        let actual = solutions::bst_inorder_successor(&t.tree, t.val);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}, val={}", t.tree, t.val),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_inorder_successor(tree: &[Option<i32>], val: i32) -> Option<i32> {
    let (arena, root) = build_tree(tree);
    let vals = inorder(&arena, root);
    let pos = vals.iter().position(|&v| v == val)?;
    if pos + 1 < vals.len() { Some(vals[pos + 1]) } else { None }
}

// ── Medium 5: Lowest Common Ancestor in BST ───────────────────────────

struct BstLca;
struct BstLcaTest {
    tree: Vec<Option<i32>>,
    p: i32,
    q: i32,
}

impl Problem for BstLca {
    fn id(&self) -> &str { "bst_lca" }
    fn name(&self) -> &str { "Lowest Common Ancestor of a BST" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a BST and two node values p and q, find the lowest common ancestor.\n\n\
         Use the BST property: if both p and q are less than the current node, the LCA \
         is in the left subtree; if both are greater, it is in the right subtree; \
         otherwise the current node is the LCA.\n\n\
         Example: tree=[6,2,8,0,4,7,9,None,None,3,5], p=2, q=8 -> 6\n\
         Example: tree=[6,2,8,0,4,7,9,None,None,3,5], p=2, q=4 -> 2\n\n\
         Constraints:\n\
         - All values are unique.\n\
         - p != q, both exist in the tree."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let size = rng.random_range(2..=25);
            let tree = random_bst(&mut rng, size, -200, 200);
            let (arena, root) = build_tree(&tree);
            let vals = inorder(&arena, root);
            let pi = rng.random_range(0..vals.len());
            let mut qi = rng.random_range(0..vals.len());
            while qi == pi { qi = rng.random_range(0..vals.len()); }
            let p = vals[pi];
            let q = vals[qi];
            TestCase { data: Box::new(BstLcaTest { tree, p, q }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstLcaTest>().unwrap();
        let expected = ref_bst_lca(&t.tree, t.p, t.q);
        let actual = solutions::bst_lca(&t.tree, t.p, t.q);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}, p={}, q={}", t.tree, t.p, t.q),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_bst_lca(tree: &[Option<i32>], p: i32, q: i32) -> i32 {
    let (arena, root) = build_tree(tree);
    fn lca(arena: &[TreeNode], node: Option<usize>, p: i32, q: i32) -> i32 {
        let idx = node.unwrap();
        let val = arena[idx].val;
        if p < val && q < val {
            lca(arena, arena[idx].left, p, q)
        } else if p > val && q > val {
            lca(arena, arena[idx].right, p, q)
        } else {
            val
        }
    }
    lca(&arena, root, p, q)
}

// ── Hard 1: Recover BST ──────────────────────────────────────────────

struct BstRecover;
struct BstRecoverTest {
    tree: Vec<Option<i32>>,
}

impl Problem for BstRecover {
    fn id(&self) -> &str { "bst_recover" }
    fn name(&self) -> &str { "Recover Binary Search Tree" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Two nodes of a BST were swapped by mistake. Recover the tree without \
         changing its structure -- just swap the two values back.\n\n\
         Return the corrected tree as level-order.\n\n\
         Example: [1,3,None,None,2] -> [3,1,None,None,2] (1 and 3 were swapped)\n\
         Example: [3,1,4,None,None,2] -> [2,1,4,None,None,3] (2 and 3 were swapped)\n\n\
         Constraints:\n\
         - 2 <= number of nodes <= 100\n\
         - All values are unique."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let size = rng.random_range(2..=20);
            let valid_tree = random_bst(&mut rng, size, -200, 200);
            let (mut arena, root) = build_tree(&valid_tree);
            // Swap two random nodes' values
            let vals = inorder(&arena, root);
            let i = rng.random_range(0..vals.len());
            let mut j = rng.random_range(0..vals.len());
            while j == i { j = rng.random_range(0..vals.len()); }
            // Find the arena indices for these two inorder positions
            let mut inorder_indices = Vec::new();
            fn collect_inorder_indices(arena: &[TreeNode], node: Option<usize>, indices: &mut Vec<usize>) {
                if let Some(idx) = node {
                    collect_inorder_indices(arena, arena[idx].left, indices);
                    indices.push(idx);
                    collect_inorder_indices(arena, arena[idx].right, indices);
                }
            }
            collect_inorder_indices(&arena, root, &mut inorder_indices);
            let ai = inorder_indices[i];
            let aj = inorder_indices[j];
            // Swap values
            let tmp = arena[ai].val;
            arena[ai].val = arena[aj].val;
            arena[aj].val = tmp;
            let broken_tree = tree_to_level_order(&arena, root);
            TestCase { data: Box::new(BstRecoverTest { tree: broken_tree }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstRecoverTest>().unwrap();
        let actual = solutions::bst_recover(&t.tree);
        // Validate: result must be a valid BST with the same structure
        let is_valid = ref_is_valid_bst(&actual);
        // Same structure: both trees should have same shape
        let (orig_arena, orig_root) = build_tree(&t.tree);
        let (act_arena, act_root) = build_tree(&actual);
        let orig_vals: std::collections::HashSet<i32> = inorder(&orig_arena, orig_root).into_iter().collect();
        let act_vals: std::collections::HashSet<i32> = inorder(&act_arena, act_root).into_iter().collect();
        let same_vals = orig_vals == act_vals;
        let correct = is_valid && same_vals;
        SolutionResult {
            is_correct: correct,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("valid BST with same values, valid={}, vals_match={}", true, true),
            actual: format!("tree={actual:?}, valid={is_valid}, vals_match={same_vals}"),
        }
    }
}

// ── Hard 2: Count Nodes in Range ──────────────────────────────────────

struct BstCountNodesInRange;
struct BstCountNodesInRangeTest {
    tree: Vec<Option<i32>>,
    lo: i32,
    hi: i32,
}

impl Problem for BstCountNodesInRange {
    fn id(&self) -> &str { "bst_count_nodes_in_range" }
    fn name(&self) -> &str { "Count BST Nodes in Range" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given a BST and two integers lo and hi, count how many nodes have values \
         in the inclusive range [lo, hi].\n\n\
         Use the BST property to prune branches efficiently.\n\n\
         Example: tree=[10,5,15,3,7,13,18,1,None,6], lo=6, hi=13 -> 4 (6,7,10,13)\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 100\n\
         - All values are unique.\n\
         - lo <= hi"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let size = rng.random_range(1..=30);
            let tree = random_bst(&mut rng, size, -200, 200);
            let a = rng.random_range(-250..=250);
            let b = rng.random_range(-250..=250);
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            TestCase { data: Box::new(BstCountNodesInRangeTest { tree, lo, hi }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstCountNodesInRangeTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let vals = inorder(&arena, root);
        let expected = vals.iter().filter(|&&v| v >= t.lo && v <= t.hi).count() as i32;
        let actual = solutions::bst_count_nodes_in_range(&t.tree, t.lo, t.hi);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}, lo={}, hi={}", t.tree, t.lo, t.hi),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Hard 3: Construct BST from Preorder ───────────────────────────────

struct BstFromPreorder;
struct BstFromPreorderTest {
    preorder: Vec<i32>,
}

impl Problem for BstFromPreorder {
    fn id(&self) -> &str { "bst_from_preorder" }
    fn name(&self) -> &str { "Construct BST from Preorder Traversal" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given a preorder traversal of a BST (all values unique), construct the BST \
         and return its level-order representation.\n\n\
         Example: [8,5,1,7,10,12] -> [8,5,10,1,7,None,12]\n\n\
         Constraints:\n\
         - 1 <= len <= 100\n\
         - All values are unique."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let size = rng.random_range(1..=20);
            let tree = random_bst(&mut rng, size, -200, 200);
            let preorder = ref_preorder_traversal(&tree);
            TestCase { data: Box::new(BstFromPreorderTest { preorder }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstFromPreorderTest>().unwrap();
        let expected = ref_bst_from_preorder(&t.preorder);
        let actual = solutions::bst_from_preorder(&t.preorder);
        // Validate: same preorder and valid BST
        let e_pre = ref_preorder_traversal(&expected);
        let a_pre = ref_preorder_traversal(&actual);
        let is_valid = ref_is_valid_bst(&actual);
        let correct = e_pre == a_pre && is_valid;
        SolutionResult {
            is_correct: correct,
            input_description: format!("preorder={:?}", t.preorder),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_preorder_traversal(tree: &[Option<i32>]) -> Vec<i32> {
    let (arena, root) = build_tree(tree);
    let mut result = Vec::new();
    fn helper(arena: &[TreeNode], node: Option<usize>, result: &mut Vec<i32>) {
        if let Some(idx) = node {
            result.push(arena[idx].val);
            helper(arena, arena[idx].left, result);
            helper(arena, arena[idx].right, result);
        }
    }
    helper(&arena, root, &mut result);
    result
}

fn ref_bst_from_preorder(preorder: &[i32]) -> Vec<Option<i32>> {
    if preorder.is_empty() { return vec![]; }
    let mut arena = Vec::new();
    let mut idx = 0;
    fn build(preorder: &[i32], idx: &mut usize, min: i64, max: i64, arena: &mut Vec<TreeNode>) -> Option<usize> {
        if *idx >= preorder.len() { return None; }
        let val = preorder[*idx] as i64;
        if val <= min || val >= max { return None; }
        let node_idx = arena.len();
        arena.push(TreeNode { val: preorder[*idx], left: None, right: None });
        *idx += 1;
        let left = build(preorder, idx, min, val, arena);
        let right = build(preorder, idx, val, max, arena);
        arena[node_idx].left = left;
        arena[node_idx].right = right;
        Some(node_idx)
    }
    let root = build(preorder, &mut idx, i64::MIN, i64::MAX, &mut arena);
    tree_to_level_order(&arena, root)
}

// ── Hard 4: BST Iterator ─────────────────────────────────────────────

struct BstIterator;
struct BstIteratorTest {
    tree: Vec<Option<i32>>,
}

impl Problem for BstIterator {
    fn id(&self) -> &str { "bst_iterator" }
    fn name(&self) -> &str { "Binary Search Tree Iterator" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Implement a BST iterator that returns elements in inorder (ascending) sequence.\n\n\
         For this simplified version, given a BST as level-order, return its complete \
         inorder traversal as Vec<i32>.\n\n\
         In the full problem, the iterator supports next() and hasNext() with O(h) memory \
         using a stack-based approach.\n\n\
         Example: [7,3,15,None,None,9,20] -> [3,7,9,15,20]\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 100\n\
         - All values are unique."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let size = rng.random_range(1..=30);
            let tree = random_bst(&mut rng, size, -200, 200);
            TestCase { data: Box::new(BstIteratorTest { tree }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstIteratorTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let expected = inorder(&arena, root);
        let actual = solutions::bst_iterator(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 5: Merge Two BSTs ────────────────────────────────────────────

struct BstMergeTwo;
struct BstMergeTwoTest {
    tree1: Vec<Option<i32>>,
    tree2: Vec<Option<i32>>,
}

impl Problem for BstMergeTwo {
    fn id(&self) -> &str { "bst_merge_two" }
    fn name(&self) -> &str { "Merge Two BSTs" }
    fn topic(&self) -> &str { "bst" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given two BSTs, merge all their values into a single sorted array.\n\n\
         Use the BST property to do this efficiently: perform inorder traversal \
         on both trees (each gives a sorted array), then merge the two sorted arrays.\n\n\
         Example: tree1=[2,1,4], tree2=[3,0,5] -> [0,1,2,3,4,5]\n\n\
         Constraints:\n\
         - 0 <= nodes in each tree <= 100\n\
         - All values across both trees are unique."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let size1 = rng.random_range(0..=15);
            let size2 = rng.random_range(0..=15);
            let total = size1 + size2;
            if total == 0 {
                return TestCase { data: Box::new(BstMergeTwoTest {
                    tree1: vec![], tree2: vec![],
                }) };
            }
            let all_vals = random_unique_vec(&mut rng, total, -200, 200);
            let vals1: Vec<i32> = all_vals[..size1].to_vec();
            let vals2: Vec<i32> = all_vals[size1..].to_vec();
            let tree1 = if vals1.is_empty() {
                vec![]
            } else {
                let mut sorted1 = vals1.clone();
                sorted1.sort();
                ref_sorted_to_bst(&sorted1)
            };
            let tree2 = if vals2.is_empty() {
                vec![]
            } else {
                let mut sorted2 = vals2.clone();
                sorted2.sort();
                ref_sorted_to_bst(&sorted2)
            };
            TestCase { data: Box::new(BstMergeTwoTest { tree1, tree2 }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BstMergeTwoTest>().unwrap();
        let (a1, r1) = build_tree(&t.tree1);
        let (a2, r2) = build_tree(&t.tree2);
        let mut expected = inorder(&a1, r1);
        expected.extend(inorder(&a2, r2));
        expected.sort();
        let actual = solutions::bst_merge_two(&t.tree1, &t.tree2);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree1={:?}, tree2={:?}", t.tree1, t.tree2),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_sorted_to_bst(sorted: &[i32]) -> Vec<Option<i32>> {
    if sorted.is_empty() { return vec![]; }
    let mut arena = Vec::new();
    fn build(sorted: &[i32], lo: usize, hi: usize, arena: &mut Vec<TreeNode>) -> Option<usize> {
        if lo > hi { return None; }
        let mid = lo + (hi - lo) / 2;
        let idx = arena.len();
        arena.push(TreeNode { val: sorted[mid], left: None, right: None });
        let left = if mid > lo { build(sorted, lo, mid - 1, arena) } else { None };
        let right = if mid < hi { build(sorted, mid + 1, hi, arena) } else { None };
        arena[idx].left = left;
        arena[idx].right = right;
        Some(idx)
    }
    let root = build(sorted, 0, sorted.len() - 1, &mut arena);
    tree_to_level_order(&arena, root)
}
