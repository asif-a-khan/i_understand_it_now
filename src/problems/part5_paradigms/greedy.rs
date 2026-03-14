use rand::Rng;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part5_paradigms::greedy as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(AssignCookies),
        Box::new(BestTimeStockII),
        Box::new(LemonadeChange),
        Box::new(MaximumUnits),
        Box::new(CanPlaceFlowers),
        Box::new(JumpGame),
        Box::new(JumpGameII),
        Box::new(GasStation),
        Box::new(TaskScheduler),
        Box::new(PartitionLabels),
        Box::new(Candy),
        Box::new(MinArrowsBurstBalloons),
        Box::new(NonOverlappingIntervals),
        Box::new(QueueReconstruction),
        Box::new(Ipo),
    ]
}

// ── Easy 1: Assign Cookies ───────────────────────────────────────────

struct AssignCookies;

struct AssignCookiesTest {
    children: Vec<i32>,
    cookies: Vec<i32>,
}

impl Problem for AssignCookies {
    fn id(&self) -> &str { "greedy_assign_cookies" }
    fn name(&self) -> &str { "Assign Cookies" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "You are a great parent and want to give cookies to your children. Each child i \
         has a greed factor g[i] — the minimum cookie size that will make them content. \
         Each cookie j has a size s[j]. A child gets at most one cookie, and a cookie of \
         size s[j] satisfies child i only if s[j] >= g[i].\n\n\
         Maximize the number of content children.\n\n\
         Constraints:\n\
         - 1 <= children.len(), cookies.len() <= 30000\n\
         - 1 <= g[i], s[j] <= 2^31 - 1\n\n\
         Example: children=[1,2,3], cookies=[1,1] => 1\n\
         Example: children=[1,2], cookies=[1,2,3] => 2"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let nc = rng.random_range(1..=20);
            let ns = rng.random_range(1..=20);
            let children: Vec<i32> = (0..nc).map(|_| rng.random_range(1..=100)).collect();
            let cookies: Vec<i32> = (0..ns).map(|_| rng.random_range(1..=100)).collect();
            TestCase { data: Box::new(AssignCookiesTest { children, cookies }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<AssignCookiesTest>().unwrap();
        let expected = ref_assign_cookies(&t.children, &t.cookies);
        let actual = solutions::assign_cookies(&t.children, &t.cookies);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("children={:?}, cookies={:?}", t.children, t.cookies),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_assign_cookies(children: &[i32], cookies: &[i32]) -> i32 {
    let mut g = children.to_vec();
    let mut s = cookies.to_vec();
    g.sort();
    s.sort();
    let mut ci = 0;
    let mut si = 0;
    while ci < g.len() && si < s.len() {
        if s[si] >= g[ci] {
            ci += 1;
        }
        si += 1;
    }
    ci as i32
}

// ── Easy 2: Best Time to Buy and Sell Stock II ───────────────────────

struct BestTimeStockII;

struct BestTimeStockIITest {
    prices: Vec<i32>,
}

impl Problem for BestTimeStockII {
    fn id(&self) -> &str { "greedy_best_time_stock_ii" }
    fn name(&self) -> &str { "Best Time to Buy and Sell Stock II" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given an array `prices` where prices[i] is the price of a stock on day i, \
         find the maximum profit. You may complete as many transactions as you like \
         (buy one and sell one share, multiple times). You must sell before buying again.\n\n\
         Constraints:\n\
         - 1 <= prices.len() <= 30000\n\
         - 0 <= prices[i] <= 10000\n\n\
         Example: prices=[7,1,5,3,6,4] => 7 (buy@1,sell@5,buy@3,sell@6)\n\
         Example: prices=[7,6,4,3,1] => 0"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let prices: Vec<i32> = (0..n).map(|_| rng.random_range(0..=200)).collect();
            TestCase { data: Box::new(BestTimeStockIITest { prices }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<BestTimeStockIITest>().unwrap();
        let expected = ref_max_profit_ii(&t.prices);
        let actual = solutions::best_time_stock_ii(&t.prices);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("prices={:?}", t.prices),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_profit_ii(prices: &[i32]) -> i32 {
    let mut profit = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            profit += prices[i] - prices[i - 1];
        }
    }
    profit
}

// ── Easy 3: Lemonade Change ──────────────────────────────────────────

struct LemonadeChange;

struct LemonadeChangeTest {
    bills: Vec<i32>,
}

impl Problem for LemonadeChange {
    fn id(&self) -> &str { "greedy_lemonade_change" }
    fn name(&self) -> &str { "Lemonade Change" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "At a lemonade stand, each lemonade costs $5. Customers stand in a queue and \
         pay with $5, $10, or $20 bills. You must provide the correct change for each \
         customer (using only bills you have received). Return true if you can provide \
         change for every customer.\n\n\
         Constraints:\n\
         - 1 <= bills.len() <= 100000\n\
         - bills[i] is 5, 10, or 20\n\n\
         Example: bills=[5,5,5,10,20] => true\n\
         Example: bills=[5,5,10,10,20] => false"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let bills: Vec<i32> = (0..n)
                .map(|_| {
                    let r = rng.random_range(0..3);
                    match r { 0 => 5, 1 => 10, _ => 20 }
                })
                .collect();
            TestCase { data: Box::new(LemonadeChangeTest { bills }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LemonadeChangeTest>().unwrap();
        let expected = ref_lemonade_change(&t.bills);
        let actual = solutions::lemonade_change(&t.bills);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("bills={:?}", t.bills),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_lemonade_change(bills: &[i32]) -> bool {
    let mut fives = 0;
    let mut tens = 0;
    for &bill in bills {
        match bill {
            5 => fives += 1,
            10 => {
                if fives == 0 { return false; }
                fives -= 1;
                tens += 1;
            }
            20 => {
                if tens > 0 && fives > 0 {
                    tens -= 1;
                    fives -= 1;
                } else if fives >= 3 {
                    fives -= 3;
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

// ── Easy 4: Maximum Units on a Truck ─────────────────────────────────

struct MaximumUnits;

struct MaximumUnitsTest {
    box_types: Vec<(i32, i32)>,
    truck_size: i32,
}

impl Problem for MaximumUnits {
    fn id(&self) -> &str { "greedy_maximum_units" }
    fn name(&self) -> &str { "Maximum Units on a Truck" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "You are assigned to put some boxes onto a truck. You are given a list of box types \
         where box_types[i] = (numberOfBoxes_i, numberOfUnitsPerBox_i). The truck can carry \
         at most truck_size boxes total.\n\n\
         Return the maximum total number of units that can be put on the truck.\n\n\
         Constraints:\n\
         - 1 <= box_types.len() <= 1000\n\
         - 1 <= numberOfBoxes_i, numberOfUnitsPerBox_i <= 1000\n\
         - 1 <= truck_size <= 10^6\n\n\
         Example: box_types=[(1,3),(2,2),(3,1)], truck_size=4 => 8"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=10);
            let box_types: Vec<(i32, i32)> = (0..n)
                .map(|_| {
                    (rng.random_range(1..=20), rng.random_range(1..=50))
                })
                .collect();
            let total_boxes: i32 = box_types.iter().map(|&(cnt, _)| cnt).sum();
            let truck_size = rng.random_range(1..=total_boxes.max(1));
            TestCase { data: Box::new(MaximumUnitsTest { box_types, truck_size }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaximumUnitsTest>().unwrap();
        let expected = ref_maximum_units(&t.box_types, t.truck_size);
        let actual = solutions::maximum_units(&t.box_types, t.truck_size);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("box_types={:?}, truck_size={}", t.box_types, t.truck_size),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_maximum_units(box_types: &[(i32, i32)], truck_size: i32) -> i32 {
    let mut sorted = box_types.to_vec();
    // Sort by units per box descending
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    let mut remaining = truck_size;
    let mut total = 0;
    for (count, units) in &sorted {
        let take = (*count).min(remaining);
        total += take * units;
        remaining -= take;
        if remaining == 0 { break; }
    }
    total
}

// ── Easy 5: Can Place Flowers ────────────────────────────────────────

struct CanPlaceFlowers;

struct CanPlaceFlowersTest {
    flowerbed: Vec<i32>,
    n: i32,
}

impl Problem for CanPlaceFlowers {
    fn id(&self) -> &str { "greedy_can_place_flowers" }
    fn name(&self) -> &str { "Can Place Flowers" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "You have a flowerbed (array of 0s and 1s). 1 means planted, 0 means empty. \
         Flowers cannot be planted in adjacent plots. Return true if n new flowers can \
         be planted without violating the no-adjacent rule.\n\n\
         Constraints:\n\
         - 1 <= flowerbed.len() <= 20000\n\
         - flowerbed[i] is 0 or 1\n\
         - No two adjacent flowers in the initial flowerbed\n\n\
         Example: flowerbed=[1,0,0,0,1], n=1 => true\n\
         Example: flowerbed=[1,0,0,0,1], n=2 => false"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let len = rng.random_range(1..=20);
            let mut flowerbed = vec![0i32; len];
            // Place some flowers respecting adjacency
            let mut i = 0;
            while i < len {
                if rng.random_range(0..3) == 0 {
                    flowerbed[i] = 1;
                    i += 2; // skip next
                } else {
                    i += 1;
                }
            }
            let n = rng.random_range(0..=len as i32 / 2 + 1);
            TestCase { data: Box::new(CanPlaceFlowersTest { flowerbed, n }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CanPlaceFlowersTest>().unwrap();
        let expected = ref_can_place_flowers(&t.flowerbed, t.n);
        let actual = solutions::can_place_flowers(&t.flowerbed, t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("flowerbed={:?}, n={}", t.flowerbed, t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_can_place_flowers(flowerbed: &[i32], n: i32) -> bool {
    let mut bed = flowerbed.to_vec();
    let mut count = 0;
    for i in 0..bed.len() {
        if bed[i] == 0 {
            let left_empty = i == 0 || bed[i - 1] == 0;
            let right_empty = i == bed.len() - 1 || bed[i + 1] == 0;
            if left_empty && right_empty {
                bed[i] = 1;
                count += 1;
            }
        }
    }
    count >= n
}

// ── Medium 1: Jump Game ──────────────────────────────────────────────

struct JumpGame;

struct JumpGameTest {
    nums: Vec<i32>,
}

impl Problem for JumpGame {
    fn id(&self) -> &str { "greedy_jump_game" }
    fn name(&self) -> &str { "Jump Game" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "You are given an array of non-negative integers nums. You are initially at the \
         first index. Each element represents the maximum jump length from that position. \
         Return true if you can reach the last index.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 10000\n\
         - 0 <= nums[i] <= 10^5\n\n\
         Example: nums=[2,3,1,1,4] => true\n\
         Example: nums=[3,2,1,0,4] => false"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=5)).collect();
            TestCase { data: Box::new(JumpGameTest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<JumpGameTest>().unwrap();
        let expected = ref_jump_game(&t.nums);
        let actual = solutions::jump_game(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_jump_game(nums: &[i32]) -> bool {
    let mut max_reach = 0usize;
    for (i, &jump) in nums.iter().enumerate() {
        if i > max_reach { return false; }
        max_reach = max_reach.max(i + jump as usize);
        if max_reach >= nums.len() - 1 { return true; }
    }
    true
}

// ── Medium 2: Jump Game II ───────────────────────────────────────────

struct JumpGameII;

struct JumpGameIITest {
    nums: Vec<i32>,
}

impl Problem for JumpGameII {
    fn id(&self) -> &str { "greedy_jump_game_ii" }
    fn name(&self) -> &str { "Jump Game II" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given an array of non-negative integers nums, you are initially at the first \
         index. Each element represents the maximum jump length from that position. \
         Return the minimum number of jumps to reach the last index. \
         It is guaranteed that you can reach the last index.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 10000\n\
         - 0 <= nums[i] <= 1000\n\n\
         Example: nums=[2,3,1,1,4] => 2 (jump 1 step to index 1, then 3 steps to end)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let mut nums: Vec<i32> = (0..n).map(|_| rng.random_range(1..=5)).collect();
            // Ensure reachable: set last element to 0 (doesn't matter)
            if n > 0 { nums[n - 1] = 0; }
            TestCase { data: Box::new(JumpGameIITest { nums }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<JumpGameIITest>().unwrap();
        let expected = ref_jump_game_ii(&t.nums);
        let actual = solutions::jump_game_ii(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_jump_game_ii(nums: &[i32]) -> i32 {
    if nums.len() <= 1 { return 0; }
    let mut jumps = 0;
    let mut current_end = 0usize;
    let mut farthest = 0usize;
    for i in 0..nums.len() - 1 {
        farthest = farthest.max(i + nums[i] as usize);
        if i == current_end {
            jumps += 1;
            current_end = farthest;
            if current_end >= nums.len() - 1 { break; }
        }
    }
    jumps
}

// ── Medium 3: Gas Station ────────────────────────────────────────────

struct GasStation;

struct GasStationTest {
    gas: Vec<i32>,
    cost: Vec<i32>,
}

impl Problem for GasStation {
    fn id(&self) -> &str { "greedy_gas_station" }
    fn name(&self) -> &str { "Gas Station" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "There are n gas stations along a circular route. gas[i] is the amount of gas at \
         station i, and cost[i] is the gas needed to travel from station i to i+1.\n\n\
         Return the starting station index if you can travel around the circuit once \
         clockwise, otherwise return -1. If a solution exists, it is guaranteed to be unique.\n\n\
         Constraints:\n\
         - 1 <= gas.len() == cost.len() <= 10000\n\
         - 0 <= gas[i], cost[i] <= 10000\n\n\
         Example: gas=[1,2,3,4,5], cost=[3,4,5,1,2] => 3"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=20);
            let gas: Vec<i32> = (0..n).map(|_| rng.random_range(0..=20)).collect();
            let cost: Vec<i32> = (0..n).map(|_| rng.random_range(0..=20)).collect();
            TestCase { data: Box::new(GasStationTest { gas, cost }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<GasStationTest>().unwrap();
        let expected = ref_gas_station(&t.gas, &t.cost);
        let actual = solutions::gas_station(&t.gas, &t.cost);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("gas={:?}, cost={:?}", t.gas, t.cost),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_gas_station(gas: &[i32], cost: &[i32]) -> i32 {
    let total: i32 = gas.iter().sum::<i32>() - cost.iter().sum::<i32>();
    if total < 0 { return -1; }
    let mut tank = 0;
    let mut start = 0;
    for i in 0..gas.len() {
        tank += gas[i] - cost[i];
        if tank < 0 {
            tank = 0;
            start = i + 1;
        }
    }
    start as i32
}

// ── Medium 4: Task Scheduler ─────────────────────────────────────────

struct TaskScheduler;

struct TaskSchedulerTest {
    tasks: Vec<char>,
    n: i32,
}

impl Problem for TaskScheduler {
    fn id(&self) -> &str { "greedy_task_scheduler" }
    fn name(&self) -> &str { "Task Scheduler" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a list of CPU tasks (each represented by a character A-Z) and a cooldown \
         interval n, return the minimum number of intervals the CPU needs to execute all \
         tasks. In each interval, the CPU can complete one task or remain idle. Identical \
         tasks must be separated by at least n intervals.\n\n\
         Constraints:\n\
         - 1 <= tasks.len() <= 10000\n\
         - tasks[i] is uppercase English letter\n\
         - 0 <= n <= 100\n\n\
         Example: tasks=['A','A','A','B','B','B'], n=2 => 8 (A B idle A B idle A B)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let len = rng.random_range(1..=30);
            let num_tasks = rng.random_range(1..=5);
            let tasks: Vec<char> = (0..len)
                .map(|_| (b'A' + rng.random_range(0..num_tasks as u8)) as char)
                .collect();
            let n = rng.random_range(0..=4);
            TestCase { data: Box::new(TaskSchedulerTest { tasks, n }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TaskSchedulerTest>().unwrap();
        let expected = ref_task_scheduler(&t.tasks, t.n);
        let actual = solutions::task_scheduler(&t.tasks, t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("tasks={:?}, n={}", t.tasks, t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_task_scheduler(tasks: &[char], n: i32) -> i32 {
    let mut freq = [0i32; 26];
    for &t in tasks {
        freq[(t as u8 - b'A') as usize] += 1;
    }
    freq.sort();
    let max_freq = freq[25];
    let max_count = freq.iter().filter(|&&f| f == max_freq).count() as i32;
    let result = (max_freq - 1) * (n + 1) + max_count;
    result.max(tasks.len() as i32)
}

// ── Medium 5: Partition Labels ───────────────────────────────────────

struct PartitionLabels;

struct PartitionLabelsTest {
    s: String,
}

impl Problem for PartitionLabels {
    fn id(&self) -> &str { "greedy_partition_labels" }
    fn name(&self) -> &str { "Partition Labels" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given a string s, partition it so that each letter appears in at most one part. \
         Return a list of the sizes of these parts.\n\n\
         The concatenation of all parts should equal s. Maximize the number of parts.\n\n\
         Constraints:\n\
         - 1 <= s.len() <= 500\n\
         - s consists of lowercase English letters\n\n\
         Example: s=\"ababcbacadefegdehijhklij\" => [9,7,8]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=40);
            let s = crate::problems::helpers::random_string_from(&mut rng, n, b"abcdefgh");
            TestCase { data: Box::new(PartitionLabelsTest { s }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<PartitionLabelsTest>().unwrap();
        let expected = ref_partition_labels(&t.s);
        let actual = solutions::partition_labels(&t.s);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("s=\"{}\"", t.s),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_partition_labels(s: &str) -> Vec<i32> {
    let bytes = s.as_bytes();
    let mut last = [0usize; 26];
    for (i, &b) in bytes.iter().enumerate() {
        last[(b - b'a') as usize] = i;
    }
    let mut result = Vec::new();
    let mut start = 0;
    let mut end = 0;
    for (i, &b) in bytes.iter().enumerate() {
        end = end.max(last[(b - b'a') as usize]);
        if i == end {
            result.push((end - start + 1) as i32);
            start = end + 1;
        }
    }
    result
}

// ── Hard 1: Candy ────────────────────────────────────────────────────

struct Candy;

struct CandyTest {
    ratings: Vec<i32>,
}

impl Problem for Candy {
    fn id(&self) -> &str { "greedy_candy" }
    fn name(&self) -> &str { "Candy" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "There are n children standing in a line. Each child has a rating. You give \
         candies to these children with these rules:\n\
         - Each child must have at least one candy.\n\
         - Children with a higher rating get more candies than their neighbors.\n\n\
         Return the minimum number of candies you must give.\n\n\
         Constraints:\n\
         - 1 <= ratings.len() <= 20000\n\n\
         Example: ratings=[1,0,2] => 5 (give [2,1,2])\n\
         Example: ratings=[1,2,2] => 4 (give [1,2,1])"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=30);
            let ratings: Vec<i32> = (0..n).map(|_| rng.random_range(0..=20)).collect();
            TestCase { data: Box::new(CandyTest { ratings }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CandyTest>().unwrap();
        let expected = ref_candy(&t.ratings);
        let actual = solutions::candy(&t.ratings);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("ratings={:?}", t.ratings),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_candy(ratings: &[i32]) -> i32 {
    let n = ratings.len();
    if n == 0 { return 0; }
    let mut candies = vec![1i32; n];
    // Left to right
    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }
    // Right to left
    for i in (0..n - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            candies[i] = candies[i].max(candies[i + 1] + 1);
        }
    }
    candies.iter().sum()
}

// ── Hard 2: Minimum Number of Arrows to Burst Balloons ───────────────

struct MinArrowsBurstBalloons;

struct MinArrowsTest {
    points: Vec<(i32, i32)>,
}

impl Problem for MinArrowsBurstBalloons {
    fn id(&self) -> &str { "greedy_min_number_arrows" }
    fn name(&self) -> &str { "Minimum Number of Arrows to Burst Balloons" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Balloons are represented as intervals [x_start, x_end]. An arrow shot at x \
         bursts all balloons where x_start <= x <= x_end. Return the minimum number \
         of arrows needed to burst all balloons.\n\n\
         Constraints:\n\
         - 1 <= points.len() <= 10000\n\
         - points[i] = (x_start, x_end), x_start <= x_end\n\n\
         Example: points=[(10,16),(2,8),(1,6),(7,12)] => 2"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=20);
            let points: Vec<(i32, i32)> = (0..n)
                .map(|_| {
                    let a = rng.random_range(-50..=50);
                    let b = rng.random_range(a..=a + 30);
                    (a, b)
                })
                .collect();
            TestCase { data: Box::new(MinArrowsTest { points }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MinArrowsTest>().unwrap();
        let expected = ref_min_arrows(&t.points);
        let actual = solutions::min_number_arrows(&t.points);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("points={:?}", t.points),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_min_arrows(points: &[(i32, i32)]) -> i32 {
    if points.is_empty() { return 0; }
    let mut sorted = points.to_vec();
    sorted.sort_by_key(|&(_, end)| end);
    let mut arrows = 1;
    let mut arrow_pos = sorted[0].1;
    for &(start, end) in &sorted[1..] {
        if start > arrow_pos {
            arrows += 1;
            arrow_pos = end;
        }
    }
    arrows
}

// ── Hard 3: Non-overlapping Intervals ────────────────────────────────

struct NonOverlappingIntervals;

struct NonOverlappingTest {
    intervals: Vec<(i32, i32)>,
}

impl Problem for NonOverlappingIntervals {
    fn id(&self) -> &str { "greedy_non_overlapping_intervals" }
    fn name(&self) -> &str { "Non-overlapping Intervals" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given an array of intervals where intervals[i] = (start_i, end_i), return \
         the minimum number of intervals you need to remove to make the rest non-overlapping.\n\n\
         Two intervals [a,b) and [c,d) overlap if c < b and a < d.\n\n\
         Constraints:\n\
         - 1 <= intervals.len() <= 10000\n\n\
         Example: intervals=[(1,2),(2,3),(3,4),(1,3)] => 1 (remove [1,3])"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=20);
            let intervals: Vec<(i32, i32)> = (0..n)
                .map(|_| {
                    let a = rng.random_range(-20..=20);
                    let b = rng.random_range(a + 1..=a + 15);
                    (a, b)
                })
                .collect();
            TestCase { data: Box::new(NonOverlappingTest { intervals }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<NonOverlappingTest>().unwrap();
        let expected = ref_erase_overlap(&t.intervals);
        let actual = solutions::non_overlapping_intervals(&t.intervals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("intervals={:?}", t.intervals),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_erase_overlap(intervals: &[(i32, i32)]) -> i32 {
    if intervals.is_empty() { return 0; }
    let mut sorted = intervals.to_vec();
    sorted.sort_by_key(|&(_, end)| end);
    let mut count = 0;
    let mut prev_end = sorted[0].1;
    for &(start, end) in &sorted[1..] {
        if start < prev_end {
            count += 1;
        } else {
            prev_end = end;
        }
    }
    count
}

// ── Hard 4: Queue Reconstruction by Height ───────────────────────────

struct QueueReconstruction;

struct QueueReconstructionTest {
    people: Vec<(i32, i32)>,
}

impl Problem for QueueReconstruction {
    fn id(&self) -> &str { "greedy_queue_reconstruction" }
    fn name(&self) -> &str { "Queue Reconstruction by Height" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "You are given an array of people where people[i] = (h_i, k_i). h_i is the \
         height and k_i is the number of people in front who have height >= h_i.\n\n\
         Reconstruct the queue. Return the reconstructed queue.\n\n\
         Constraints:\n\
         - 1 <= people.len() <= 2000\n\
         - 0 <= h_i <= 10^6\n\
         - 0 <= k_i < people.len()\n\n\
         Example: people=[(7,0),(4,4),(7,1),(5,0),(6,1),(5,2)]\n\
         => [(5,0),(7,0),(5,2),(6,1),(4,4),(7,1)]"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            // Build a valid queue, then shuffle to create the input
            let n = rng.random_range(1..=15);
            let mut heights: Vec<i32> = (0..n).map(|_| rng.random_range(1..=20)).collect();
            heights.sort();
            // Build the queue from shortest to tallest
            let mut queue: Vec<(i32, i32)> = Vec::new();
            for &h in &heights {
                let taller_or_equal = queue.iter().filter(|&&(qh, _)| qh >= h).count() as i32;
                let k = rng.random_range(0..=taller_or_equal);
                // Insert at position k among those >= h
                let mut pos = 0;
                let mut count = 0;
                while pos < queue.len() && count < k {
                    if queue[pos].0 >= h {
                        count += 1;
                    }
                    pos += 1;
                }
                // Find the insertion point
                while pos < queue.len() && queue[pos].0 < h {
                    pos += 1;
                }
                queue.insert(pos, (h, k));
            }
            // Compute k values from the actual queue
            let people: Vec<(i32, i32)> = queue.iter().enumerate().map(|(i, &(h, _))| {
                let k = queue[..i].iter().filter(|&&(qh, _)| qh >= h).count() as i32;
                (h, k)
            }).collect();
            // Shuffle the people array
            let mut shuffled = people;
            for i in (1..shuffled.len()).rev() {
                let j = rng.random_range(0..=i);
                shuffled.swap(i, j);
            }
            TestCase { data: Box::new(QueueReconstructionTest { people: shuffled }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<QueueReconstructionTest>().unwrap();
        let expected = ref_queue_reconstruction(&t.people);
        let actual = solutions::queue_reconstruction(&t.people);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("people={:?}", t.people),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_queue_reconstruction(people: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut sorted = people.to_vec();
    // Sort by height descending, then by k ascending
    sorted.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            b.0.cmp(&a.0)
        }
    });
    let mut result: Vec<(i32, i32)> = Vec::new();
    for (h, k) in sorted {
        result.insert(k as usize, (h, k));
    }
    result
}

// ── Hard 5: IPO ──────────────────────────────────────────────────────

struct Ipo;

struct IpoTest {
    k: i32,
    w: i32,
    profits: Vec<i32>,
    capital: Vec<i32>,
}

impl Problem for Ipo {
    fn id(&self) -> &str { "greedy_ipo" }
    fn name(&self) -> &str { "IPO" }
    fn topic(&self) -> &str { "greedy" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "You have initial capital w and can pick at most k projects. Each project i \
         requires capital[i] to start and yields profits[i] upon completion. Your capital \
         increases by the project's profit after finishing it.\n\n\
         Return the maximized final capital.\n\n\
         Constraints:\n\
         - 1 <= k <= 10^5\n\
         - 0 <= w <= 10^9\n\
         - profits.len() == capital.len() <= 10^5\n\
         - 0 <= profits[i], capital[i] <= 10^9\n\n\
         Example: k=2, w=0, profits=[1,2,3], capital=[0,1,1] => 4\n\
         (pick project 0 profit=1 => w=1, pick project 2 profit=3 => w=4)"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(1..=20);
            let k = rng.random_range(1..=n as i32);
            let w = rng.random_range(0..=50);
            let profits: Vec<i32> = (0..n).map(|_| rng.random_range(0..=100)).collect();
            let capital: Vec<i32> = (0..n).map(|_| rng.random_range(0..=50)).collect();
            TestCase { data: Box::new(IpoTest { k, w, profits, capital }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IpoTest>().unwrap();
        let expected = ref_ipo(t.k, t.w, &t.profits, &t.capital);
        let actual = solutions::ipo(t.k, t.w, &t.profits, &t.capital);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("k={}, w={}, profits={:?}, capital={:?}", t.k, t.w, t.profits, t.capital),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_ipo(k: i32, w: i32, profits: &[i32], capital: &[i32]) -> i32 {
    use std::collections::BinaryHeap;
    let n = profits.len();
    let mut projects: Vec<(i32, i32)> = capital.iter().copied().zip(profits.iter().copied()).collect();
    projects.sort_by_key(|&(c, _)| c);
    let mut current_w = w;
    let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut idx = 0;
    for _ in 0..k {
        while idx < n && projects[idx].0 <= current_w {
            max_heap.push(projects[idx].1);
            idx += 1;
        }
        if let Some(profit) = max_heap.pop() {
            current_w += profit;
        } else {
            break;
        }
    }
    current_w
}
