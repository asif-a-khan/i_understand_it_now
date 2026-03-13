use rand::Rng;
use std::collections::HashMap;

use super::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions;
use crate::tracker::OperationLog;

pub struct ArraysTwoSum;

/// Test data for the two-sum problem.
struct TwoSumTest {
    nums: Vec<i32>,
    target: i32,
}

impl Problem for ArraysTwoSum {
    fn id(&self) -> &str {
        "arrays_two_sum"
    }

    fn name(&self) -> &str {
        "Two Sum"
    }

    fn topic(&self) -> &str {
        "arrays"
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }

    fn description(&self) -> &str {
        "Given an array of integers `nums` and an integer `target`, return the indices \
         of the two numbers that add up to `target`.\n\n\
         Constraints:\n\
         - 2 <= nums.len() <= 1000\n\
         - -10^6 <= nums[i] <= 10^6\n\
         - Exactly one valid answer exists.\n\
         - Return indices in ascending order."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let mut tests = Vec::new();

        for _ in 0..10 {
            let n = rng.random_range(2..=20);
            let idx_a = rng.random_range(0..n);
            let mut idx_b = rng.random_range(0..n);
            while idx_b == idx_a {
                idx_b = rng.random_range(0..n);
            }

            let val_a: i32 = rng.random_range(-1000..=1000);
            let val_b: i32 = rng.random_range(-1000..=1000);
            let target = val_a + val_b;

            let mut nums: Vec<i32> = (0..n).map(|_| rng.random_range(-1000..=1000)).collect();
            nums[idx_a] = val_a;
            nums[idx_b] = val_b;

            tests.push(TestCase {
                data: Box::new(TwoSumTest { nums, target }),
            });
        }

        tests
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let test_data = test.data.downcast_ref::<TwoSumTest>().unwrap();
        let nums = &test_data.nums;
        let target = test_data.target;

        let expected = brute_force_two_sum(nums, target);
        let actual = solutions::arrays_two_sum::solve(nums, target);

        // Normalize: sort indices for comparison
        let mut expected_sorted = expected.clone();
        let mut actual_sorted = actual.clone();
        expected_sorted.sort();
        actual_sorted.sort();

        SolutionResult {
            is_correct: expected_sorted == actual_sorted,
            input_description: format!("nums={nums:?}, target={target}"),
            expected: format!("{expected_sorted:?}"),
            actual: format!("{actual_sorted:?}"),
        }
    }
}

/// Reference solution for validating user answers.
fn brute_force_two_sum(nums: &[i32], target: i32) -> Vec<usize> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return vec![j, i];
        }
        map.insert(num, i);
    }
    vec![]
}
