# Lesson 29: Greedy Algorithms

## The Idea: Always Take the Best Thing Right Now

You pull into a packed parking garage. Strategy A: grab the very first open spot you
see. Strategy B: drive past it, hoping for something closer to the entrance, risking
finding nothing. Strategy A is greedy -- you take the locally best option without
considering what lies ahead. Sometimes this works perfectly (the first spot IS great).
Sometimes it backfires (the first spot is on the roof, but the second floor was wide open).

A **greedy algorithm** builds a solution step by step, making the **locally optimal choice**
at each step, hoping that these local choices lead to a **globally optimal solution**. There
is no backtracking, no exploring alternatives, no memoizing subproblem results. You commit
to each choice and move on.

The critical question is always: *does greedy actually produce the optimal answer for this
problem?* For some problems it is provably optimal. For others it gives wrong answers.
Knowing the difference is the core skill.

---

## When Does Greedy Work? Two Required Properties

### 1. Greedy Choice Property

At each step, there exists a locally optimal choice that is part of *some* globally optimal
solution. You never need to reconsider a past decision.

### 2. Optimal Substructure

After making a greedy choice, the remaining problem is a smaller instance of the same type,
and the optimal solution to the whole contains the optimal solution to this subproblem.

If both hold, greedy works. If either fails, you typically need dynamic programming.

---

## Real-World Analogy: Making Change

You owe 87 cents and want the **fewest coins**. US denominations: 25c, 10c, 5c, 1c.

**Greedy strategy:** always pick the largest coin that fits.

```
  87c remaining
    Take 25c -> 62c remaining
    Take 25c -> 37c remaining
    Take 25c -> 12c remaining
    Take 10c ->  2c remaining
    Take  1c ->  1c remaining
    Take  1c ->  0c remaining

  Result: 6 coins (3 quarters + 1 dime + 2 pennies). Optimal for US coins.
```

Now change the denominations to {1, 3, 4} and target 6:

```
  GREEDY (largest first):           OPTIMAL:
  Take 4 -> 2 remaining            Take 3 -> 3 remaining
  Take 1 -> 1 remaining            Take 3 -> 0 remaining
  Take 1 -> 0 remaining
  = 3 coins  [4+1+1]               = 2 coins  [3+3]

  +----+---+---+                    +-----+-----+
  | 4  | 1 | 1 |                   |  3  |  3  |
  +----+---+---+                    +-----+-----+
  0    4   5   6                    0     3     6
```

Greedy fails because picking 4 -- the locally largest coin -- locks you into a subproblem
(making 2) that has no efficient solution. The greedy choice property does not hold here.
This needs dynamic programming.

---

## Proving Greedy Correct: Two Standard Techniques

### Technique 1: Exchange Argument

1. Assume an optimal solution OPT differs from the greedy solution GREEDY.
2. Find the first point where they diverge.
3. Show you can "swap" OPT's choice for GREEDY's choice without making things worse.
4. Repeat until OPT becomes GREEDY.

### Technique 2: Greedy Stays Ahead

1. Define a measure of progress (e.g., number of activities selected so far).
2. Show that after every step, the greedy solution is at least as good as any other
   solution by that measure.
3. Since greedy is never behind, it finishes at least as well as optimal.

The exchange argument says "any optimal can become greedy." Greedy-stays-ahead says
"greedy is never worse at any point, so it cannot be worse at the end."

---

## Classic Problem 1: Activity Selection

**Problem:** Given activities with start and end times (no overlapping allowed), select
the **maximum number** of non-overlapping activities.

```
  Activities (sorted by end time):

  Time:  0  1  2  3  4  5  6  7  8  9  10 11 12 13
         |  |  |  |  |  |  |  |  |  |  |  |  |  |
  A:     [=====]                                      end=2
  B:        [=====]                                   end=3
  C:     [============]                               end=5
  D:           [======]                               end=5
  E:              [======]                            end=7
  F:                    [=====]                       end=8
  G:                             [=========]          end=11
  H:                                   [=========]    end=13

  Greedy: sort by end time, always pick earliest-ending non-conflicting.

  Pick A (ends 2) -> Pick D (starts 2, ends 5) -> Pick F (starts 5, ends 8)
       -> Pick G (starts 8, ends 11) = 4 activities. Optimal.

  Selected (no overlaps):
  A:     [=====]
  D:           [======]
  F:                    [=====]
  G:                             [=========]
```

