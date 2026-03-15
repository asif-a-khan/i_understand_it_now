use rand::Rng;
use std::collections::{HashMap, VecDeque};

use std::cell::RefCell;
use std::rc::Rc;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part1_foundations::stacks_queues as solutions;
use crate::tracker::{track_slice, track_string, OperationLog};

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        // Easy (5)
        Box::new(ValidParentheses),
        Box::new(MinStack),
        Box::new(QueueUsingStacks),
        Box::new(BaseballGame),
        Box::new(NextGreaterElement),
        // Medium (5)
        Box::new(DailyTemperatures),
        Box::new(EvalRPN),
        Box::new(DecodeString),
        Box::new(AsteroidCollision),
        Box::new(OnlineStockSpan),
        // Hard (5)
        Box::new(SlidingWindowMax),
        Box::new(LargestRectangleHistogram),
        Box::new(TrappingRainWaterStack),
        Box::new(BasicCalculator),
        Box::new(MaxFrequencyStack),
    ]
}

// ── Easy 1: Valid Parentheses ─────────────────────────────────────────

struct ValidParentheses;
struct ValidParenthesesTest {
    s: String,
}

impl Problem for ValidParentheses {
    fn id(&self) -> &str {
        "stacks_valid_parentheses"
    }
    fn name(&self) -> &str {
        "Valid Parentheses"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a string `s` containing only the characters '(', ')', '{', '}', '[' and ']',\n\
         determine if the input string is valid.\n\n\
         A string is valid if:\n\
         - Open brackets are closed by the same type of brackets.\n\
         - Open brackets are closed in the correct order.\n\
         - Every close bracket has a corresponding open bracket of the same type.\n\n\
         Constraints:\n\
         - 0 <= s.len() <= 40\n\
         - s consists of parentheses only '()[]{}'."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        let brackets = [b'(', b')', b'[', b']', b'{', b'}'];
        (0..10)
            .map(|_| {
                let len = rng.random_range(0..=20);
                let s: String = (0..len)
                    .map(|_| brackets[rng.random_range(0..brackets.len())] as char)
                    .collect();
                TestCase {
                    data: Box::new(ValidParenthesesTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ValidParenthesesTest>().unwrap();
        let expected = ref_valid_parentheses(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::valid_parentheses(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}", t.s),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_valid_parentheses(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            _ => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}

// ── Easy 2: Min Stack ────────────────────────────────────────────────

struct MinStack;

/// Operations: ("push", Some(val)), ("pop", None), ("top", None), ("getMin", None)
/// Query ops (top, getMin) produce results; push/pop produce None.
struct MinStackTest {
    operations: Vec<(String, Option<i32>)>,
}

impl Problem for MinStack {
    fn id(&self) -> &str {
        "stacks_min_stack"
    }
    fn name(&self) -> &str {
        "Min Stack"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Design a stack that supports push, pop, top, and retrieving the minimum element \
         in constant time.\n\n\
         Implement the function `min_stack(ops)` that takes a list of operations:\n\
         - (\"push\", Some(val)) -> push val onto stack, return None\n\
         - (\"pop\", None)       -> remove top element, return None\n\
         - (\"top\", None)       -> return Some(top element)\n\
         - (\"getMin\", None)    -> return Some(minimum element)\n\n\
         Return a Vec<Option<i32>> with one entry per operation.\n\n\
         Constraints:\n\
         - Methods pop, top, and getMin will always be called on non-empty stacks.\n\
         - At most 30 operations."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let mut ops = Vec::new();
                let mut stack_size = 0usize;
                let num_ops = rng.random_range(5..=30);
                for _ in 0..num_ops {
                    if stack_size == 0 {
                        // Must push when stack is empty
                        let val = rng.random_range(-100..=100);
                        ops.push(("push".to_string(), Some(val)));
                        stack_size += 1;
                    } else {
                        let choice = rng.random_range(0..4);
                        match choice {
                            0 => {
                                let val = rng.random_range(-100..=100);
                                ops.push(("push".to_string(), Some(val)));
                                stack_size += 1;
                            }
                            1 => {
                                ops.push(("pop".to_string(), None));
                                stack_size -= 1;
                            }
                            2 => {
                                ops.push(("top".to_string(), None));
                            }
                            _ => {
                                ops.push(("getMin".to_string(), None));
                            }
                        }
                    }
                }
                TestCase {
                    data: Box::new(MinStackTest { operations: ops }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MinStackTest>().unwrap();
        let expected = ref_min_stack(&t.operations);
        let actual = solutions::min_stack(&t.operations, log);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("ops={:?}", t.operations),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_min_stack(ops: &[(String, Option<i32>)]) -> Vec<Option<i32>> {
    let mut stack: Vec<i32> = Vec::new();
    let mut min_stack: Vec<i32> = Vec::new();
    let mut results = Vec::new();

    for (op, val) in ops {
        match op.as_str() {
            "push" => {
                let v = val.unwrap();
                stack.push(v);
                let cur_min = min_stack.last().copied().unwrap_or(i32::MAX);
                min_stack.push(v.min(cur_min));
                results.push(None);
            }
            "pop" => {
                stack.pop();
                min_stack.pop();
                results.push(None);
            }
            "top" => {
                results.push(Some(*stack.last().unwrap()));
            }
            "getMin" => {
                results.push(Some(*min_stack.last().unwrap()));
            }
            _ => results.push(None),
        }
    }
    results
}

// ── Easy 3: Queue Using Stacks ──────────────────────────────────────

struct QueueUsingStacks;

/// Operations: ("push", Some(val)), ("pop", None), ("peek", None), ("empty", None)
/// pop returns Some(front), peek returns Some(front), empty returns Some(0 or 1).
struct QueueUsingStacksTest {
    operations: Vec<(String, Option<i32>)>,
}

impl Problem for QueueUsingStacks {
    fn id(&self) -> &str {
        "stacks_queue_using_stacks"
    }
    fn name(&self) -> &str {
        "Queue Using Two Stacks"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Implement a FIFO queue using only two stacks.\n\n\
         Implement the function `queue_using_stacks(ops)` that takes operations:\n\
         - (\"push\", Some(val)) -> enqueue val, return None\n\
         - (\"pop\", None)       -> dequeue front element, return Some(val)\n\
         - (\"peek\", None)      -> return Some(front element) without removing\n\
         - (\"empty\", None)     -> return Some(1) if empty, Some(0) otherwise\n\n\
         Return a Vec<Option<i32>> with one entry per operation.\n\n\
         Constraints:\n\
         - pop and peek are called only when the queue is non-empty.\n\
         - At most 30 operations."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let mut ops = Vec::new();
                let mut queue_size = 0usize;
                let num_ops = rng.random_range(5..=30);
                for _ in 0..num_ops {
                    if queue_size == 0 {
                        let val = rng.random_range(-100..=100);
                        ops.push(("push".to_string(), Some(val)));
                        queue_size += 1;
                    } else {
                        let choice = rng.random_range(0..4);
                        match choice {
                            0 => {
                                let val = rng.random_range(-100..=100);
                                ops.push(("push".to_string(), Some(val)));
                                queue_size += 1;
                            }
                            1 => {
                                ops.push(("pop".to_string(), None));
                                queue_size -= 1;
                            }
                            2 => {
                                ops.push(("peek".to_string(), None));
                            }
                            _ => {
                                ops.push(("empty".to_string(), None));
                            }
                        }
                    }
                }
                TestCase {
                    data: Box::new(QueueUsingStacksTest { operations: ops }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<QueueUsingStacksTest>().unwrap();
        let expected = ref_queue_using_stacks(&t.operations);
        let actual = solutions::queue_using_stacks(&t.operations, log);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("ops={:?}", t.operations),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_queue_using_stacks(ops: &[(String, Option<i32>)]) -> Vec<Option<i32>> {
    let mut in_stack: Vec<i32> = Vec::new();
    let mut out_stack: Vec<i32> = Vec::new();
    let mut results = Vec::new();

    let transfer = |in_s: &mut Vec<i32>, out_s: &mut Vec<i32>| {
        if out_s.is_empty() {
            while let Some(v) = in_s.pop() {
                out_s.push(v);
            }
        }
    };

    for (op, val) in ops {
        match op.as_str() {
            "push" => {
                in_stack.push(val.unwrap());
                results.push(None);
            }
            "pop" => {
                transfer(&mut in_stack, &mut out_stack);
                results.push(Some(out_stack.pop().unwrap()));
            }
            "peek" => {
                transfer(&mut in_stack, &mut out_stack);
                results.push(Some(*out_stack.last().unwrap()));
            }
            "empty" => {
                let empty = if in_stack.is_empty() && out_stack.is_empty() {
                    1
                } else {
                    0
                };
                results.push(Some(empty));
            }
            _ => results.push(None),
        }
    }
    results
}

// ── Easy 4: Baseball Game ───────────────────────────────────────────

struct BaseballGame;
struct BaseballGameTest {
    ops: Vec<String>,
}

impl Problem for BaseballGame {
    fn id(&self) -> &str {
        "stacks_baseball_game"
    }
    fn name(&self) -> &str {
        "Baseball Game"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "You are keeping score for a baseball game with strange rules. Given a list of \
         string operations, apply them in order:\n\n\
         - An integer x: record a new score of x.\n\
         - \"+\": record a new score that is the sum of the previous two scores.\n\
         - \"D\": record a new score that is double the previous score.\n\
         - \"C\": invalidate (remove) the previous score.\n\n\
         Return the sum of all scores after applying all operations.\n\n\
         Constraints:\n\
         - 1 <= ops.len() <= 30\n\
         - \"+\" always has at least 2 previous scores.\n\
         - \"C\" and \"D\" always have at least 1 previous score."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let mut ops = Vec::new();
                let mut stack_size = 0usize;
                let num_ops = rng.random_range(3..=20);
                for _ in 0..num_ops {
                    if stack_size < 2 {
                        // Push a number to build up the stack
                        let val = rng.random_range(-100..=100);
                        ops.push(val.to_string());
                        stack_size += 1;
                    } else {
                        let choice = rng.random_range(0..4);
                        match choice {
                            0 => {
                                let val = rng.random_range(-100..=100);
                                ops.push(val.to_string());
                                stack_size += 1;
                            }
                            1 => {
                                ops.push("+".to_string());
                                stack_size += 1;
                            }
                            2 => {
                                ops.push("D".to_string());
                                stack_size += 1;
                            }
                            3 => {
                                ops.push("C".to_string());
                                stack_size -= 1;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                TestCase {
                    data: Box::new(BaseballGameTest { ops }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BaseballGameTest>().unwrap();
        let expected = ref_baseball_game(&t.ops);
        let actual = solutions::baseball_game(&t.ops, log);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("ops={:?}", t.ops),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_baseball_game(ops: &[String]) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for op in ops {
        match op.as_str() {
            "+" => {
                let top = stack[stack.len() - 1];
                let second = stack[stack.len() - 2];
                stack.push(top + second);
            }
            "D" => {
                let top = stack[stack.len() - 1];
                stack.push(top * 2);
            }
            "C" => {
                stack.pop();
            }
            num => {
                stack.push(num.parse::<i32>().unwrap());
            }
        }
    }
    stack.iter().sum()
}

// ── Easy 5: Next Greater Element ────────────────────────────────────

struct NextGreaterElement;
struct NextGreaterElementTest {
    nums: Vec<i32>,
}

impl Problem for NextGreaterElement {
    fn id(&self) -> &str {
        "stacks_next_greater_element"
    }
    fn name(&self) -> &str {
        "Next Greater Element"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a circular array `nums`, find the next greater element for each element.\n\
         The next greater element of element nums[i] is the first element that is greater \
         than nums[i] traversing the array in a circular fashion. If none exists, output -1.\n\n\
         Hint: use a monotonic stack, traverse the array twice (circular).\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 50\n\
         - -100 <= nums[i] <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                TestCase {
                    data: Box::new(NextGreaterElementTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<NextGreaterElementTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_next_greater_element(&t.nums);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::next_greater_element(&tracked_nums);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_next_greater_element(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![-1i32; n];
    let mut stack: Vec<usize> = Vec::new();
    for i in (0..2 * n).rev() {
        let idx = i % n;
        while let Some(&top) = stack.last() {
            if nums[top] <= nums[idx] {
                stack.pop();
            } else {
                break;
            }
        }
        if i < n {
            result[idx] = stack.last().map_or(-1, |&top| nums[top]);
        }
        stack.push(idx);
    }
    result
}

// ── Medium 1: Daily Temperatures ────────────────────────────────────

struct DailyTemperatures;
struct DailyTemperaturesTest {
    temperatures: Vec<i32>,
}

impl Problem for DailyTemperatures {
    fn id(&self) -> &str {
        "stacks_daily_temperatures"
    }
    fn name(&self) -> &str {
        "Daily Temperatures"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an array of integers `temperatures` representing daily temperatures, \
         return an array `answer` such that `answer[i]` is the number of days you have to \
         wait after the ith day to get a warmer temperature. If there is no future day \
         with a warmer temperature, set `answer[i] = 0`.\n\n\
         Hint: use a monotonic decreasing stack storing indices.\n\n\
         Constraints:\n\
         - 1 <= temperatures.len() <= 100\n\
         - 30 <= temperatures[i] <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=50);
                let temperatures: Vec<i32> = (0..n).map(|_| rng.random_range(30..=100)).collect();
                TestCase {
                    data: Box::new(DailyTemperaturesTest { temperatures }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DailyTemperaturesTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_daily_temperatures(&t.temperatures);
        let tracked_temperatures = track_slice(&t.temperatures, shared_log.clone());
        let actual = solutions::daily_temperatures(&tracked_temperatures);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("temps={:?}", t.temperatures),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_daily_temperatures(temps: &[i32]) -> Vec<i32> {
    let n = temps.len();
    let mut result = vec![0i32; n];
    let mut stack: Vec<usize> = Vec::new();
    for i in 0..n {
        while let Some(&top) = stack.last() {
            if temps[top] < temps[i] {
                stack.pop();
                result[top] = (i - top) as i32;
            } else {
                break;
            }
        }
        stack.push(i);
    }
    result
}

// ── Medium 2: Evaluate Reverse Polish Notation ──────────────────────

struct EvalRPN;
struct EvalRPNTest {
    tokens: Vec<String>,
}

impl Problem for EvalRPN {
    fn id(&self) -> &str {
        "stacks_eval_rpn"
    }
    fn name(&self) -> &str {
        "Evaluate Reverse Polish Notation"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Evaluate the value of an arithmetic expression in Reverse Polish Notation.\n\n\
         Valid operators are +, -, *, /. Each operand may be an integer or another expression.\n\
         Division truncates toward zero.\n\n\
         Input: Vec<String> of tokens (numbers and operators).\n\
         Output: i32 result.\n\n\
         Constraints:\n\
         - 1 <= tokens.len() <= 30\n\
         - tokens[i] is either an operator or an integer in range [-200, 200].\n\
         - The expression is always valid."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let tokens = gen_rpn_expression(&mut rng);
                TestCase {
                    data: Box::new(EvalRPNTest { tokens }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<EvalRPNTest>().unwrap();
        let expected = ref_eval_rpn(&t.tokens);
        let actual = solutions::eval_rpn(&t.tokens, log);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tokens={:?}", t.tokens),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

/// Generate a valid RPN expression with controlled depth to avoid overflow.
fn gen_rpn_expression(rng: &mut impl Rng) -> Vec<String> {
    let ops_count = rng.random_range(1..=5);
    let mut tokens = Vec::new();
    // Start with two numbers
    tokens.push(rng.random_range(-20..=20).to_string());
    tokens.push(rng.random_range(-20..=20).to_string());
    let operators = ["+", "-", "*"];
    tokens.push(operators[rng.random_range(0..operators.len())].to_string());
    for _ in 1..ops_count {
        tokens.push(rng.random_range(-20..=20).to_string());
        // Avoid division by zero — use only +, -, *
        tokens.push(operators[rng.random_range(0..operators.len())].to_string());
    }
    tokens
}

fn ref_eval_rpn(tokens: &[String]) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for token in tokens {
        match token.as_str() {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match token.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            num => {
                stack.push(num.parse::<i32>().unwrap());
            }
        }
    }
    stack[0]
}

// ── Medium 3: Decode String ─────────────────────────────────────────

struct DecodeString;
struct DecodeStringTest {
    s: String,
}

impl Problem for DecodeString {
    fn id(&self) -> &str {
        "stacks_decode_string"
    }
    fn name(&self) -> &str {
        "Decode String"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an encoded string, return its decoded string.\n\n\
         The encoding rule is: k[encoded_string], where the encoded_string inside the \
         square brackets is repeated exactly k times.\n\n\
         Examples:\n\
         - \"3[a]\" -> \"aaa\"\n\
         - \"3[a2[c]]\" -> \"accaccacc\"\n\
         - \"2[abc]3[cd]ef\" -> \"abcabccdcdcdef\"\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 30\n\
         - s consists of lowercase English letters, digits, and square brackets.\n\
         - s is guaranteed to be a valid input.\n\
         - All integers in s are in range [1, 9]."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let s = gen_encoded_string(&mut rng, 2);
                TestCase {
                    data: Box::new(DecodeStringTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DecodeStringTest>().unwrap();
        let expected = ref_decode_string(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::decode_string(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}", t.s),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

/// Generate a random valid encoded string with nesting up to max_depth.
fn gen_encoded_string(rng: &mut impl Rng, max_depth: usize) -> String {
    let mut result = String::new();
    let parts = rng.random_range(1..=3);
    for _ in 0..parts {
        let choice = if max_depth == 0 {
            0
        } else {
            rng.random_range(0..2)
        };
        match choice {
            0 => {
                // Plain letters
                let len = rng.random_range(1..=3);
                for _ in 0..len {
                    result.push((b'a' + rng.random_range(0..26u8)) as char);
                }
            }
            _ => {
                // k[...]
                let k = rng.random_range(1..=3);
                result.push_str(&k.to_string());
                result.push('[');
                result.push_str(&gen_encoded_string(rng, max_depth - 1));
                result.push(']');
            }
        }
    }
    result
}

fn ref_decode_string(s: &str) -> String {
    let mut stack: Vec<(String, usize)> = Vec::new();
    let mut current = String::new();
    let mut num = 0usize;

    for c in s.chars() {
        match c {
            '0'..='9' => {
                num = num * 10 + (c as usize - '0' as usize);
            }
            '[' => {
                stack.push((current.clone(), num));
                current.clear();
                num = 0;
            }
            ']' => {
                let (prev_str, count) = stack.pop().unwrap();
                let repeated = current.repeat(count);
                current = prev_str + &repeated;
            }
            _ => {
                current.push(c);
            }
        }
    }
    current
}

// ── Medium 4: Asteroid Collision ────────────────────────────────────

struct AsteroidCollision;
struct AsteroidCollisionTest {
    asteroids: Vec<i32>,
}

impl Problem for AsteroidCollision {
    fn id(&self) -> &str {
        "stacks_asteroid_collision"
    }
    fn name(&self) -> &str {
        "Asteroid Collision"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "We are given an array `asteroids` of integers representing asteroids in a row.\n\n\
         For each asteroid, the absolute value represents its size, and the sign represents \
         its direction (positive = right, negative = left). Each asteroid moves at the same speed.\n\n\
         When two asteroids meet, the smaller one explodes. If both are the same size, both explode. \
         Two asteroids moving in the same direction will never meet.\n\n\
         Return the state of the asteroids after all collisions.\n\n\
         Constraints:\n\
         - 1 <= asteroids.len() <= 50\n\
         - -100 <= asteroids[i] <= 100, asteroids[i] != 0"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let asteroids: Vec<i32> = (0..n)
                    .map(|_| {
                        let size = rng.random_range(1..=50);
                        if rng.random_range(0..2) == 0 {
                            size
                        } else {
                            -size
                        }
                    })
                    .collect();
                TestCase {
                    data: Box::new(AsteroidCollisionTest { asteroids }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AsteroidCollisionTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_asteroid_collision(&t.asteroids);
        let tracked_asteroids = track_slice(&t.asteroids, shared_log.clone());
        let actual = solutions::asteroid_collision(&tracked_asteroids);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("asteroids={:?}", t.asteroids),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_asteroid_collision(asteroids: &[i32]) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for &ast in asteroids {
        let mut alive = true;
        while alive && ast < 0 && !stack.is_empty() && *stack.last().unwrap() > 0 {
            let top = *stack.last().unwrap();
            if top < -ast {
                stack.pop();
            } else if top == -ast {
                stack.pop();
                alive = false;
            } else {
                alive = false;
            }
        }
        if alive {
            stack.push(ast);
        }
    }
    stack
}

// ── Medium 5: Online Stock Span ─────────────────────────────────────

struct OnlineStockSpan;
struct OnlineStockSpanTest {
    prices: Vec<i32>,
}

impl Problem for OnlineStockSpan {
    fn id(&self) -> &str {
        "stacks_online_stock_span"
    }
    fn name(&self) -> &str {
        "Online Stock Span"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a series of daily stock prices, for each day compute the \"span\" of the \
         stock's price: the maximum number of consecutive days (starting from today and \
         going backward) for which the stock price was less than or equal to today's price.\n\n\
         Input: Vec<i32> of daily prices.\n\
         Output: Vec<i32> of spans (one per day).\n\n\
         Example: prices = [100, 80, 60, 70, 60, 75, 85]\n\
         Output:          [1,   1,  1,  2,  1,  4,  6]\n\n\
         Hint: use a monotonic stack storing (price, span) pairs.\n\n\
         Constraints:\n\
         - 1 <= prices.len() <= 50\n\
         - 1 <= prices[i] <= 1000"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=30);
                let prices: Vec<i32> = (0..n).map(|_| rng.random_range(1..=200)).collect();
                TestCase {
                    data: Box::new(OnlineStockSpanTest { prices }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<OnlineStockSpanTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_online_stock_span(&t.prices);
        let tracked_prices = track_slice(&t.prices, shared_log.clone());
        let actual = solutions::online_stock_span(&tracked_prices);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("prices={:?}", t.prices),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_online_stock_span(prices: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    // Stack of (price, accumulated_span)
    let mut stack: Vec<(i32, i32)> = Vec::new();
    for &price in prices {
        let mut span = 1;
        while let Some(&(top_price, top_span)) = stack.last() {
            if top_price <= price {
                span += top_span;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push((price, span));
        result.push(span);
    }
    result
}

// ── Hard 1: Sliding Window Maximum ──────────────────────────────────

struct SlidingWindowMax;
struct SlidingWindowMaxTest {
    nums: Vec<i32>,
    k: usize,
}

impl Problem for SlidingWindowMax {
    fn id(&self) -> &str {
        "stacks_sliding_window_max"
    }
    fn name(&self) -> &str {
        "Sliding Window Maximum"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array of integers `nums` and a sliding window of size `k`, \
         return the maximum value in each window as the window slides from left to right.\n\n\
         Input: (nums: Vec<i32>, k: usize)\n\
         Output: Vec<i32> of maximums, length = nums.len() - k + 1.\n\n\
         Hint: use a monotonic deque that stores indices.\n\n\
         Constraints:\n\
         - 1 <= k <= nums.len() <= 100\n\
         - -100 <= nums[i] <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=50);
                let k = rng.random_range(1..=n);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
                TestCase {
                    data: Box::new(SlidingWindowMaxTest { nums, k }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SlidingWindowMaxTest>().unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_sliding_window_max(&t.nums, t.k);
        let tracked_nums = track_slice(&t.nums, shared_log.clone());
        let actual = solutions::sliding_window_max(&tracked_nums, t.k);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}, k={}", t.nums, t.k),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_sliding_window_max(nums: &[i32], k: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..nums.len() {
        // Remove indices that are out of the current window
        while let Some(&front) = deque.front() {
            if front + k <= i {
                deque.pop_front();
            } else {
                break;
            }
        }
        // Remove smaller elements from the back
        while let Some(&back) = deque.back() {
            if nums[back] <= nums[i] {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(i);
        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap()]);
        }
    }
    result
}

// ── Hard 2: Largest Rectangle in Histogram ──────────────────────────

struct LargestRectangleHistogram;
struct LargestRectangleHistogramTest {
    heights: Vec<i32>,
}

impl Problem for LargestRectangleHistogram {
    fn id(&self) -> &str {
        "stacks_largest_rectangle_histogram"
    }
    fn name(&self) -> &str {
        "Largest Rectangle in Histogram"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given an array of integers `heights` representing the histogram's bar heights \
         where the width of each bar is 1, return the area of the largest rectangle \
         in the histogram.\n\n\
         Hint: use a monotonic increasing stack. For each bar, find the first shorter bar \
         to its left and right.\n\n\
         Constraints:\n\
         - 1 <= heights.len() <= 100\n\
         - 0 <= heights[i] <= 200"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=40);
                let heights: Vec<i32> = (0..n).map(|_| rng.random_range(0..=50)).collect();
                TestCase {
                    data: Box::new(LargestRectangleHistogramTest { heights }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<LargestRectangleHistogramTest>()
            .unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_largest_rectangle_histogram(&t.heights);
        let tracked_heights = track_slice(&t.heights, shared_log.clone());
        let actual = solutions::largest_rectangle_histogram(&tracked_heights);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("heights={:?}", t.heights),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_largest_rectangle_histogram(heights: &[i32]) -> i32 {
    let n = heights.len();
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0i32;

    for i in 0..=n {
        let cur_height = if i == n { 0 } else { heights[i] };
        while let Some(&top) = stack.last() {
            if heights[top] >= cur_height {
                stack.pop();
                let h = heights[top];
                let w = match stack.last() {
                    Some(&left) => i - left - 1,
                    None => i,
                };
                max_area = max_area.max(h * w as i32);
            } else {
                break;
            }
        }
        stack.push(i);
    }
    max_area
}

// ── Hard 3: Trapping Rain Water (Stack) ─────────────────────────────

struct TrappingRainWaterStack;
struct TrappingRainWaterStackTest {
    height: Vec<i32>,
}

impl Problem for TrappingRainWaterStack {
    fn id(&self) -> &str {
        "stacks_trapping_rain_water_stack"
    }
    fn name(&self) -> &str {
        "Trapping Rain Water (Stack)"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given `n` non-negative integers representing an elevation map where the width \
         of each bar is 1, compute how much water it can trap after raining.\n\n\
         This is the same problem as the arrays version, but you should solve it using \
         a stack-based approach: process bars left to right, using a stack to find \
         bounded regions.\n\n\
         Constraints:\n\
         - 0 <= height.len() <= 100\n\
         - 0 <= height[i] <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=40);
                let height: Vec<i32> = (0..n).map(|_| rng.random_range(0..=20)).collect();
                TestCase {
                    data: Box::new(TrappingRainWaterStackTest { height }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<TrappingRainWaterStackTest>()
            .unwrap();
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let expected = ref_trapping_rain_water_stack(&t.height);
        let tracked_height = track_slice(&t.height, shared_log.clone());
        let actual = solutions::trapping_rain_water_stack(&tracked_height);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("height={:?}", t.height),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_trapping_rain_water_stack(height: &[i32]) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut water = 0i32;

    for i in 0..height.len() {
        while let Some(&top) = stack.last() {
            if height[top] < height[i] {
                stack.pop();
                if let Some(&left) = stack.last() {
                    let bounded_height = height[i].min(height[left]) - height[top];
                    let width = (i - left - 1) as i32;
                    water += bounded_height * width;
                }
            } else {
                break;
            }
        }
        stack.push(i);
    }
    water
}

// ── Hard 4: Basic Calculator ────────────────────────────────────────

struct BasicCalculator;
struct BasicCalculatorTest {
    s: String,
}

impl Problem for BasicCalculator {
    fn id(&self) -> &str {
        "stacks_basic_calculator"
    }
    fn name(&self) -> &str {
        "Basic Calculator"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a string `s` representing a valid expression, implement a basic calculator \
         to evaluate it and return the result.\n\n\
         The expression string may contain:\n\
         - Digits '0'-'9'\n\
         - '+', '-'\n\
         - '(' and ')'\n\
         - Spaces ' '\n\n\
         Note: unary minus is handled (e.g. \"(-1+2)\" is valid).\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 100\n\
         - The expression is always valid."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let s = gen_calc_expression(&mut rng, 2);
                TestCase {
                    data: Box::new(BasicCalculatorTest { s }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BasicCalculatorTest>().unwrap();
        let expected = ref_basic_calculator(&t.s);
        let shared_log = Rc::new(RefCell::new(OperationLog::new()));
        let tracked = track_string(&t.s, shared_log.clone());
        let actual = solutions::basic_calculator(&tracked);
        for op in shared_log.borrow().operations() {
            log.record(op.clone());
        }
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s={:?}", t.s),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

/// Generate a valid calculator expression with parentheses.
fn gen_calc_expression(rng: &mut impl Rng, max_depth: usize) -> String {
    let terms = rng.random_range(1..=3);
    let mut result = gen_calc_term(rng, max_depth);
    for _ in 1..terms {
        let op = if rng.random_range(0..2) == 0 {
            " + "
        } else {
            " - "
        };
        result.push_str(op);
        result.push_str(&gen_calc_term(rng, max_depth));
    }
    result
}

fn gen_calc_term(rng: &mut impl Rng, max_depth: usize) -> String {
    if max_depth == 0 || rng.random_range(0..3) != 0 {
        rng.random_range(1..=50).to_string()
    } else {
        format!("({})", gen_calc_expression(rng, max_depth - 1))
    }
}

fn ref_basic_calculator(s: &str) -> i32 {
    let mut stack: Vec<(i32, i32)> = Vec::new(); // (result_so_far, sign_before_paren)
    let mut result = 0i32;
    let mut sign = 1i32;
    let mut num = 0i32;

    for c in s.chars() {
        match c {
            '0'..='9' => {
                num = num * 10 + (c as i32 - '0' as i32);
            }
            '+' => {
                result += sign * num;
                num = 0;
                sign = 1;
            }
            '-' => {
                result += sign * num;
                num = 0;
                sign = -1;
            }
            '(' => {
                stack.push((result, sign));
                result = 0;
                sign = 1;
            }
            ')' => {
                result += sign * num;
                num = 0;
                let (prev_result, prev_sign) = stack.pop().unwrap();
                result = prev_result + prev_sign * result;
            }
            _ => {} // spaces
        }
    }
    result + sign * num
}

// ── Hard 5: Maximum Frequency Stack ─────────────────────────────────

struct MaxFrequencyStack;

/// Operations: ("push", Some(val)), ("pop", None)
/// Push returns None, pop returns Some(most_frequent_element).
struct MaxFrequencyStackTest {
    operations: Vec<(String, Option<i32>)>,
}

impl Problem for MaxFrequencyStack {
    fn id(&self) -> &str {
        "stacks_max_frequency_stack"
    }
    fn name(&self) -> &str {
        "Maximum Frequency Stack"
    }
    fn topic(&self) -> &str {
        "stacks_queues"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Implement FreqStack, a stack-like data structure that pops the most frequent element.\n\n\
         If there is a tie for the most frequent element, pop the element closest to the \
         top of the stack.\n\n\
         Implement the function `max_frequency_stack(ops)` that takes operations:\n\
         - (\"push\", Some(val)) -> push val, return None\n\
         - (\"pop\", None)       -> pop most frequent element, return Some(val)\n\n\
         Return a Vec<Option<i32>> with one entry per operation.\n\n\
         Constraints:\n\
         - 0 <= val <= 100\n\
         - At most 50 operations.\n\
         - pop is not called on an empty stack."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let mut ops = Vec::new();
                let mut size = 0usize;
                let num_ops = rng.random_range(5..=30);
                // Push some values first to build diversity
                let num_pushes_first = rng.random_range(3..=num_ops.min(8));
                for _ in 0..num_pushes_first {
                    // Use a small range to create frequency ties
                    let val = rng.random_range(0..=10);
                    ops.push(("push".to_string(), Some(val)));
                    size += 1;
                }
                for _ in num_pushes_first..num_ops {
                    if size == 0 || rng.random_range(0..3) != 0 {
                        let val = rng.random_range(0..=10);
                        ops.push(("push".to_string(), Some(val)));
                        size += 1;
                    } else {
                        ops.push(("pop".to_string(), None));
                        size -= 1;
                    }
                }
                TestCase {
                    data: Box::new(MaxFrequencyStackTest { operations: ops }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxFrequencyStackTest>().unwrap();
        let expected = ref_max_frequency_stack(&t.operations);
        let actual = solutions::max_frequency_stack(&t.operations, log);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("ops={:?}", t.operations),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_max_frequency_stack(ops: &[(String, Option<i32>)]) -> Vec<Option<i32>> {
    let mut freq: HashMap<i32, i32> = HashMap::new();
    let mut group: HashMap<i32, Vec<i32>> = HashMap::new(); // frequency -> stack of values
    let mut max_freq = 0i32;
    let mut results = Vec::new();

    for (op, val) in ops {
        match op.as_str() {
            "push" => {
                let v = val.unwrap();
                let f = freq.entry(v).or_insert(0);
                *f += 1;
                max_freq = max_freq.max(*f);
                group.entry(*f).or_default().push(v);
                results.push(None);
            }
            "pop" => {
                let stack = group.get_mut(&max_freq).unwrap();
                let v = stack.pop().unwrap();
                if stack.is_empty() {
                    group.remove(&max_freq);
                    max_freq -= 1;
                }
                *freq.get_mut(&v).unwrap() -= 1;
                results.push(Some(v));
            }
            _ => results.push(None),
        }
    }
    results
}
