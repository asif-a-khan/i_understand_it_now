use rand::Rng;
use std::collections::VecDeque;

use crate::problems::helpers::{
    build_tree, inorder, random_tree, tree_to_level_order, TreeNode,
};
use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part3_trees::binary_trees as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy
        Box::new(MaxDepth),
        Box::new(InorderTraversal),
        Box::new(IsSymmetric),
        Box::new(IsSameTree),
        Box::new(InvertTree),
        // Medium
        Box::new(LevelOrder),
        Box::new(ZigzagLevelOrder),
        Box::new(RightSideView),
        Box::new(FlattenToLinkedList),
        Box::new(ConstructFromPreorderInorder),
        // Hard
        Box::new(MaxPathSum),
        Box::new(SerializeDeserialize),
        Box::new(LowestCommonAncestor),
        Box::new(Diameter),
        Box::new(CountCompleteTreeNodes),
    ]
}

// ── Easy 1: Maximum Depth of Binary Tree ──────────────────────────────

struct MaxDepth;
struct MaxDepthTest {
    tree: Vec<Option<i32>>,
}

impl Problem for MaxDepth {
    fn id(&self) -> &str { "binary_trees_max_depth" }
    fn name(&self) -> &str { "Maximum Depth of Binary Tree" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a binary tree (level-order representation), return its maximum depth.\n\n\
         The maximum depth is the number of nodes along the longest path from the \
         root node down to the farthest leaf node.\n\n\
         Example: [3, 9, 20, None, None, 15, 7] -> 3\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 100\n\
         - -100 <= Node.val <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8).map(|_| {
            let size = rng.random_range(1..=20);
            let tree = random_tree(&mut rng, size, -100, 100);
            TestCase { data: Box::new(MaxDepthTest { tree }) }
        }).collect();
        // Edge case: empty tree
        tests.push(TestCase { data: Box::new(MaxDepthTest { tree: vec![] }) });
        // Edge case: single node
        tests.push(TestCase { data: Box::new(MaxDepthTest { tree: vec![Some(1)] }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxDepthTest>().unwrap();
        let expected = ref_max_depth(&t.tree);
        let actual = solutions::max_depth(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_depth(tree: &[Option<i32>]) -> i32 {
    let (arena, root) = build_tree(tree);
    fn depth(arena: &[TreeNode], node: Option<usize>) -> i32 {
        match node {
            None => 0,
            Some(idx) => {
                let l = depth(arena, arena[idx].left);
                let r = depth(arena, arena[idx].right);
                1 + l.max(r)
            }
        }
    }
    depth(&arena, root)
}

// ── Easy 2: Inorder Traversal ─────────────────────────────────────────

struct InorderTraversal;
struct InorderTraversalTest {
    tree: Vec<Option<i32>>,
}

impl Problem for InorderTraversal {
    fn id(&self) -> &str { "binary_trees_inorder_traversal" }
    fn name(&self) -> &str { "Binary Tree Inorder Traversal" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a binary tree (level-order representation), return its inorder traversal \
         as a Vec<i32>.\n\n\
         Inorder: left subtree, root, right subtree.\n\n\
         Example: [1, None, 2, 3] -> [1, 3, 2]\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 100\n\
         - -100 <= Node.val <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8).map(|_| {
            let size = rng.random_range(1..=20);
            let tree = random_tree(&mut rng, size, -100, 100);
            TestCase { data: Box::new(InorderTraversalTest { tree }) }
        }).collect();
        tests.push(TestCase { data: Box::new(InorderTraversalTest { tree: vec![] }) });
        tests.push(TestCase { data: Box::new(InorderTraversalTest { tree: vec![Some(42)] }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<InorderTraversalTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let expected = inorder(&arena, root);
        let actual = solutions::inorder_traversal(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 3: Symmetric Tree ────────────────────────────────────────────

struct IsSymmetric;
struct IsSymmetricTest {
    tree: Vec<Option<i32>>,
}

impl Problem for IsSymmetric {
    fn id(&self) -> &str { "binary_trees_is_symmetric" }
    fn name(&self) -> &str { "Symmetric Tree" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a binary tree, check whether it is a mirror of itself (symmetric around \
         its center).\n\n\
         Example: [1, 2, 2, 3, 4, 4, 3] -> true\n\
         Example: [1, 2, 2, None, 3, None, 3] -> false\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 100\n\
         - -100 <= Node.val <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = Vec::new();
        // Generate symmetric trees
        for _ in 0..4 {
            let tree = ref_random_symmetric_tree(&mut rng);
            tests.push(TestCase { data: Box::new(IsSymmetricTest { tree }) });
        }
        // Generate random (likely non-symmetric) trees
        for _ in 0..4 {
            let size = rng.random_range(2..=15);
            let tree = random_tree(&mut rng, size, -10, 10);
            tests.push(TestCase { data: Box::new(IsSymmetricTest { tree }) });
        }
        // Edge cases
        tests.push(TestCase { data: Box::new(IsSymmetricTest { tree: vec![] }) });
        tests.push(TestCase { data: Box::new(IsSymmetricTest { tree: vec![Some(1)] }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IsSymmetricTest>().unwrap();
        let expected = ref_is_symmetric(&t.tree);
        let actual = solutions::is_symmetric(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_random_symmetric_tree(rng: &mut impl Rng) -> Vec<Option<i32>> {
    // Build a symmetric tree by mirroring decisions
    let depth = rng.random_range(1..=4);
    let root_val = rng.random_range(-10..=10);
    let mut result = vec![Some(root_val)];
    // Build level by level, mirroring left to right
    let mut level_size = 1;
    for _ in 1..depth {
        let mut left_half = Vec::new();
        let parent_count = level_size;
        for p in 0..parent_count {
            let parent_idx = result.len() - parent_count + p;
            if result[parent_idx].is_some() {
                let has_child = rng.random_range(0..3) != 0;
                if has_child {
                    left_half.push(Some(rng.random_range(-10..=10)));
                } else {
                    left_half.push(None);
                }
            } else {
                left_half.push(None);
            }
        }
        // Mirror: left half children, then right half children (reversed vals)
        let half = left_half.len();
        let mut level = Vec::new();
        for i in 0..half {
            level.push(left_half[i]);
        }
        for i in (0..half).rev() {
            level.push(left_half[i]);
        }
        level_size = level.len();
        result.extend(level);
    }
    // Trim trailing Nones
    while result.last() == Some(&None) {
        result.pop();
    }
    result
}

fn ref_is_symmetric(tree: &[Option<i32>]) -> bool {
    let (arena, root) = build_tree(tree);
    fn is_mirror(arena: &[TreeNode], a: Option<usize>, b: Option<usize>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(ai), Some(bi)) => {
                arena[ai].val == arena[bi].val
                    && is_mirror(arena, arena[ai].left, arena[bi].right)
                    && is_mirror(arena, arena[ai].right, arena[bi].left)
            }
            _ => false,
        }
    }
    match root {
        None => true,
        Some(r) => is_mirror(&arena, arena[r].left, arena[r].right),
    }
}

// ── Easy 4: Same Tree ─────────────────────────────────────────────────

struct IsSameTree;
struct IsSameTreeTest {
    tree1: Vec<Option<i32>>,
    tree2: Vec<Option<i32>>,
}

impl Problem for IsSameTree {
    fn id(&self) -> &str { "binary_trees_is_same_tree" }
    fn name(&self) -> &str { "Same Tree" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given two binary trees (level-order), check if they are structurally identical \
         and have the same node values.\n\n\
         Example: p=[1,2,3], q=[1,2,3] -> true\n\
         Example: p=[1,2], q=[1,None,2] -> false\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 100\n\
         - -100 <= Node.val <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = Vec::new();
        // Same trees
        for _ in 0..4 {
            let size = rng.random_range(1..=15);
            let tree = random_tree(&mut rng, size, -50, 50);
            tests.push(TestCase {
                data: Box::new(IsSameTreeTest { tree1: tree.clone(), tree2: tree }),
            });
        }
        // Different trees
        for _ in 0..4 {
            let size1 = rng.random_range(1..=15);
            let size2 = rng.random_range(1..=15);
            let tree1 = random_tree(&mut rng, size1, -50, 50);
            let tree2 = random_tree(&mut rng, size2, -50, 50);
            tests.push(TestCase {
                data: Box::new(IsSameTreeTest { tree1, tree2 }),
            });
        }
        // Edge cases
        tests.push(TestCase {
            data: Box::new(IsSameTreeTest { tree1: vec![], tree2: vec![] }),
        });
        tests.push(TestCase {
            data: Box::new(IsSameTreeTest { tree1: vec![Some(1)], tree2: vec![] }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IsSameTreeTest>().unwrap();
        let expected = ref_is_same_tree(&t.tree1, &t.tree2);
        let actual = solutions::is_same_tree(&t.tree1, &t.tree2);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("p={:?}, q={:?}", t.tree1, t.tree2),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_is_same_tree(tree1: &[Option<i32>], tree2: &[Option<i32>]) -> bool {
    let (a1, r1) = build_tree(tree1);
    let (a2, r2) = build_tree(tree2);
    fn same(a1: &[TreeNode], n1: Option<usize>, a2: &[TreeNode], n2: Option<usize>) -> bool {
        match (n1, n2) {
            (None, None) => true,
            (Some(i), Some(j)) => {
                a1[i].val == a2[j].val
                    && same(a1, a1[i].left, a2, a2[j].left)
                    && same(a1, a1[i].right, a2, a2[j].right)
            }
            _ => false,
        }
    }
    same(&a1, r1, &a2, r2)
}

// ── Easy 5: Invert Binary Tree ────────────────────────────────────────

struct InvertTree;
struct InvertTreeTest {
    tree: Vec<Option<i32>>,
}

impl Problem for InvertTree {
    fn id(&self) -> &str { "binary_trees_invert" }
    fn name(&self) -> &str { "Invert Binary Tree" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a binary tree (level-order), invert it (swap left and right subtrees \
         at every node) and return the result as a level-order representation.\n\n\
         Example: [4,2,7,1,3,6,9] -> [4,7,2,9,6,3,1]\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 100\n\
         - -100 <= Node.val <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8).map(|_| {
            let size = rng.random_range(1..=20);
            let tree = random_tree(&mut rng, size, -100, 100);
            TestCase { data: Box::new(InvertTreeTest { tree }) }
        }).collect();
        tests.push(TestCase { data: Box::new(InvertTreeTest { tree: vec![] }) });
        tests.push(TestCase { data: Box::new(InvertTreeTest { tree: vec![Some(1)] }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<InvertTreeTest>().unwrap();
        let expected = ref_invert(&t.tree);
        let actual = solutions::invert_tree(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_invert(tree: &[Option<i32>]) -> Vec<Option<i32>> {
    let (mut arena, root) = build_tree(tree);
    fn invert_helper(arena: &mut Vec<TreeNode>, node: Option<usize>) {
        if let Some(idx) = node {
            let left = arena[idx].left;
            let right = arena[idx].right;
            arena[idx].left = right;
            arena[idx].right = left;
            invert_helper(arena, left);
            invert_helper(arena, right);
        }
    }
    invert_helper(&mut arena, root);
    tree_to_level_order(&arena, root)
}

// ── Medium 1: Level Order Traversal ───────────────────────────────────

struct LevelOrder;
struct LevelOrderTest {
    tree: Vec<Option<i32>>,
}

impl Problem for LevelOrder {
    fn id(&self) -> &str { "binary_trees_level_order" }
    fn name(&self) -> &str { "Binary Tree Level Order Traversal" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a binary tree, return its level order traversal as Vec<Vec<i32>> \
         (i.e., from left to right, level by level).\n\n\
         Example: [3,9,20,None,None,15,7] -> [[3],[9,20],[15,7]]\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 100\n\
         - -100 <= Node.val <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8).map(|_| {
            let size = rng.random_range(1..=25);
            let tree = random_tree(&mut rng, size, -100, 100);
            TestCase { data: Box::new(LevelOrderTest { tree }) }
        }).collect();
        tests.push(TestCase { data: Box::new(LevelOrderTest { tree: vec![] }) });
        tests.push(TestCase { data: Box::new(LevelOrderTest { tree: vec![Some(1)] }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LevelOrderTest>().unwrap();
        let expected = ref_level_order(&t.tree);
        let actual = solutions::level_order(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_level_order(tree: &[Option<i32>]) -> Vec<Vec<i32>> {
    let (arena, root) = build_tree(tree);
    let mut result = Vec::new();
    let Some(r) = root else { return result };
    let mut queue = VecDeque::new();
    queue.push_back(r);
    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level = Vec::new();
        for _ in 0..level_size {
            let idx = queue.pop_front().unwrap();
            level.push(arena[idx].val);
            if let Some(l) = arena[idx].left { queue.push_back(l); }
            if let Some(r) = arena[idx].right { queue.push_back(r); }
        }
        result.push(level);
    }
    result
}

// ── Medium 2: Zigzag Level Order Traversal ────────────────────────────

struct ZigzagLevelOrder;
struct ZigzagLevelOrderTest {
    tree: Vec<Option<i32>>,
}

impl Problem for ZigzagLevelOrder {
    fn id(&self) -> &str { "binary_trees_zigzag_level_order" }
    fn name(&self) -> &str { "Binary Tree Zigzag Level Order Traversal" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a binary tree, return its zigzag level order traversal.\n\n\
         The first level is left-to-right, the second right-to-left, the third \
         left-to-right, and so on.\n\n\
         Example: [3,9,20,None,None,15,7] -> [[3],[20,9],[15,7]]\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 100\n\
         - -100 <= Node.val <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8).map(|_| {
            let size = rng.random_range(1..=25);
            let tree = random_tree(&mut rng, size, -100, 100);
            TestCase { data: Box::new(ZigzagLevelOrderTest { tree }) }
        }).collect();
        tests.push(TestCase { data: Box::new(ZigzagLevelOrderTest { tree: vec![] }) });
        tests.push(TestCase { data: Box::new(ZigzagLevelOrderTest { tree: vec![Some(1)] }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ZigzagLevelOrderTest>().unwrap();
        let expected = ref_zigzag_level_order(&t.tree);
        let actual = solutions::zigzag_level_order(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_zigzag_level_order(tree: &[Option<i32>]) -> Vec<Vec<i32>> {
    let mut levels = ref_level_order(tree);
    for (i, level) in levels.iter_mut().enumerate() {
        if i % 2 == 1 {
            level.reverse();
        }
    }
    levels
}

// ── Medium 3: Right Side View ─────────────────────────────────────────

struct RightSideView;
struct RightSideViewTest {
    tree: Vec<Option<i32>>,
}

impl Problem for RightSideView {
    fn id(&self) -> &str { "binary_trees_right_side_view" }
    fn name(&self) -> &str { "Binary Tree Right Side View" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a binary tree, imagine yourself standing on the right side of it. \
         Return the values of the nodes you can see ordered from top to bottom.\n\n\
         Example: [1,2,3,None,5,None,4] -> [1,3,4]\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 100\n\
         - -100 <= Node.val <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8).map(|_| {
            let size = rng.random_range(1..=25);
            let tree = random_tree(&mut rng, size, -100, 100);
            TestCase { data: Box::new(RightSideViewTest { tree }) }
        }).collect();
        tests.push(TestCase { data: Box::new(RightSideViewTest { tree: vec![] }) });
        tests.push(TestCase { data: Box::new(RightSideViewTest { tree: vec![Some(1)] }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RightSideViewTest>().unwrap();
        let expected = ref_right_side_view(&t.tree);
        let actual = solutions::right_side_view(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_right_side_view(tree: &[Option<i32>]) -> Vec<i32> {
    let levels = ref_level_order(tree);
    levels.iter().filter_map(|level| level.last().copied()).collect()
}

// ── Medium 4: Flatten Binary Tree to Linked List ──────────────────────

struct FlattenToLinkedList;
struct FlattenToLinkedListTest {
    tree: Vec<Option<i32>>,
}

impl Problem for FlattenToLinkedList {
    fn id(&self) -> &str { "binary_trees_flatten_to_linked_list" }
    fn name(&self) -> &str { "Flatten Binary Tree to Linked List" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a binary tree, flatten it to a linked list in-place following preorder \
         traversal. Return the values in preorder as a Vec<i32>.\n\n\
         Example: [1,2,5,3,4,None,6] -> [1,2,3,4,5,6]\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 100\n\
         - -100 <= Node.val <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8).map(|_| {
            let size = rng.random_range(1..=20);
            let tree = random_tree(&mut rng, size, -100, 100);
            TestCase { data: Box::new(FlattenToLinkedListTest { tree }) }
        }).collect();
        tests.push(TestCase { data: Box::new(FlattenToLinkedListTest { tree: vec![] }) });
        tests.push(TestCase { data: Box::new(FlattenToLinkedListTest { tree: vec![Some(1)] }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FlattenToLinkedListTest>().unwrap();
        let expected = ref_preorder(&t.tree);
        let actual = solutions::flatten_to_linked_list(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_preorder(tree: &[Option<i32>]) -> Vec<i32> {
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

// ── Medium 5: Construct Binary Tree from Preorder and Inorder ─────────

struct ConstructFromPreorderInorder;
struct ConstructFromPreorderInorderTest {
    preorder: Vec<i32>,
    inorder_vals: Vec<i32>,
}

impl Problem for ConstructFromPreorderInorder {
    fn id(&self) -> &str { "binary_trees_construct_from_preorder_inorder" }
    fn name(&self) -> &str { "Construct Binary Tree from Preorder and Inorder Traversal" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given two integer arrays `preorder` and `inorder` where `preorder` is the \
         preorder traversal and `inorder` is the inorder traversal of the same tree, \
         construct the binary tree and return its level-order representation.\n\n\
         All values are unique.\n\n\
         Example: preorder=[3,9,20,15,7], inorder=[9,3,15,20,7]\n\
         -> [3,9,20,None,None,15,7]\n\n\
         Constraints:\n\
         - 1 <= len <= 30\n\
         - All values are unique."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..9).map(|_| {
            let size = rng.random_range(1..=15);
            let tree = random_tree(&mut rng, size, -200, 200);
            // Rebuild to get consistent unique values
            let (arena, root) = build_tree(&tree);
            // Ensure unique values: assign indices as values
            let mut unique_arena = arena.clone();
            let vals_in = inorder(&arena, root);
            // If there are duplicates, regenerate with unique values
            let unique_set: std::collections::HashSet<i32> = vals_in.iter().copied().collect();
            if unique_set.len() == vals_in.len() {
                let preorder_vals = ref_preorder(&tree);
                let inorder_vals = inorder(&unique_arena, root);
                TestCase {
                    data: Box::new(ConstructFromPreorderInorderTest {
                        preorder: preorder_vals,
                        inorder_vals,
                    }),
                }
            } else {
                // Reassign unique values
                let mut counter = 0i32;
                fn assign_unique(arena: &mut Vec<TreeNode>, node: Option<usize>, counter: &mut i32) {
                    if let Some(idx) = node {
                        assign_unique(arena, arena[idx].left, counter);
                        arena[idx].val = *counter;
                        *counter += 1;
                        assign_unique(arena, arena[idx].right, counter);
                    }
                }
                assign_unique(&mut unique_arena, root, &mut counter);
                let level_order = tree_to_level_order(&unique_arena, root);
                let preorder_vals = ref_preorder(&level_order);
                let inorder_vals = inorder(&unique_arena, root);
                TestCase {
                    data: Box::new(ConstructFromPreorderInorderTest {
                        preorder: preorder_vals,
                        inorder_vals,
                    }),
                }
            }
        }).collect();
        // Edge case: single node
        tests.push(TestCase {
            data: Box::new(ConstructFromPreorderInorderTest {
                preorder: vec![1],
                inorder_vals: vec![1],
            }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ConstructFromPreorderInorderTest>().unwrap();
        let expected = ref_build_from_preorder_inorder(&t.preorder, &t.inorder_vals);
        let actual = solutions::construct_from_preorder_inorder(&t.preorder, &t.inorder_vals);
        // Compare by rebuilding and checking inorder traversal (structure match)
        let (ea, er) = build_tree(&expected);
        let (aa, ar) = build_tree(&actual);
        let e_in = inorder(&ea, er);
        let a_in = inorder(&aa, ar);
        let e_pre = ref_preorder(&expected);
        let a_pre = ref_preorder(&actual);
        SolutionResult {
            is_correct: e_in == a_in && e_pre == a_pre,
            input_description: format!("preorder={:?}, inorder={:?}", t.preorder, t.inorder_vals),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_build_from_preorder_inorder(preorder: &[i32], inorder_vals: &[i32]) -> Vec<Option<i32>> {
    use std::collections::HashMap;
    let in_map: HashMap<i32, usize> = inorder_vals.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let mut pre_idx = 0;
    let mut arena = Vec::new();

    fn build(
        preorder: &[i32],
        pre_idx: &mut usize,
        in_lo: usize,
        in_hi: usize,
        in_map: &HashMap<i32, usize>,
        arena: &mut Vec<TreeNode>,
    ) -> Option<usize> {
        if in_lo > in_hi || *pre_idx >= preorder.len() {
            return None;
        }
        let root_val = preorder[*pre_idx];
        *pre_idx += 1;
        let idx = arena.len();
        arena.push(TreeNode { val: root_val, left: None, right: None });
        let in_pos = in_map[&root_val];
        let left = if in_pos > in_lo {
            build(preorder, pre_idx, in_lo, in_pos - 1, in_map, arena)
        } else {
            None
        };
        let right = if in_pos < in_hi {
            build(preorder, pre_idx, in_pos + 1, in_hi, in_map, arena)
        } else {
            None
        };
        arena[idx].left = left;
        arena[idx].right = right;
        Some(idx)
    }

    if preorder.is_empty() {
        return vec![];
    }
    let root = build(preorder, &mut pre_idx, 0, inorder_vals.len() - 1, &in_map, &mut arena);
    tree_to_level_order(&arena, root)
}

// ── Hard 1: Binary Tree Maximum Path Sum ──────────────────────────────

struct MaxPathSum;
struct MaxPathSumTest {
    tree: Vec<Option<i32>>,
}

impl Problem for MaxPathSum {
    fn id(&self) -> &str { "binary_trees_max_path_sum" }
    fn name(&self) -> &str { "Binary Tree Maximum Path Sum" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given a binary tree, find the maximum path sum. A path is a sequence of nodes \
         where each pair of adjacent nodes has an edge. The path does not need to pass \
         through the root. The path sum is the sum of the node values in the path.\n\n\
         Example: [1,2,3] -> 6 (path 2->1->3)\n\
         Example: [-10,9,20,None,None,15,7] -> 42 (path 15->20->7)\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 100\n\
         - -1000 <= Node.val <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8).map(|_| {
            let size = rng.random_range(1..=20);
            let tree = random_tree(&mut rng, size, -100, 100);
            TestCase { data: Box::new(MaxPathSumTest { tree }) }
        }).collect();
        // Edge case: single node
        tests.push(TestCase { data: Box::new(MaxPathSumTest { tree: vec![Some(-5)] }) });
        // All negative
        tests.push(TestCase { data: Box::new(MaxPathSumTest {
            tree: vec![Some(-3), Some(-2), Some(-1)],
        }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxPathSumTest>().unwrap();
        let expected = ref_max_path_sum(&t.tree);
        let actual = solutions::max_path_sum(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_path_sum(tree: &[Option<i32>]) -> i32 {
    let (arena, root) = build_tree(tree);
    let mut max_sum = i32::MIN;
    fn gain(arena: &[TreeNode], node: Option<usize>, max_sum: &mut i32) -> i32 {
        let Some(idx) = node else { return 0 };
        let left_gain = gain(arena, arena[idx].left, max_sum).max(0);
        let right_gain = gain(arena, arena[idx].right, max_sum).max(0);
        let path_through = arena[idx].val + left_gain + right_gain;
        *max_sum = (*max_sum).max(path_through);
        arena[idx].val + left_gain.max(right_gain)
    }
    gain(&arena, root, &mut max_sum);
    max_sum
}

// ── Hard 2: Serialize and Deserialize Binary Tree ─────────────────────

struct SerializeDeserialize;
struct SerializeDeserializeTest {
    tree: Vec<Option<i32>>,
}

impl Problem for SerializeDeserialize {
    fn id(&self) -> &str { "binary_trees_serialize_deserialize" }
    fn name(&self) -> &str { "Serialize and Deserialize Binary Tree" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Design an algorithm to serialize a binary tree into a string and deserialize \
         the string back to the original tree.\n\n\
         Input: a tree as Vec<Option<i32>> (level-order).\n\
         Output: Vec<Option<i32>> -- the round-trip result after serialize then deserialize.\n\n\
         Your output must produce a tree structurally identical to the input.\n\n\
         Example: [1,2,3,None,None,4,5] -> [1,2,3,None,None,4,5]\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 100\n\
         - -1000 <= Node.val <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8).map(|_| {
            let size = rng.random_range(1..=20);
            let tree = random_tree(&mut rng, size, -1000, 1000);
            TestCase { data: Box::new(SerializeDeserializeTest { tree }) }
        }).collect();
        tests.push(TestCase { data: Box::new(SerializeDeserializeTest { tree: vec![] }) });
        tests.push(TestCase { data: Box::new(SerializeDeserializeTest { tree: vec![Some(42)] }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SerializeDeserializeTest>().unwrap();
        // The expected output is the canonical level-order form of the input tree.
        let (arena, root) = build_tree(&t.tree);
        let expected = tree_to_level_order(&arena, root);
        let actual = solutions::serialize_deserialize(&t.tree);
        // Compare by rebuilding both and checking structural equality
        let (ea, er) = build_tree(&expected);
        let (aa, ar) = build_tree(&actual);
        let correct = ref_is_same_tree(
            &tree_to_level_order(&ea, er),
            &tree_to_level_order(&aa, ar),
        );
        SolutionResult {
            is_correct: correct,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 3: Lowest Common Ancestor ────────────────────────────────────

struct LowestCommonAncestor;
struct LowestCommonAncestorTest {
    tree: Vec<Option<i32>>,
    p: i32,
    q: i32,
}

impl Problem for LowestCommonAncestor {
    fn id(&self) -> &str { "binary_trees_lowest_common_ancestor" }
    fn name(&self) -> &str { "Lowest Common Ancestor of a Binary Tree" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given a binary tree and two node values `p` and `q`, find the lowest common \
         ancestor (LCA). The LCA is the deepest node that is an ancestor of both p \
         and q (a node can be an ancestor of itself).\n\n\
         All node values are unique. Both p and q exist in the tree.\n\n\
         Example: tree=[3,5,1,6,2,0,8,None,None,7,4], p=5, q=1 -> 3\n\
         Example: tree=[3,5,1,6,2,0,8,None,None,7,4], p=5, q=4 -> 5\n\n\
         Constraints:\n\
         - 2 <= number of nodes <= 100\n\
         - All values are unique.\n\
         - p != q"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = Vec::new();
        for _ in 0..10 {
            let size = rng.random_range(2..=20);
            // Build tree with unique values
            let vals: Vec<i32> = crate::problems::helpers::random_unique_vec(&mut rng, size, -200, 200);
            // Build a tree from unique values
            let tree = random_tree(&mut rng, size, -200, 200);
            let (mut arena, root) = build_tree(&tree);
            // Reassign unique values via inorder
            let mut idx = 0;
            fn assign_vals(arena: &mut Vec<TreeNode>, node: Option<usize>, vals: &[i32], idx: &mut usize) {
                if let Some(n) = node {
                    assign_vals(arena, arena[n].left, vals, idx);
                    arena[n].val = vals[*idx];
                    *idx += 1;
                    assign_vals(arena, arena[n].right, vals, idx);
                }
            }
            assign_vals(&mut arena, root, &vals, &mut idx);
            let tree_lo = tree_to_level_order(&arena, root);
            // Collect all values and pick two
            let all_vals = inorder(&arena, root);
            let pi = rng.random_range(0..all_vals.len());
            let mut qi = rng.random_range(0..all_vals.len());
            while qi == pi { qi = rng.random_range(0..all_vals.len()); }
            let p = all_vals[pi];
            let q = all_vals[qi];
            tests.push(TestCase {
                data: Box::new(LowestCommonAncestorTest { tree: tree_lo, p, q }),
            });
        }
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LowestCommonAncestorTest>().unwrap();
        let expected = ref_lca(&t.tree, t.p, t.q);
        let actual = solutions::lowest_common_ancestor(&t.tree, t.p, t.q);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}, p={}, q={}", t.tree, t.p, t.q),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_lca(tree: &[Option<i32>], p: i32, q: i32) -> i32 {
    let (arena, root) = build_tree(tree);
    fn find_lca(arena: &[TreeNode], node: Option<usize>, p: i32, q: i32) -> Option<usize> {
        let idx = node?;
        let left = find_lca(arena, arena[idx].left, p, q);
        let right = find_lca(arena, arena[idx].right, p, q);
        if arena[idx].val == p || arena[idx].val == q {
            return Some(idx);
        }
        match (left, right) {
            (Some(_), Some(_)) => Some(idx),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }
    let lca_idx = find_lca(&arena, root, p, q).unwrap();
    arena[lca_idx].val
}

// ── Hard 4: Diameter of Binary Tree ───────────────────────────────────

struct Diameter;
struct DiameterTest {
    tree: Vec<Option<i32>>,
}

impl Problem for Diameter {
    fn id(&self) -> &str { "binary_trees_diameter" }
    fn name(&self) -> &str { "Diameter of Binary Tree" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given a binary tree, return its diameter. The diameter is the length of the \
         longest path between any two nodes (measured in number of edges).\n\n\
         The path may or may not pass through the root.\n\n\
         Example: [1,2,3,4,5] -> 3 (path 4->2->1->3 or 5->2->1->3)\n\n\
         Constraints:\n\
         - 1 <= number of nodes <= 100\n\
         - -100 <= Node.val <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8).map(|_| {
            let size = rng.random_range(1..=25);
            let tree = random_tree(&mut rng, size, -100, 100);
            TestCase { data: Box::new(DiameterTest { tree }) }
        }).collect();
        tests.push(TestCase { data: Box::new(DiameterTest { tree: vec![Some(1)] }) });
        tests.push(TestCase { data: Box::new(DiameterTest {
            tree: vec![Some(1), Some(2)],
        }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DiameterTest>().unwrap();
        let expected = ref_diameter(&t.tree);
        let actual = solutions::diameter(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_diameter(tree: &[Option<i32>]) -> i32 {
    let (arena, root) = build_tree(tree);
    let mut max_diameter = 0;
    fn height(arena: &[TreeNode], node: Option<usize>, max_d: &mut i32) -> i32 {
        let Some(idx) = node else { return 0 };
        let l = height(arena, arena[idx].left, max_d);
        let r = height(arena, arena[idx].right, max_d);
        *max_d = (*max_d).max(l + r);
        1 + l.max(r)
    }
    height(&arena, root, &mut max_diameter);
    max_diameter
}

// ── Hard 5: Count Complete Tree Nodes ─────────────────────────────────

struct CountCompleteTreeNodes;
struct CountCompleteTreeNodesTest {
    tree: Vec<Option<i32>>,
}

impl Problem for CountCompleteTreeNodes {
    fn id(&self) -> &str { "binary_trees_count_complete_tree_nodes" }
    fn name(&self) -> &str { "Count Complete Tree Nodes" }
    fn topic(&self) -> &str { "binary_trees" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given a complete binary tree, return the number of nodes.\n\n\
         A complete binary tree is one where every level, except possibly the last, \
         is completely filled, and all nodes in the last level are as far left as possible.\n\n\
         Design an algorithm that runs in less than O(n) time.\n\n\
         Example: [1,2,3,4,5,6] -> 6\n\n\
         Constraints:\n\
         - 0 <= number of nodes <= 1000\n\
         - The tree is a complete binary tree."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8).map(|_| {
            let size = rng.random_range(1..=50);
            // Build a complete binary tree
            let tree: Vec<Option<i32>> = (0..size).map(|i| Some(i as i32 + 1)).collect();
            TestCase { data: Box::new(CountCompleteTreeNodesTest { tree }) }
        }).collect();
        tests.push(TestCase { data: Box::new(CountCompleteTreeNodesTest { tree: vec![] }) });
        tests.push(TestCase { data: Box::new(CountCompleteTreeNodesTest { tree: vec![Some(1)] }) });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountCompleteTreeNodesTest>().unwrap();
        let (arena, root) = build_tree(&t.tree);
        let expected = ref_count_nodes(&arena, root);
        let actual = solutions::count_complete_tree_nodes(&t.tree);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tree={:?}", t.tree),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_count_nodes(arena: &[TreeNode], root: Option<usize>) -> i32 {
    match root {
        None => 0,
        Some(idx) => 1 + ref_count_nodes(arena, arena[idx].left) + ref_count_nodes(arena, arena[idx].right),
    }
}