**Why it works (exchange argument):** If an optimal solution starts with some activity
other than the earliest-finishing one, swap it in. The replacement finishes no later, so
the rest of the schedule remains valid. One more step agrees with greedy. Repeat.

```rust
#[derive(Debug, Clone, Copy)]
struct Activity {
    start: u32,
    end: u32,
    id: char,
}

/// Returns the maximum set of non-overlapping activities.
/// Greedy strategy: sort by end time, pick earliest-ending that fits.
fn activity_selection(activities: &mut [Activity]) -> Vec<Activity> {
    activities.sort_by_key(|a| (a.end, a.start));

    let mut selected: Vec<Activity> = Vec::new();
    let mut last_end = 0;

    for &act in activities.iter() {
        if act.start >= last_end {
            selected.push(act);
            last_end = act.end;
        }
    }
    selected
}

fn main() {
    let mut acts = vec![
        Activity { start: 0, end: 2, id: 'A' },
        Activity { start: 1, end: 3, id: 'B' },
        Activity { start: 0, end: 5, id: 'C' },
        Activity { start: 2, end: 5, id: 'D' },
        Activity { start: 3, end: 7, id: 'E' },
        Activity { start: 5, end: 8, id: 'F' },
        Activity { start: 8, end: 11, id: 'G' },
        Activity { start: 9, end: 13, id: 'H' },
    ];
    let result = activity_selection(&mut acts);
    println!("Selected {} activities:", result.len());
    for a in &result {
        println!("  {} [{}, {})", a.id, a.start, a.end);
    }
}
```

**Time: O(n log n)** for the sort + O(n) scan. **Space: O(n)** for the output.

---

## Classic Problem 2: Fractional Knapsack

**Problem:** Knapsack with capacity W. Items have weight and value. You can take
**fractions** of items. Maximize total value.

**Greedy:** Sort items by value-per-unit-weight (descending). Take greedily.

```
  Capacity: 50 kg

  Item    Weight   Value   $/kg
  ─────────────────────────────
  A       10 kg    $60     $6.00
  B       20 kg    $100    $5.00
  C       30 kg    $120    $4.00

  Take all A (10kg, $60).  Remaining: 40kg.
  Take all B (20kg, $100). Remaining: 20kg.
  Take 2/3 of C (20kg, $80). Remaining: 0kg.

  Total: $240. Optimal.
```

```rust
#[derive(Debug)]
struct Item { weight: f64, value: f64 }

fn fractional_knapsack(items: &[Item], capacity: f64) -> f64 {
    // Build (value_density, index) pairs and sort descending by density.
    let mut by_density: Vec<(f64, usize)> = items
        .iter()
        .enumerate()
        .map(|(i, item)| (item.value / item.weight, i))
        .collect();
    by_density.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let mut remaining = capacity;
    let mut total = 0.0;

    for &(density, idx) in &by_density {
        if remaining <= 0.0 { break; }
        let take = remaining.min(items[idx].weight);
        total += take * density;
        remaining -= take;
    }
    total
}

fn main() {
    let items = vec![
        Item { weight: 10.0, value: 60.0 },
        Item { weight: 20.0, value: 100.0 },
        Item { weight: 30.0, value: 120.0 },
    ];
    println!("Max value: ${:.2}", fractional_knapsack(&items, 50.0));
    // Output: Max value: $240.00
}
```

**Time: O(n log n). Space: O(n).**

**Critical caveat:** This does NOT work for **0/1 knapsack** (take-it-or-leave-it). That
requires DP. The best-ratio item might block a superior combination of smaller items.

---

## Classic Problem 3: Huffman Coding

Huffman coding assigns shorter bit strings to frequent characters, minimizing total
encoding length. The greedy choice: always merge the two least-frequent nodes first.

