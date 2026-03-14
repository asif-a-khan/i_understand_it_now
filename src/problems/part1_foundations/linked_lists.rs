use rand::Rng;

use crate::problems::helpers::{random_sorted_vec, random_vec};
use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part1_foundations::linked_lists as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy
        Box::new(ReverseList),
        Box::new(MergeTwoSorted),
        Box::new(HasCycle),
        Box::new(RemoveNthFromEnd),
        Box::new(IsPalindrome),
        // Medium
        Box::new(AddTwoNumbers),
        Box::new(ReorderList),
        Box::new(SortList),
        Box::new(RemoveDuplicatesII),
        Box::new(RotateList),
        // Hard
        Box::new(ReverseKGroup),
        Box::new(MergeKSorted),
        Box::new(CopyRandomPointer),
        Box::new(LruCache),
        Box::new(FlattenMultilevel),
    ]
}

// ── Easy 1: Reverse Linked List ─────────────────────────────────────────

struct ReverseList;
struct ReverseListTest {
    vals: Vec<i32>,
}

impl Problem for ReverseList {
    fn id(&self) -> &str {
        "linked_lists_reverse"
    }
    fn name(&self) -> &str {
        "Reverse Linked List"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a singly linked list (represented as a Vec<i32> of values), reverse it.\n\n\
         Example: [1, 2, 3, 4, 5] -> [5, 4, 3, 2, 1]\n\n\
         Constraints:\n\
         - 0 <= len <= 50\n\
         - -100 <= val <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..9)
            .map(|_| {
                let n = rng.random_range(0..=20);
                let vals = random_vec(&mut rng, n, -100, 100);
                TestCase {
                    data: Box::new(ReverseListTest { vals }),
                }
            })
            .collect();
        // Edge case: empty list
        tests.push(TestCase {
            data: Box::new(ReverseListTest { vals: vec![] }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ReverseListTest>().unwrap();
        let expected: Vec<i32> = t.vals.iter().rev().cloned().collect();
        let actual = solutions::reverse_list(&t.vals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("vals={:?}", t.vals),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 2: Merge Two Sorted Lists ──────────────────────────────────────

struct MergeTwoSorted;
struct MergeTwoSortedTest {
    l1: Vec<i32>,
    l2: Vec<i32>,
}

impl Problem for MergeTwoSorted {
    fn id(&self) -> &str {
        "linked_lists_merge_two_sorted"
    }
    fn name(&self) -> &str {
        "Merge Two Sorted Lists"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Merge two sorted linked lists into one sorted list.\n\n\
         Example: l1=[1,2,4], l2=[1,3,4] -> [1,1,2,3,4,4]\n\n\
         Constraints:\n\
         - 0 <= len(l1), len(l2) <= 50\n\
         - Both lists are sorted in non-decreasing order."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8)
            .map(|_| {
                let n1 = rng.random_range(0..=15);
                let n2 = rng.random_range(0..=15);
                let l1 = random_sorted_vec(&mut rng, n1, -50, 50);
                let l2 = random_sorted_vec(&mut rng, n2, -50, 50);
                TestCase {
                    data: Box::new(MergeTwoSortedTest { l1, l2 }),
                }
            })
            .collect();
        // Edge cases: one or both empty
        tests.push(TestCase {
            data: Box::new(MergeTwoSortedTest {
                l1: vec![],
                l2: vec![1, 3, 5],
            }),
        });
        tests.push(TestCase {
            data: Box::new(MergeTwoSortedTest {
                l1: vec![],
                l2: vec![],
            }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MergeTwoSortedTest>().unwrap();
        let mut expected = t.l1.clone();
        expected.extend_from_slice(&t.l2);
        expected.sort();
        let actual = solutions::merge_two_sorted(&t.l1, &t.l2);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("l1={:?}, l2={:?}", t.l1, t.l2),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 3: Linked List Has Cycle ───────────────────────────────────────

struct HasCycle;
struct HasCycleTest {
    vals: Vec<i32>,
    cycle_pos: Option<usize>,
}

impl Problem for HasCycle {
    fn id(&self) -> &str {
        "linked_lists_has_cycle"
    }
    fn name(&self) -> &str {
        "Linked List Cycle"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Detect if a linked list has a cycle.\n\n\
         You receive the list values and a `cycle_pos` indicating which index \
         the tail connects back to (None means no cycle).\n\n\
         Use Floyd's cycle detection (tortoise and hare) for O(1) space.\n\n\
         Constraints:\n\
         - 0 <= len <= 50\n\
         - cycle_pos is None or a valid index in the list."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = Vec::new();
        // Generate lists with cycles
        for _ in 0..5 {
            let n = rng.random_range(2..=20);
            let vals = random_vec(&mut rng, n, -100, 100);
            let cycle_pos = Some(rng.random_range(0..n));
            tests.push(TestCase {
                data: Box::new(HasCycleTest { vals, cycle_pos }),
            });
        }
        // Generate lists without cycles
        for _ in 0..4 {
            let n = rng.random_range(0..=20);
            let vals = random_vec(&mut rng, n, -100, 100);
            tests.push(TestCase {
                data: Box::new(HasCycleTest {
                    vals,
                    cycle_pos: None,
                }),
            });
        }
        // Edge case: single node with no cycle
        tests.push(TestCase {
            data: Box::new(HasCycleTest {
                vals: vec![1],
                cycle_pos: None,
            }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<HasCycleTest>().unwrap();
        let expected = t.cycle_pos.is_some();
        let actual = solutions::has_cycle(&t.vals, t.cycle_pos);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("vals={:?}, cycle_pos={:?}", t.vals, t.cycle_pos),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Easy 4: Remove Nth Node From End ────────────────────────────────────

struct RemoveNthFromEnd;
struct RemoveNthTest {
    vals: Vec<i32>,
    n: usize,
}

impl Problem for RemoveNthFromEnd {
    fn id(&self) -> &str {
        "linked_lists_remove_nth_from_end"
    }
    fn name(&self) -> &str {
        "Remove Nth Node From End of List"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Remove the nth node from the end of a linked list and return the result.\n\n\
         n is 1-indexed: n=1 removes the last node, n=len removes the first node.\n\n\
         Example: vals=[1,2,3,4,5], n=2 -> [1,2,3,5]\n\n\
         Constraints:\n\
         - 1 <= len <= 30\n\
         - 1 <= n <= len"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8)
            .map(|_| {
                let len = rng.random_range(1..=20);
                let vals = random_vec(&mut rng, len, -100, 100);
                let n = rng.random_range(1..=len);
                TestCase {
                    data: Box::new(RemoveNthTest { vals, n }),
                }
            })
            .collect();
        // Edge case: single element, remove it
        tests.push(TestCase {
            data: Box::new(RemoveNthTest {
                vals: vec![42],
                n: 1,
            }),
        });
        // Edge case: remove first element
        tests.push(TestCase {
            data: Box::new(RemoveNthTest {
                vals: vec![1, 2, 3],
                n: 3,
            }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RemoveNthTest>().unwrap();
        let remove_idx = t.vals.len() - t.n;
        let mut expected = t.vals.clone();
        expected.remove(remove_idx);
        let actual = solutions::remove_nth_from_end(&t.vals, t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("vals={:?}, n={}", t.vals, t.n),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Easy 5: Palindrome Linked List ──────────────────────────────────────

struct IsPalindrome;
struct IsPalindromeTest {
    vals: Vec<i32>,
}

impl Problem for IsPalindrome {
    fn id(&self) -> &str {
        "linked_lists_is_palindrome"
    }
    fn name(&self) -> &str {
        "Palindrome Linked List"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Check if a linked list is a palindrome.\n\n\
         Example: [1,2,2,1] -> true, [1,2] -> false\n\n\
         Constraints:\n\
         - 0 <= len <= 50\n\
         - 0 <= val <= 9"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = Vec::new();
        // Generate palindromes
        for _ in 0..4 {
            let half_len = rng.random_range(1..=10);
            let mut vals: Vec<i32> = (0..half_len).map(|_| rng.random_range(0..=9)).collect();
            let mut rev = vals.clone();
            rev.reverse();
            // Optionally add a middle element for odd-length palindromes
            if rng.random_range(0..2) == 0 {
                vals.push(rng.random_range(0..=9));
            }
            vals.extend(rev);
            tests.push(TestCase {
                data: Box::new(IsPalindromeTest { vals }),
            });
        }
        // Generate non-palindromes
        for _ in 0..4 {
            let n = rng.random_range(2..=15);
            let mut vals: Vec<i32> = (0..n).map(|_| rng.random_range(0..=9)).collect();
            // Ensure it is NOT a palindrome by making first != last
            if vals.first() == vals.last() {
                let last = vals.len() - 1;
                vals[last] = (vals[0] + 1) % 10;
            }
            tests.push(TestCase {
                data: Box::new(IsPalindromeTest { vals }),
            });
        }
        // Edge cases
        tests.push(TestCase {
            data: Box::new(IsPalindromeTest { vals: vec![] }),
        });
        tests.push(TestCase {
            data: Box::new(IsPalindromeTest { vals: vec![5] }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IsPalindromeTest>().unwrap();
        let reversed: Vec<i32> = t.vals.iter().rev().cloned().collect();
        let expected = t.vals == reversed;
        let actual = solutions::is_palindrome(&t.vals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("vals={:?}", t.vals),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

// ── Medium 1: Add Two Numbers ───────────────────────────────────────────

struct AddTwoNumbers;
struct AddTwoNumbersTest {
    l1: Vec<i32>,
    l2: Vec<i32>,
}

impl Problem for AddTwoNumbers {
    fn id(&self) -> &str {
        "linked_lists_add_two_numbers"
    }
    fn name(&self) -> &str {
        "Add Two Numbers"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Two numbers are represented as linked lists where each node is a digit, \
         stored in reverse order (1's digit first).\n\n\
         Add the two numbers and return the sum as a reversed digit list.\n\n\
         Example: l1=[2,4,3] (342), l2=[5,6,4] (465) -> [7,0,8] (807)\n\n\
         Constraints:\n\
         - 1 <= len <= 20\n\
         - 0 <= digit <= 9\n\
         - No leading zeros (except the number 0 itself)."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8)
            .map(|_| {
                let n1 = rng.random_range(1..=10);
                let n2 = rng.random_range(1..=10);
                let l1 = ref_make_number_digits(&mut rng, n1);
                let l2 = ref_make_number_digits(&mut rng, n2);
                TestCase {
                    data: Box::new(AddTwoNumbersTest { l1, l2 }),
                }
            })
            .collect();
        // Edge case: adding zeros
        tests.push(TestCase {
            data: Box::new(AddTwoNumbersTest {
                l1: vec![0],
                l2: vec![0],
            }),
        });
        // Edge case: carry propagation
        tests.push(TestCase {
            data: Box::new(AddTwoNumbersTest {
                l1: vec![9, 9, 9],
                l2: vec![1],
            }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AddTwoNumbersTest>().unwrap();
        let expected = ref_add_two_numbers(&t.l1, &t.l2);
        let actual = solutions::add_two_numbers(&t.l1, &t.l2);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("l1={:?}, l2={:?}", t.l1, t.l2),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

/// Generate a valid reversed-digit list representing a random number.
fn ref_make_number_digits(rng: &mut impl Rng, len: usize) -> Vec<i32> {
    let mut digits: Vec<i32> = (0..len).map(|_| rng.random_range(0..=9)).collect();
    // Ensure no leading zeros in the number (last digit of reversed list != 0), unless len==1
    if len > 1 && digits[len - 1] == 0 {
        digits[len - 1] = rng.random_range(1..=9);
    }
    digits
}

fn ref_add_two_numbers(l1: &[i32], l2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut carry = 0;
    let mut i = 0;
    while i < l1.len() || i < l2.len() || carry > 0 {
        let a = if i < l1.len() { l1[i] } else { 0 };
        let b = if i < l2.len() { l2[i] } else { 0 };
        let sum = a + b + carry;
        result.push(sum % 10);
        carry = sum / 10;
        i += 1;
    }
    result
}

// ── Medium 2: Reorder List ──────────────────────────────────────────────

struct ReorderList;
struct ReorderListTest {
    vals: Vec<i32>,
}

impl Problem for ReorderList {
    fn id(&self) -> &str {
        "linked_lists_reorder"
    }
    fn name(&self) -> &str {
        "Reorder List"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Reorder a linked list from L0->L1->...->Ln to L0->Ln->L1->Ln-1->L2->Ln-2->...\n\n\
         Example: [1,2,3,4] -> [1,4,2,3]\n\
         Example: [1,2,3,4,5] -> [1,5,2,4,3]\n\n\
         Constraints:\n\
         - 1 <= len <= 50"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let vals = random_vec(&mut rng, n, -100, 100);
                TestCase {
                    data: Box::new(ReorderListTest { vals }),
                }
            })
            .collect();
        // Edge cases
        tests.push(TestCase {
            data: Box::new(ReorderListTest { vals: vec![1] }),
        });
        tests.push(TestCase {
            data: Box::new(ReorderListTest { vals: vec![1, 2] }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ReorderListTest>().unwrap();
        let expected = ref_reorder_list(&t.vals);
        let actual = solutions::reorder_list(&t.vals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("vals={:?}", t.vals),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_reorder_list(vals: &[i32]) -> Vec<i32> {
    if vals.len() <= 2 {
        return vals.to_vec();
    }
    let mut result = Vec::with_capacity(vals.len());
    let mut left = 0;
    let mut right = vals.len() - 1;
    while left < right {
        result.push(vals[left]);
        result.push(vals[right]);
        left += 1;
        right -= 1;
    }
    if left == right {
        result.push(vals[left]);
    }
    result
}

// ── Medium 3: Sort List ─────────────────────────────────────────────────

struct SortList;
struct SortListTest {
    vals: Vec<i32>,
}

impl Problem for SortList {
    fn id(&self) -> &str {
        "linked_lists_sort"
    }
    fn name(&self) -> &str {
        "Sort List"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Sort a linked list in O(n log n) time and O(1) extra space.\n\n\
         Example: [4,2,1,3] -> [1,2,3,4]\n\n\
         Constraints:\n\
         - 0 <= len <= 50\n\
         - -10^5 <= val <= 10^5"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..9)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let vals = random_vec(&mut rng, n, -1000, 1000);
                TestCase {
                    data: Box::new(SortListTest { vals }),
                }
            })
            .collect();
        // Edge case: empty
        tests.push(TestCase {
            data: Box::new(SortListTest { vals: vec![] }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SortListTest>().unwrap();
        let mut expected = t.vals.clone();
        expected.sort();
        let actual = solutions::sort_list(&t.vals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("vals={:?}", t.vals),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Medium 4: Remove Duplicates from Sorted List II ─────────────────────

struct RemoveDuplicatesII;
struct RemoveDuplicatesIITest {
    vals: Vec<i32>,
}

impl Problem for RemoveDuplicatesII {
    fn id(&self) -> &str {
        "linked_lists_remove_duplicates_ii"
    }
    fn name(&self) -> &str {
        "Remove Duplicates from Sorted List II"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a sorted linked list, delete all nodes that have duplicate numbers, \
         leaving only distinct numbers.\n\n\
         Example: [1,2,3,3,4,4,5] -> [1,2,5]\n\
         Example: [1,1,1,2,3] -> [2,3]\n\n\
         Constraints:\n\
         - 0 <= len <= 50\n\
         - -100 <= val <= 100\n\
         - List is sorted in ascending order."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8)
            .map(|_| {
                let n = rng.random_range(0..=20);
                // Use a narrow range to encourage duplicates
                let vals = random_sorted_vec(&mut rng, n, -10, 10);
                TestCase {
                    data: Box::new(RemoveDuplicatesIITest { vals }),
                }
            })
            .collect();
        // Edge: all duplicates
        tests.push(TestCase {
            data: Box::new(RemoveDuplicatesIITest {
                vals: vec![1, 1, 2, 2, 3, 3],
            }),
        });
        // Edge: no duplicates
        tests.push(TestCase {
            data: Box::new(RemoveDuplicatesIITest {
                vals: vec![1, 2, 3, 4, 5],
            }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RemoveDuplicatesIITest>().unwrap();
        let expected = ref_remove_duplicates_ii(&t.vals);
        let actual = solutions::remove_duplicates_ii(&t.vals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("vals={:?}", t.vals),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_remove_duplicates_ii(vals: &[i32]) -> Vec<i32> {
    if vals.is_empty() {
        return vec![];
    }
    let mut counts: Vec<(i32, usize)> = Vec::new();
    for &v in vals {
        if let Some(last) = counts.last_mut() {
            if last.0 == v {
                last.1 += 1;
                continue;
            }
        }
        counts.push((v, 1));
    }
    counts
        .into_iter()
        .filter(|&(_, c)| c == 1)
        .map(|(v, _)| v)
        .collect()
}

// ── Medium 5: Rotate List ───────────────────────────────────────────────

struct RotateList;
struct RotateListTest {
    vals: Vec<i32>,
    k: usize,
}

impl Problem for RotateList {
    fn id(&self) -> &str {
        "linked_lists_rotate"
    }
    fn name(&self) -> &str {
        "Rotate List"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Rotate a linked list to the right by k places.\n\n\
         Example: [1,2,3,4,5], k=2 -> [4,5,1,2,3]\n\n\
         Constraints:\n\
         - 0 <= len <= 50\n\
         - 0 <= k <= 2 * 10^9 (k can be larger than list length)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let vals = random_vec(&mut rng, n, -100, 100);
                let k = rng.random_range(0..=n * 3);
                TestCase {
                    data: Box::new(RotateListTest { vals, k }),
                }
            })
            .collect();
        // Edge case: empty list
        tests.push(TestCase {
            data: Box::new(RotateListTest {
                vals: vec![],
                k: 5,
            }),
        });
        // Edge case: k=0
        tests.push(TestCase {
            data: Box::new(RotateListTest {
                vals: vec![1, 2, 3],
                k: 0,
            }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RotateListTest>().unwrap();
        let expected = ref_rotate_list(&t.vals, t.k);
        let actual = solutions::rotate_list(&t.vals, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("vals={:?}, k={}", t.vals, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_rotate_list(vals: &[i32], k: usize) -> Vec<i32> {
    if vals.is_empty() {
        return vec![];
    }
    let n = vals.len();
    let k = k % n;
    if k == 0 {
        return vals.to_vec();
    }
    let mut result = Vec::with_capacity(n);
    result.extend_from_slice(&vals[n - k..]);
    result.extend_from_slice(&vals[..n - k]);
    result
}

// ── Hard 1: Reverse Nodes in K-Group ────────────────────────────────────

struct ReverseKGroup;
struct ReverseKGroupTest {
    vals: Vec<i32>,
    k: usize,
}

impl Problem for ReverseKGroup {
    fn id(&self) -> &str {
        "linked_lists_reverse_k_group"
    }
    fn name(&self) -> &str {
        "Reverse Nodes in k-Group"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Reverse the nodes of a linked list k at a time and return the modified list.\n\n\
         k is a positive integer <= length of the list. If the number of nodes is not a \
         multiple of k, the remaining nodes at the end stay in their original order.\n\n\
         Example: [1,2,3,4,5], k=2 -> [2,1,4,3,5]\n\
         Example: [1,2,3,4,5], k=3 -> [3,2,1,4,5]\n\n\
         Constraints:\n\
         - 1 <= len <= 50\n\
         - 1 <= k <= len"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let vals = random_vec(&mut rng, n, -100, 100);
                let k = rng.random_range(1..=n);
                TestCase {
                    data: Box::new(ReverseKGroupTest { vals, k }),
                }
            })
            .collect();
        // Edge case: k=1 (no change)
        tests.push(TestCase {
            data: Box::new(ReverseKGroupTest {
                vals: vec![1, 2, 3],
                k: 1,
            }),
        });
        // Edge case: k=len (full reverse)
        tests.push(TestCase {
            data: Box::new(ReverseKGroupTest {
                vals: vec![1, 2, 3, 4],
                k: 4,
            }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ReverseKGroupTest>().unwrap();
        let expected = ref_reverse_k_group(&t.vals, t.k);
        let actual = solutions::reverse_k_group(&t.vals, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("vals={:?}, k={}", t.vals, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_reverse_k_group(vals: &[i32], k: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(vals.len());
    let mut i = 0;
    while i + k <= vals.len() {
        let mut chunk: Vec<i32> = vals[i..i + k].to_vec();
        chunk.reverse();
        result.extend(chunk);
        i += k;
    }
    // Remaining nodes stay in original order
    result.extend_from_slice(&vals[i..]);
    result
}

// ── Hard 2: Merge K Sorted Lists ────────────────────────────────────────

struct MergeKSorted;
struct MergeKSortedTest {
    lists: Vec<Vec<i32>>,
}

impl Problem for MergeKSorted {
    fn id(&self) -> &str {
        "linked_lists_merge_k_sorted"
    }
    fn name(&self) -> &str {
        "Merge k Sorted Lists"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Merge k sorted linked lists into one sorted list.\n\n\
         Example: [[1,4,5],[1,3,4],[2,6]] -> [1,1,2,3,4,4,5,6]\n\n\
         Constraints:\n\
         - 0 <= k <= 20\n\
         - 0 <= len(list_i) <= 30\n\
         - Each list is sorted in ascending order.\n\
         - Target: O(N log k) using a min-heap."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8)
            .map(|_| {
                let k = rng.random_range(0..=10);
                let lists: Vec<Vec<i32>> = (0..k)
                    .map(|_| {
                        let n = rng.random_range(0..=10);
                        random_sorted_vec(&mut rng, n, -100, 100)
                    })
                    .collect();
                TestCase {
                    data: Box::new(MergeKSortedTest { lists }),
                }
            })
            .collect();
        // Edge case: empty lists
        tests.push(TestCase {
            data: Box::new(MergeKSortedTest { lists: vec![] }),
        });
        // Edge case: single list
        tests.push(TestCase {
            data: Box::new(MergeKSortedTest {
                lists: vec![vec![1, 3, 5, 7]],
            }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MergeKSortedTest>().unwrap();
        let mut expected: Vec<i32> = t.lists.iter().flat_map(|l| l.iter().cloned()).collect();
        expected.sort();
        let actual = solutions::merge_k_sorted(&t.lists);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("lists={:?}", t.lists),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 3: Copy List with Random Pointer ───────────────────────────────

struct CopyRandomPointer;
struct CopyRandomPointerTest {
    vals: Vec<i32>,
}

impl Problem for CopyRandomPointer {
    fn id(&self) -> &str {
        "linked_lists_copy_random_pointer"
    }
    fn name(&self) -> &str {
        "Copy List with Random Pointer"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Deep copy a linked list. In the full problem, each node also has a random pointer; \
         here the task is simplified to producing a deep copy of the values.\n\n\
         Your copy must be a new Vec<i32> with the same values in the same order.\n\n\
         Constraints:\n\
         - 0 <= len <= 50\n\
         - -100 <= val <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..9)
            .map(|_| {
                let n = rng.random_range(0..=20);
                let vals = random_vec(&mut rng, n, -100, 100);
                TestCase {
                    data: Box::new(CopyRandomPointerTest { vals }),
                }
            })
            .collect();
        tests.push(TestCase {
            data: Box::new(CopyRandomPointerTest { vals: vec![] }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CopyRandomPointerTest>().unwrap();
        let expected = t.vals.clone();
        let actual = solutions::copy_random_pointer(&t.vals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("vals={:?}", t.vals),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

// ── Hard 4: LRU Cache ──────────────────────────────────────────────────

struct LruCache;
struct LruCacheTest {
    capacity: usize,
    ops: Vec<(String, Vec<i32>)>,
}

impl Problem for LruCache {
    fn id(&self) -> &str {
        "linked_lists_lru_cache"
    }
    fn name(&self) -> &str {
        "LRU Cache"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Design and implement an LRU (Least Recently Used) cache.\n\n\
         Operations:\n\
         - get(key): Return the value if key exists (and mark as recently used), \
           otherwise return -1. Represented as Some(value) or Some(-1).\n\
         - put(key, value): Insert or update the key-value pair. If the cache is at capacity, \
           evict the least recently used item first. Returns None.\n\n\
         Input: capacity and a list of (operation_name, args) tuples.\n\
         Output: Vec<Option<i32>> with one entry per operation.\n\n\
         Constraints:\n\
         - 1 <= capacity <= 50\n\
         - 0 <= key, value <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..8)
            .map(|_| {
                let capacity = rng.random_range(1..=10);
                let num_ops = rng.random_range(5..=30);
                let ops = ref_generate_lru_ops(&mut rng, num_ops);
                TestCase {
                    data: Box::new(LruCacheTest { capacity, ops }),
                }
            })
            .collect();
        // Edge case: capacity 1
        tests.push(TestCase {
            data: Box::new(LruCacheTest {
                capacity: 1,
                ops: vec![
                    ("put".to_string(), vec![1, 10]),
                    ("put".to_string(), vec![2, 20]),
                    ("get".to_string(), vec![1]),
                    ("get".to_string(), vec![2]),
                ],
            }),
        });
        // Classic test case
        tests.push(TestCase {
            data: Box::new(LruCacheTest {
                capacity: 2,
                ops: vec![
                    ("put".to_string(), vec![1, 1]),
                    ("put".to_string(), vec![2, 2]),
                    ("get".to_string(), vec![1]),
                    ("put".to_string(), vec![3, 3]),
                    ("get".to_string(), vec![2]),
                    ("put".to_string(), vec![4, 4]),
                    ("get".to_string(), vec![1]),
                    ("get".to_string(), vec![3]),
                    ("get".to_string(), vec![4]),
                ],
            }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LruCacheTest>().unwrap();
        let expected = ref_lru_cache(t.capacity, &t.ops);
        let actual = solutions::lru_cache(t.capacity, &t.ops);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("capacity={}, ops={:?}", t.capacity, t.ops),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_generate_lru_ops(rng: &mut impl Rng, count: usize) -> Vec<(String, Vec<i32>)> {
    let mut ops = Vec::with_capacity(count);
    for _ in 0..count {
        if rng.random_range(0..3) == 0 {
            // get
            let key = rng.random_range(0..=20);
            ops.push(("get".to_string(), vec![key]));
        } else {
            // put
            let key = rng.random_range(0..=20);
            let value = rng.random_range(0..=100);
            ops.push(("put".to_string(), vec![key, value]));
        }
    }
    ops
}

fn ref_lru_cache(capacity: usize, ops: &[(String, Vec<i32>)]) -> Vec<Option<i32>> {
    // Use a Vec as an ordered map (most recently used at the end).
    let mut cache: Vec<(i32, i32)> = Vec::new();
    let mut results = Vec::new();

    for (op, args) in ops {
        match op.as_str() {
            "get" => {
                let key = args[0];
                if let Some(pos) = cache.iter().position(|&(k, _)| k == key) {
                    let entry = cache.remove(pos);
                    cache.push(entry);
                    results.push(Some(entry.1));
                } else {
                    results.push(Some(-1));
                }
            }
            "put" => {
                let key = args[0];
                let value = args[1];
                if let Some(pos) = cache.iter().position(|&(k, _)| k == key) {
                    cache.remove(pos);
                }
                cache.push((key, value));
                if cache.len() > capacity {
                    cache.remove(0); // evict least recently used
                }
                results.push(None);
            }
            _ => {}
        }
    }
    results
}

// ── Hard 5: Flatten Multilevel Doubly Linked List ───────────────────────

struct FlattenMultilevel;
struct FlattenMultilevelTest {
    vals: Vec<i32>,
}

impl Problem for FlattenMultilevel {
    fn id(&self) -> &str {
        "linked_lists_flatten_multilevel"
    }
    fn name(&self) -> &str {
        "Flatten a Multilevel Doubly Linked List"
    }
    fn topic(&self) -> &str {
        "linked_lists"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Flatten a multilevel doubly linked list. In this simplified version, \
         you receive a flat list of values and must return them as-is (the flattening \
         is already represented by the input order).\n\n\
         In the real problem, a multilevel list has child pointers that create sub-lists; \
         flattening inserts child lists inline between the current node and its next node.\n\n\
         For practice purposes, implement a DFS-based flattening: given values, return them \
         in the same order (proving you can traverse and reconstruct the list).\n\n\
         Constraints:\n\
         - 0 <= len <= 100\n\
         - 1 <= val <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests: Vec<TestCase> = (0..9)
            .map(|_| {
                let n = rng.random_range(0..=30);
                let vals = random_vec(&mut rng, n, 1, 1000);
                TestCase {
                    data: Box::new(FlattenMultilevelTest { vals }),
                }
            })
            .collect();
        tests.push(TestCase {
            data: Box::new(FlattenMultilevelTest { vals: vec![] }),
        });
        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FlattenMultilevelTest>().unwrap();
        let expected = t.vals.clone();
        let actual = solutions::flatten_multilevel(&t.vals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("vals={:?}", t.vals),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}