```
  Frequencies:  a=45  b=13  c=12  d=16  e=9  f=5

  Build tree bottom-up (min-heap merges):

  Step 1: Merge f(5) + e(9)  -> (14)
  Step 2: Merge c(12) + b(13) -> (25)
  Step 3: Merge (14) + d(16) -> (30)
  Step 4: Merge (25) + (30)  -> (55)
  Step 5: Merge a(45) + (55) -> (100)

              (100)
             /     \
           a(45)  (55)
                 /    \
              (25)    (30)
             /   \    /   \
          c(12) b(13)(14) d(16)
                     /  \
                   f(5) e(9)

  Codes:  a=0  c=100  b=101  f=1100  e=1101  d=111
```

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// A node in the Huffman tree.
#[derive(Debug)]
enum HuffNode {
    Leaf(char, u32),
    Internal(u32, Box<HuffNode>, Box<HuffNode>),
}

impl HuffNode {
    fn freq(&self) -> u32 {
        match self {
            HuffNode::Leaf(_, f) => *f,
            HuffNode::Internal(f, _, _) => *f,
        }
    }
}

/// Build a Huffman tree from character frequencies.
fn build_huffman_tree(freqs: &[(char, u32)]) -> HuffNode {
    // Min-heap: Reverse wraps so BinaryHeap (max-heap) acts as min-heap.
    // We store (Reverse(freq), index_tiebreaker, node).
    let mut heap: BinaryHeap<(Reverse<u32>, usize, Box<HuffNode>)> = BinaryHeap::new();

    for (i, &(ch, freq)) in freqs.iter().enumerate() {
        heap.push((Reverse(freq), i, Box::new(HuffNode::Leaf(ch, freq))));
    }

    let mut counter = freqs.len(); // tiebreaker for equal frequencies

    while heap.len() > 1 {
        let (_, _, left) = heap.pop().unwrap();
        let (_, _, right) = heap.pop().unwrap();
        let combined_freq = left.freq() + right.freq();
        let parent = HuffNode::Internal(combined_freq, left, right);
        heap.push((Reverse(combined_freq), counter, Box::new(parent)));
        counter += 1;
    }

    *heap.pop().unwrap().2
}

/// Walk the tree to extract codes.
fn extract_codes(node: &HuffNode, prefix: &str, codes: &mut Vec<(char, String)>) {
    match node {
        HuffNode::Leaf(ch, _) => {
            // If tree has only one character, assign "0".
            let code = if prefix.is_empty() { "0".to_string() } else { prefix.to_string() };
            codes.push((*ch, code));
        }
        HuffNode::Internal(_, left, right) => {
            extract_codes(left, &format!("{prefix}0"), codes);
            extract_codes(right, &format!("{prefix}1"), codes);
        }
    }
}

fn main() {
    let freqs = vec![('a', 45), ('b', 13), ('c', 12), ('d', 16), ('e', 9), ('f', 5)];
    let tree = build_huffman_tree(&freqs);
    let mut codes = Vec::new();
    extract_codes(&tree, "", &mut codes);
    codes.sort_by_key(|&(ch, _)| ch);
    for (ch, code) in &codes {
        println!("  {} = {}", ch, code);
    }
}
```

**Time: O(n log n)** (n heap insertions/removals). **Space: O(n)** for the tree.

---

## Classic Problem 4: Jump Game (LeetCode 55)

**Problem:** `nums[i]` = max jump length from index i. Can you reach the last index?

```
  nums = [2, 3, 1, 1, 4]
  Index:  0  1  2  3  4
          ^
  Farthest reachable from 0: index 2
  At index 1: farthest = max(2, 1+3) = 4 >= last index. Done!

  nums = [3, 2, 1, 0, 4]
  Index:  0  1  2  3  4
  Farthest: 3, 3, 3, 3 -- stuck at 3 (jump=0), never reach 4.
```

**Greedy:** Track the farthest reachable position in a single scan.

```rust
fn can_jump(nums: &[i32]) -> bool {
    let mut farthest = 0usize;
    for i in 0..nums.len() {
        if i > farthest {
            return false; // stranded
        }
        farthest = farthest.max(i + nums[i] as usize);
        if farthest >= nums.len() - 1 {
            return true;
        }
    }
    true
}

fn main() {
    println!("{}", can_jump(&[2, 3, 1, 1, 4])); // true
    println!("{}", can_jump(&[3, 2, 1, 0, 4])); // false
}
```

**Time: O(n). Space: O(1).**

---

## Classic Problem 5: Gas Station (LeetCode 134)

**Problem:** n gas stations in a circle. Station i has `gas[i]` fuel and costs `cost[i]`
to reach the next station. Find the starting station to complete a full loop, or -1.

**Greedy insight:** If starting at station s causes you to run dry at station f, then
NO station between s and f works either (you arrived at each with non-negative fuel;
starting there with zero fuel is strictly worse). Skip ahead to f+1.

```rust
fn can_complete_circuit(gas: &[i32], cost: &[i32]) -> i32 {
    let total: i32 = gas.iter().zip(cost).map(|(g, c)| g - c).sum();
    if total < 0 {
        return -1; // not enough total fuel
    }

    let mut tank = 0;
    let mut start = 0;

    for i in 0..gas.len() {
        tank += gas[i] - cost[i];
        if tank < 0 {
            start = i + 1; // everything up to i fails; try i+1
            tank = 0;
        }
    }
    start as i32
}

fn main() {
    let gas  = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];
    println!("Start at station: {}", can_complete_circuit(&gas, &cost));
    // Output: Start at station: 3
}
```

**Time: O(n). Space: O(1).**

---

## Greedy vs Dynamic Programming

```
  +---------------------------------+------------------------------------+
  |            Greedy               |       Dynamic Programming          |
  +---------------------------------+------------------------------------+
  | Makes ONE choice per step       | Explores ALL choices per step      |
  | (the locally optimal one).      | and picks the globally best.       |
  +---------------------------------+------------------------------------+
  | Never revisits past decisions.  | Builds table of subproblem results |
  |                                 | to combine later.                  |
  +---------------------------------+------------------------------------+
  | Typically O(n log n) or O(n).   | Typically O(n^2), O(n*W), etc.     |
  +---------------------------------+------------------------------------+
  | Only works when greedy choice   | Works whenever optimal             |
  | property holds.                 | substructure exists (broader).     |
  +---------------------------------+------------------------------------+
  | Examples: activity selection,   | Examples: 0/1 knapsack, coin       |
  | fractional knapsack, Huffman,   | change (general), edit distance,   |
  | Dijkstra, Kruskal/Prim         | longest common subsequence         |
  +---------------------------------+------------------------------------+
```

**Decision heuristic:** If the problem has a clear sorting step after which you can
make irrevocable choices in a single scan, try greedy. If you find a counterexample
where the locally best choice leads to a suboptimal result, switch to DP.

Greedy is a special case of DP where only one subproblem matters per step. When greedy
applies, it is faster and simpler. When it does not, DP is the fallback.

---

## Summary: When Greedy Works vs. Fails

```
  Problem                    Greedy?  Why / Why Not
  ──────────────────────────────────────────────────────────────────
  Activity selection         YES      Earliest end time leaves most room
  Fractional knapsack        YES      Best ratio items should go first
  0/1 knapsack               NO       Best-ratio item may block better combos
  Coin change (US coins)     YES      Denominations have special structure
  Coin change (arbitrary)    NO       Largest coin may block better combos
  Jump game (reachable?)     YES      Track farthest reachable in one pass
  Huffman coding             YES      Merging cheapest nodes minimizes cost
  Dijkstra's shortest path   YES      Non-negative weights guarantee safety
  Minimum spanning tree      YES      Cut property guarantees safe edges
  Longest common subseq      NO       Local match may misalign global LCS
  Edit distance              NO       Overlapping subproblems, no greedy choice
```

---

## Common Interview Patterns

1. **Sort-then-scan.** The majority of greedy problems: sort by some criterion
   (end time, ratio, deadline), then iterate once making irrevocable choices.

2. **Tracking a running maximum/minimum.** Jump Game, Gas Station -- maintain a
   single variable (farthest reach, running tank) updated each step.

3. **Interval problems.** Activity selection, merge intervals, minimum meeting rooms,
   burst balloons. Almost always: sort by start or end time, then greedily select/merge.

4. **Two-pointer greedy.** Assign Cookies, boats to save people -- sort both arrays,
   use two pointers to match greedily.

5. **Heap-assisted greedy.** Huffman, task scheduler, reorganize string -- use a
   priority queue to repeatedly extract the best candidate.

**Recognizing greedy in an interview:** If the problem says "maximum number," "minimum
cost," or "fewest operations" and you can sort the input to reveal a natural one-pass
strategy, try greedy first. Sketch a proof or find a counterexample before coding.

---

## Algorithms You Already Know That Are Greedy

- **Dijkstra's algorithm** ([Lesson 25](./25_shortest_path.md)): always expand the
  closest unvisited vertex. Safe because non-negative weights mean closer vertices
  cannot be improved by longer paths.

- **Kruskal's / Prim's** ([Lesson 26](./26_minimum_spanning_tree.md)): always pick the
  cheapest safe edge. The cut property guarantees optimality.

---

## Key Takeaways

1. **Greedy = commit to the locally best choice. No backtracking, no table.** Fast and
   simple when it works.

2. **Requires both greedy choice property and optimal substructure.** If the locally
   optimal choice might not be part of a global optimum, greedy fails silently.

3. **Prove it.** Use the exchange argument ("swap OPT's choice for greedy's without
   hurting") or greedy-stays-ahead ("greedy is never behind at any step").

4. **Sort first, scan once.** The dominant pattern. The key insight is *what to sort by*.

5. **Greedy fails silently** -- it gives *an* answer, just not necessarily the right one.
   Always verify with counterexamples or a correctness argument.

---

## Practice Problems

### Easy (5)

| # | Problem | Key Idea |
|---|---------|----------|
| 1 | [LeetCode 455 - Assign Cookies](https://leetcode.com/problems/assign-cookies/) | Sort both arrays, two-pointer greedy |
| 2 | [LeetCode 860 - Lemonade Change](https://leetcode.com/problems/lemonade-change/) | Greedily make change with largest bills first |
| 3 | [LeetCode 1005 - Maximize Sum After K Negations](https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/) | Flip smallest (most negative) values first |
| 4 | [LeetCode 605 - Can Place Flowers](https://leetcode.com/problems/can-place-flowers/) | Greedily plant at the earliest valid spot |
| 5 | [LeetCode 976 - Largest Perimeter Triangle](https://leetcode.com/problems/largest-perimeter-triangle/) | Sort descending, check consecutive triples |

### Medium (5)

| # | Problem | Key Idea |
|---|---------|----------|
| 1 | [LeetCode 55 - Jump Game](https://leetcode.com/problems/jump-game/) | Track farthest reachable index |
| 2 | [LeetCode 134 - Gas Station](https://leetcode.com/problems/gas-station/) | Reset start when tank goes negative |
| 3 | [LeetCode 452 - Min Arrows to Burst Balloons](https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/) | Sort by end, count non-overlapping groups |
| 4 | [LeetCode 763 - Partition Labels](https://leetcode.com/problems/partition-labels/) | Track last occurrence, extend partition greedily |
| 5 | [LeetCode 45 - Jump Game II](https://leetcode.com/problems/jump-game-ii/) | BFS-like layers: track current range end |

### Hard (5)

| # | Problem | Key Idea |
|---|---------|----------|
| 1 | [LeetCode 135 - Candy](https://leetcode.com/problems/candy/) | Two-pass greedy (left-to-right, right-to-left) |
| 2 | [LeetCode 316 - Remove Duplicate Letters](https://leetcode.com/problems/remove-duplicate-letters/) | Greedy stack with frequency tracking |
| 3 | [LeetCode 630 - Course Schedule III](https://leetcode.com/problems/course-schedule-iii/) | Sort by deadline, max-heap to drop longest course |
| 4 | [LeetCode 871 - Minimum Number of Refueling Stops](https://leetcode.com/problems/minimum-number-of-refueling-stops/) | Max-heap of passed stations, refuel greedily |
| 5 | [LeetCode 968 - Binary Tree Cameras](https://leetcode.com/problems/binary-tree-cameras/) | Post-order greedy: place cameras at parents of leaves |

---

*Next up: Dynamic Programming, where we tackle the problems greedy cannot solve -- by
systematically exploring all choices and remembering the results.*
