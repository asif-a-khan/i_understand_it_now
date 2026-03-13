# Lesson 32: Intervals & Merge Intervals

## What This Lesson Covers

You have a list of time ranges. Some overlap, some don't. You need to merge the
overlapping ones, or figure out how many rooms you need for simultaneous meetings, or
find the gaps in a schedule. These are **interval problems**, and they show up constantly
in both interviews and real systems -- calendar apps, resource schedulers, database query
planners, operating system memory allocators.

The good news: nearly every interval problem follows the same playbook. **Sort, then
sweep.** Once you internalize this pattern, the whole category opens up.

We will cover:

1. Real-world analogy and intuition
2. Formal definition -- what intervals are, overlap detection, merge conditions
3. Sorting strategies for interval problems
4. Core patterns with full Rust implementations
5. The sweep line technique
6. Time/space complexity analysis
7. Common interview patterns and recognition tips
8. Practice problems (easy, medium, hard)

---

## 1. Real-World Analogy: The Shared Calendar

Imagine you work at a company where every team shares one physical conference room. Each
team submits booking requests -- time ranges like "9:00-10:30" or "10:00-11:00". Your
job as the office manager is to answer questions like:

- **"When is the room actually busy?"** -- Merge overlapping bookings into continuous
  busy blocks. Two bookings overlap if one starts before the other ends.
- **"Can one person attend all these meetings?"** -- Check if any two bookings overlap.
  If they do, one person cannot be in both.
- **"How many rooms do we need?"** -- Find the maximum number of meetings happening at
  the same instant. That peak is your room count.
- **"A new meeting just came in -- what does the schedule look like now?"** -- Insert a
  new booking into the existing schedule and merge any resulting overlaps.
- **"What free slots do two people share?"** -- Intersect two calendars to find mutual
  availability.

Every one of these maps directly to a classic interval problem. The data structure is
always the same: a list of `[start, end]` pairs. The approach is nearly always the same:
sort them, then walk through from left to right.

```
  Your calendar for the day:

  8:00   9:00  10:00  11:00  12:00  1:00   2:00   3:00   4:00   5:00
  |------|------|------|------|------|------|------|------|------|------|
         [==Team Standup==]
                [====Design Review=====]
                       [=1:1 w/ Manager=]
                                                  [===Sprint Demo===]
                                                         [==Retro==]

  After merging overlapping meetings:

  8:00   9:00  10:00  11:00  12:00  1:00   2:00   3:00   4:00   5:00
  |------|------|------|------|------|------|------|------|------|------|
         [========== BUSY ===========]
                                                  [==== BUSY ======]
```

---

## 2. What Is an Interval?

An interval is a pair `[start, end]` representing a contiguous range on a number line:

```
  Interval [2, 6]:

  0   1   2===3===4===5===6   7   8   9
            |_____________|
            start         end
```

### Representation in Rust

For interview problems, a simple two-element array works:

```rust
/// Most interval problems use this representation.
type Interval = [i32; 2]; // [start, end]

// Or if you prefer named fields:
#[derive(Clone, Debug)]
struct NamedInterval {
    start: i32,
    end: i32,
}
```

We will use `Vec<[i32; 2]>` throughout this lesson, since that matches how LeetCode and
most interview platforms represent intervals.

### Overlap Detection

Two intervals overlap if one starts before the other ends:

```
  Overlapping (partial):
  A: [1, 5]      |=====|
  B: [3, 8]          |==========|
                  1  2  3  4  5  6  7  8

  Overlapping (containment):
  A: [1, 8]      |================|
  B: [3, 5]          |====|
                  1  2  3  4  5  6  7  8

  Not overlapping (gap between them):
  A: [1, 3]      |===|
  B: [5, 8]                |========|
                  1  2  3  4  5  6  7  8

  Edge-touching (problem-dependent):
  A: [1, 3]      |===|
  B: [3, 6]          |======|
                  1  2  3  4  5  6
```

Formally, intervals `[a_start, a_end]` and `[b_start, b_end]` overlap when:

```
a_start < b_end  AND  b_start < a_end
```

If the problem treats touching endpoints as overlapping (many merge problems do), change
`<` to `<=`. Read the problem statement carefully -- this one-character difference is a
common source of bugs.

### Merge Condition

When two sorted intervals overlap, their merge is:

```
merged_start = min(a_start, b_start)
merged_end   = max(a_end, b_end)
```

For the containment case `[1, 10]` and `[3, 5]`, the merge is `[1, 10]` -- the smaller
interval is absorbed entirely.

---

## 3. Sorting Strategies for Interval Problems

Almost every interval problem starts with sorting. The choice of sort key determines
whether your algorithm is correct.

### Sort by Start Time

Used for: merge intervals, insert interval, meeting rooms I, interval intersection.

After sorting by start, intervals are ordered left-to-right on the timeline. Overlaps
only occur between consecutive intervals, so you can process them in a single pass.

```
  Before sorting (hard to reason about):
  [8,10]  [1,3]  [15,18]  [2,6]

  After sorting by start (structure becomes clear):
  [1,3]  [2,6]  [8,10]  [15,18]
   ^      ^               ^
   These two overlap       This starts a new group
```

### Sort by End Time

Used for: activity selection (non-overlapping intervals), minimum arrows to burst
balloons, any problem asking "maximize the number of non-overlapping intervals."

Sorting by end ensures the greedy strategy of "pick the earliest-ending interval" works
correctly. Sorting by start would fail:

```
  Sort by start (WRONG for activity selection):
  [1, 100]  [2, 3]  [4, 5]
  Greedy picks [1,100] first, blocking everything.

  Sort by end (CORRECT):
  [2, 3]  [4, 5]  [1, 100]
  Greedy picks [2,3], then [4,5]. Optimal: 2 activities.
```

### When to Sort by What

| Problem Type | Sort By | Why |
|---|---|---|
| Merge overlapping | Start | Process left-to-right, extend or start new |
| Insert into sorted list | (already sorted) | Three-phase sweep |
| Greedy selection / max non-overlapping | End | Earliest finish leaves most room |
| Conflict detection | Start | Check adjacent pairs |
| Sweep line (count simultaneous) | Event time | Process events chronologically |
| Two-list intersection | (already sorted) | Two pointers, advance earlier end |

---

## 4. Core Patterns with Full Rust Implementations

### 4.1 Merge Overlapping Intervals

**Problem:** Given a collection of intervals, merge all overlapping intervals.

```
Input:  [[1,3], [2,6], [8,10], [15,18]]
Output: [[1,6], [8,10], [15,18]]
```

**Step-by-step walkthrough:**

```
  Input (unsorted): [1,3]  [8,10]  [2,6]  [15,18]

  Step 1 -- Sort by start:
  [1,3]  [2,6]  [8,10]  [15,18]

  Step 2 -- Sweep, merging as we go:

  Start with merged = [[1,3]]

  Process [2,6]:
    Last merged end = 3. Does 2 <= 3? Yes, overlap.
    Extend: end = max(3, 6) = 6.
    merged = [[1,6]]

  Process [8,10]:
    Last merged end = 6. Does 8 <= 6? No.
    Push new interval.
    merged = [[1,6], [8,10]]

  Process [15,18]:
    Last merged end = 10. Does 15 <= 10? No.
    Push new interval.
    merged = [[1,6], [8,10], [15,18]]

  Timeline:
    0  1==========6  7  8==10  ...  15===18
       |_________|      |__|        |____|
```

**The key insight:** after sorting by start, you only compare each interval with the
*last* interval in your merged result. If the current start is within the last merged
end, they overlap and you extend. Otherwise, push a new interval.

```rust
fn merge(mut intervals: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    if intervals.is_empty() {
        return vec![];
    }

    intervals.sort_by_key(|iv| iv[0]);

    let mut merged: Vec<[i32; 2]> = vec![intervals[0]];

    for i in 1..intervals.len() {
        let current = intervals[i];
        let last = merged.last_mut().unwrap();

        if current[0] <= last[1] {
            // Overlapping -- extend the end.
            last[1] = last[1].max(current[1]);
        } else {
            // No overlap -- start a new merged interval.
            merged.push(current);
        }
    }

    merged
}

fn main() {
    let intervals = vec![[1, 3], [2, 6], [8, 10], [15, 18]];
    println!("{:?}", merge(intervals));
    // [[1, 6], [8, 10], [15, 18]]

    // Containment case: [2,3] is absorbed by [1,4]
    let intervals = vec![[1, 4], [2, 3]];
    println!("{:?}", merge(intervals));
    // [[1, 4]]
}
```

**Why only compare with the last merged interval?** Because intervals are sorted by
start. If the current interval does not overlap with the last merged one, it cannot
overlap with any earlier merged interval (they all end even earlier).

**Edge cases:**
- Empty input: return empty.
- Single interval: return it unchanged.
- All intervals overlap: one big merged interval.
- No overlaps: return the sorted input.
- Containment: `[1,10]` and `[3,5]` merges to `[1,10]` thanks to `max(end)`.

**Complexity:** O(n log n) time (sort dominates), O(n) space (output).

---

### 4.2 Insert Interval

**Problem:** Given a sorted, non-overlapping list of intervals and a new interval,
insert the new interval and merge if necessary.

```
Input:  intervals = [[1,2], [3,5], [6,7], [8,10], [12,16]], new = [4,8]
Output: [[1,2], [3,10], [12,16]]
```

**Visual walkthrough:**

```
  Existing:  1=2  3==5  6=7  8==10  12===16
  New:                [4,==========8]

  Phase 1 -- before overlap (end < new_start):
    [1,2] ends at 2 < 4. Keep as-is.

  Phase 2 -- overlapping (start <= new_end):
    [3,5]: overlap. Merge -> [min(3,4), max(5,8)] = [3,8]
    [6,7]: overlap. Merge -> [min(3,6), max(8,7)] = [3,8]
    [8,10]: overlap. Merge -> [min(3,8), max(8,10)] = [3,10]

  Phase 3 -- after overlap (start > new_end):
    [12,16] starts at 12 > 10. Keep as-is.

  Result: [[1,2], [3,10], [12,16]]
```

**Three phases in a single pass:**

```rust
fn insert(intervals: Vec<[i32; 2]>, new_interval: [i32; 2]) -> Vec<[i32; 2]> {
    let mut result = Vec::new();
    let mut new = new_interval;
    let mut i = 0;
    let n = intervals.len();

    // Phase 1: intervals entirely before the new one.
    while i < n && intervals[i][1] < new[0] {
        result.push(intervals[i]);
        i += 1;
    }

    // Phase 2: overlapping intervals -- merge into new.
    while i < n && intervals[i][0] <= new[1] {
        new[0] = new[0].min(intervals[i][0]);
        new[1] = new[1].max(intervals[i][1]);
        i += 1;
    }
    result.push(new);

    // Phase 3: intervals entirely after.
    while i < n {
        result.push(intervals[i]);
        i += 1;
    }

    result
}

fn main() {
    let intervals = vec![[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]];
    println!("{:?}", insert(intervals, [4, 8]));
    // [[1, 2], [3, 10], [12, 16]]

    // No overlap at front
    let intervals = vec![[1, 5]];
    println!("{:?}", insert(intervals, [0, 0]));
    // [[0, 0], [1, 5]]
}
```

**Complexity:** O(n) time (input is already sorted, one pass). O(n) space for output.

---

### 4.3 Interval Intersection

**Problem:** Given two lists of intervals (each sorted and non-overlapping within
itself), find all intervals that appear in both lists.

```
A = [[0,2], [5,10], [13,23], [24,25]]
B = [[1,5], [8,12], [15,24], [25,26]]
Output: [[1,2], [5,5], [8,10], [15,23], [24,24], [25,25]]
```

**Visual:**

```
  A: [0,==2]     [5,==========10]       [13,=================23] [24,25]
  B:    [1,=======5]    [8,======12]          [15,============24]    [25,26]
       ----+----+----+----+----+----+----+----+----+----+----+----+----+----
       0   1    2    5         8   10        13   15        23 24 25  26

  Intersections (where both have coverage):
       [1,2]    [5,5]  [8,10]           [15,=========23] [24] [25]
```

**Two-pointer approach:** check if current intervals from A and B overlap. If so, the
intersection is `[max(starts), min(ends)]`. Advance whichever pointer ends first.

```rust
fn interval_intersection(a: &[[i32; 2]], b: &[[i32; 2]]) -> Vec<[i32; 2]> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        let start = a[i][0].max(b[j][0]);
        let end = a[i][1].min(b[j][1]);

        if start <= end {
            result.push([start, end]);
        }

        // Advance the pointer whose interval ends first.
        if a[i][1] < b[j][1] {
            i += 1;
        } else {
            j += 1;
        }
    }

    result
}

fn main() {
    let a = vec![[0, 2], [5, 10], [13, 23], [24, 25]];
    let b = vec![[1, 5], [8, 12], [15, 24], [25, 26]];
    println!("{:?}", interval_intersection(&a, &b));
    // [[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]]
}
```

**Why advance the earlier-ending interval?** If A's current interval ends before B's,
then A's interval cannot overlap with any future B interval (those all start after B's
current start, which is already past A's end). The interval that ends later might still
overlap with the next one from the other list.

**Complexity:** O(n + m) time, O(n + m) space for output.

---

### 4.4 Meeting Rooms I: Can One Person Attend All Meetings?

**Problem:** Given meeting time intervals, determine if a person could attend all.

```
Input:  [[0,30], [5,10], [15,20]]
Output: false  ([5,10] overlaps with [0,30])

Input:  [[7,10], [2,4]]
Output: true
```

Sort by start, then check if any meeting starts before the previous one ends:

```rust
fn can_attend_meetings(mut intervals: Vec<[i32; 2]>) -> bool {
    intervals.sort_by_key(|iv| iv[0]);

    for i in 1..intervals.len() {
        if intervals[i][0] < intervals[i - 1][1] {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", can_attend_meetings(vec![[0, 30], [5, 10], [15, 20]]));
    // false
    println!("{}", can_attend_meetings(vec![[7, 10], [2, 4]]));
    // true
}
```

This is the simplest interval problem: "are there any overlaps after sorting?"

**Complexity:** O(n log n) time. O(log n) space (sort stack).

---

### 4.5 Meeting Rooms II: Minimum Rooms Needed

**Problem:** Given meeting intervals, find the minimum number of conference rooms so
that no two overlapping meetings share a room.

```
Input:  [[0,30], [5,10], [15,20]]
Output: 2

Timeline visualization:
  Room 1: |======================== [0,30] ==========================|
  Room 2:      |=[5,10]=|     |==[15,20]==|
          0    5         10   15          20                         30
```

This is where the **sweep line** technique shines. See section 5 for the full
explanation. Here is the implementation:

```rust
fn min_meeting_rooms(intervals: &[[i32; 2]]) -> i32 {
    let mut events: Vec<(i32, i32)> = Vec::new();

    for iv in intervals {
        events.push((iv[0], 1));   // meeting start: +1
        events.push((iv[1], -1));  // meeting end:   -1
    }

    // Sort by time. Ends (-1) sort before starts (+1) at the same time.
    events.sort();

    let mut active = 0;
    let mut max_active = 0;

    for &(_, delta) in &events {
        active += delta;
        max_active = max_active.max(active);
    }

    max_active
}

fn main() {
    println!("{}", min_meeting_rooms(&[[0, 30], [5, 10], [15, 20]]));
    // 2
    println!("{}", min_meeting_rooms(&[[7, 10], [2, 4]]));
    // 1
}
```

**Alternative: two sorted arrays with a two-pointer merge:**

```rust
fn min_meeting_rooms_two_arrays(intervals: &[[i32; 2]]) -> i32 {
    let mut starts: Vec<i32> = intervals.iter().map(|iv| iv[0]).collect();
    let mut ends: Vec<i32> = intervals.iter().map(|iv| iv[1]).collect();

    starts.sort();
    ends.sort();

    let mut rooms = 0;
    let mut max_rooms = 0;
    let (mut s, mut e) = (0, 0);

    while s < starts.len() {
        if starts[s] < ends[e] {
            rooms += 1;
            max_rooms = max_rooms.max(rooms);
            s += 1;
        } else {
            rooms -= 1;
            e += 1;
        }
    }

    max_rooms
}
```

Both approaches: O(n log n) time, O(n) space.

---

### 4.6 Non-Overlapping Intervals (Minimum Removals)

**Problem:** Find the minimum number of intervals to remove so the rest do not overlap.
Equivalently: find the maximum set of non-overlapping intervals you can keep.

```
Input:  [[1,2], [2,3], [3,4], [1,3]]
Output: 1  (remove [1,3])
```

**The greedy insight:** sort by end time. Greedily keep the interval that finishes
earliest (it leaves the most room for future intervals). Skip anything that overlaps
with the last kept interval. This is the classic **activity selection** problem.

```
  Sorted by end: [1,2]  [2,3]  [1,3]  [3,4]

  Keep [1,2] (ends earliest). last_end = 2.
  Keep [2,3] (2 >= 2, no overlap). last_end = 3.
  Skip [1,3] (1 < 3, overlaps).
  Keep [3,4] (3 >= 3, no overlap). last_end = 4.

  Kept 3, removed 1.
```

**Why not sort by start?** Consider `[1,100], [2,3], [4,5]`. Picking `[1,100]` first
blocks everything. Sort-by-end picks `[2,3]` and `[4,5]` instead -- the optimal answer.

```rust
fn erase_overlap_intervals(mut intervals: Vec<[i32; 2]>) -> i32 {
    if intervals.is_empty() {
        return 0;
    }

    // Sort by end time -- this is critical.
    intervals.sort_by_key(|iv| iv[1]);

    let mut kept = 1;
    let mut last_end = intervals[0][1];

    for i in 1..intervals.len() {
        if intervals[i][0] >= last_end {
            kept += 1;
            last_end = intervals[i][1];
        }
    }

    intervals.len() as i32 - kept
}

fn main() {
    let intervals = vec![[1, 2], [2, 3], [3, 4], [1, 3]];
    println!("{}", erase_overlap_intervals(intervals));
    // 1
}
```

**Complexity:** O(n log n) time. O(log n) space.

---

## 5. The Sweep Line Technique

The sweep line is the most powerful technique for interval problems that ask about
simultaneous overlap counts. Instead of treating intervals as atomic units, you
decompose each interval into two **events**:

```
  Interval [5, 10]  -->  Event at time 5: +1 (something starts)
                         Event at time 10: -1 (something ends)
```

Then sort all events by time and sweep left to right, maintaining a running count of
how many intervals are currently active.

### Full Walkthrough

```
  Meetings: [0,30], [5,10], [15,20]

  Decompose into events:
    time=0:  +1  (meeting 1 starts)
    time=5:  +1  (meeting 2 starts)
    time=10: -1  (meeting 2 ends)
    time=15: +1  (meeting 3 starts)
    time=20: -1  (meeting 3 ends)
    time=30: -1  (meeting 1 ends)

  Sort events by time (ends before starts at same time):

  Sweep:
    time   event   active_count
    ----   -----   ------------
    0      +1      1
    5      +1      2           <-- peak
    10     -1      1
    15     +1      2           <-- ties peak
    20     -1      1
    30     -1      0

  Max simultaneous: 2
```

### Tie-Breaking Matters

When an end and a start happen at the same time (meeting A ends at 10, meeting B starts
at 10), they should NOT be counted as simultaneous. Process ends before starts at the
same time. Sorting tuples `(time, delta)` where delta is `-1` for ends and `+1` for
starts achieves this naturally because `-1 < +1`:

```rust
events.sort(); // (time, delta) where delta is -1 or +1
```

### When to Use Sweep Line

- "How many intervals overlap at the peak?" (meeting rooms II)
- "At what time are the most events happening?"
- "Find all points covered by at least k intervals"
- Any problem where you need the overlap count at various points in time

### The General Sweep Line Template

```rust
fn sweep_line(intervals: &[[i32; 2]]) -> i32 {
    let mut events: Vec<(i32, i32)> = Vec::new();

    for iv in intervals {
        events.push((iv[0], 1));   // start event
        events.push((iv[1], -1));  // end event
    }

    events.sort(); // ends before starts at same time

    let mut active = 0;
    let mut max_active = 0;

    for &(_, delta) in &events {
        active += delta;
        max_active = max_active.max(active);
    }

    max_active
}
```

### Sweep Line with a Sorted Map (for Dynamic Scenarios)

When intervals arrive one at a time (like in My Calendar III), you can use a `BTreeMap`
instead of sorting a flat list. Each insertion updates the event counts, and you sweep
the map to find the peak:

```rust
use std::collections::BTreeMap;

struct MyCalendarThree {
    events: BTreeMap<i32, i32>,
}

impl MyCalendarThree {
    fn new() -> Self {
        Self { events: BTreeMap::new() }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.events.entry(start).or_insert(0) += 1;
        *self.events.entry(end).or_insert(0) -= 1;

        let mut active = 0;
        let mut max_active = 0;
        for &delta in self.events.values() {
            active += delta;
            max_active = max_active.max(active);
        }
        max_active
    }
}
```

---

## 6. Time & Space Complexity Summary

| Problem | Technique | Time | Space |
|---|---|---|---|
| Merge Intervals | Sort by start, merge adjacent | O(n log n) | O(n) |
| Insert Interval | Three-phase linear scan | O(n) | O(n) |
| Interval Intersection | Two pointers | O(n + m) | O(n + m) |
| Meeting Rooms I | Sort by start, check adjacent | O(n log n) | O(log n)* |
| Meeting Rooms II | Sweep line with events | O(n log n) | O(n) |
| Non-Overlapping Intervals | Sort by end, greedy select | O(n log n) | O(log n)* |
| Burst Balloons (Arrows) | Sort by end, greedy shoot | O(n log n) | O(log n)* |

*excluding output; O(log n) for the sort's internal stack space.

**Why O(n log n) is optimal for most interval problems:** You need to examine every
interval at least once: O(n) lower bound. Sorting is O(n log n) and dominates. For
comparison-based approaches, you cannot beat O(n log n) because determining interval
order requires sorting. The exception is Insert Interval, where the input arrives
pre-sorted, allowing an O(n) single pass.

---

## 7. Common Interview Patterns and Recognition Tips

### How to Recognize an Interval Problem

Look for these signals in the problem statement:

- Words like "intervals", "ranges", "meetings", "bookings", "segments", "time slots"
- Input is pairs of numbers representing start and end
- You need to find overlaps, gaps, merges, or simultaneous counts
- The problem involves scheduling, resource allocation, or coverage

### The Decision Tree

```
  Is the input a list of [start, end] pairs?
  |
  +-- YES --> Do you need to merge overlapping ones?
  |           +-- YES --> Sort by start, merge adjacent (Sec 4.1)
  |           |
  |           +-- NO --> Do you need the max simultaneous count?
  |                      +-- YES --> Sweep line with events (Sec 5)
  |                      |
  |                      +-- NO --> Do you need max non-overlapping?
  |                                 +-- YES --> Sort by END, greedy (Sec 4.6)
  |                                 |
  |                                 +-- NO --> Are there two sorted lists?
  |                                            +-- YES --> Two pointers (Sec 4.3)
  |                                            +-- NO --> Sort by start,
  |                                                       single-pass logic
  +-- NO --> Probably not an interval problem
```

### The Sort-Then-Sweep Template

Nearly every interval solution follows this skeleton:

```rust
fn solve_interval_problem(mut intervals: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    // Step 1: Sort (by start or end depending on the problem).
    intervals.sort_by_key(|iv| iv[0]); // or iv[1]

    // Step 2: Initialize state for the sweep.
    let mut result: Vec<[i32; 2]> = Vec::new();

    // Step 3: Sweep through, updating state.
    for iv in &intervals {
        // Compare iv with current state.
        // Either merge/extend, or finalize and start new.
    }

    // Step 4: Return.
    result
}
```

### The Playbook at a Glance

| Pattern | Sort By | Sweep Logic | Example Problems |
|---------|---------|-------------|------------------|
| Merge overlapping | Start | Extend or start new | Merge Intervals |
| Insert into sorted | (already sorted) | Three phases | Insert Interval |
| Greedy selection | End | Pick earliest-ending | Non-overlapping, Burst Balloons |
| Conflict detection | Start | Check adjacent pairs | Meeting Rooms I |
| Sweep line / count | Time (events) | Track +1/-1 | Meeting Rooms II |
| Two-list intersection | (already sorted) | Two pointers | Interval Intersections |

### Common Mistakes

**1. Forgetting to sort.** If the input is not guaranteed sorted, you must sort it. Many
interval algorithms silently produce wrong results on unsorted input without crashing.

**2. Wrong sort key.** Merge = sort by start. Activity selection = sort by end. Using the
wrong key does not crash -- it gives wrong answers on specific inputs that you might not
catch until you submit.

**3. Off-by-one with endpoint inclusion.** Some problems treat `[1,3]` and `[3,5]` as
overlapping; others do not. The difference is `<=` vs `<` in one comparison. Read the
problem statement carefully.

**4. Not handling containment in merge.** If `[1,10]` is followed by `[3,5]`, the merge
is `[1,10]`, not `[1,5]`. Using `max(last_end, current_end)` handles this. Using
`current_end` directly does not.

**5. Sweep line tie-breaking.** In meeting rooms II, if a meeting ends at 10 and another
starts at 10, they should not count as simultaneous. Process end events before start
events at the same time. The `(time, -1/+1)` tuple sort handles this naturally.

---

## 8. Practice Problems

### Easy (5)

1. **Merge Intervals** (LeetCode 56) -- Sort by start, merge adjacent overlapping
   intervals. The canonical interval problem. Test with contained intervals
   (`[[1,4],[2,3]]` should produce `[[1,4]]`).

2. **Meeting Rooms** (LeetCode 252) -- Determine if a person can attend all meetings.
   Sort by start, check for any adjacent overlap.

3. **Summary Ranges** (LeetCode 228) -- Given a sorted unique integer array, return the
   smallest sorted list of ranges that cover all numbers. A warm-up for interval
   thinking.

4. **Minimum Number of Arrows to Burst Balloons** (LeetCode 452) -- Sort by end,
   greedily shoot arrows. Each arrow at the end of the earliest-ending balloon bursts
   as many overlapping ones as possible.

5. **Determine if Two Events Have Conflict** (LeetCode 2446) -- Given two events as
   time strings, check if they overlap. Good for practicing the overlap condition.

### Medium (5)

6. **Insert Interval** (LeetCode 57) -- Insert a new interval into a sorted
   non-overlapping list and merge. Three-phase linear scan.

7. **Non-overlapping Intervals** (LeetCode 435) -- Find minimum removals so no two
   intervals overlap. Sort by end, greedy activity selection.

8. **Meeting Rooms II** (LeetCode 253) -- Find the minimum number of conference rooms.
   Sweep line or min-heap approach.

9. **Interval List Intersections** (LeetCode 986) -- Two sorted interval lists, find
   all intersections. Two-pointer technique.

10. **My Calendar I** (LeetCode 729) -- Implement a calendar where you can add events
    without double-booking. Requires checking overlap on each insertion.

### Hard (5)

11. **My Calendar III** (LeetCode 732) -- Return the maximum k-booking (max number of
    simultaneous events) after each new event is added. Sweep line with a sorted map.

12. **Employee Free Time** (LeetCode 759) -- Given schedules of multiple employees
    (each a list of intervals), find the common free time across all employees.

13. **Remove Covered Intervals** (LeetCode 1288) -- Remove intervals that are completely
    covered by another. Sort by start ascending, then by end descending.

14. **Data Stream as Disjoint Intervals** (LeetCode 352) -- Implement a structure that
    tracks numbers from a data stream and returns them as disjoint intervals. Requires
    efficient merging on insertion.

15. **Amount of New Area Painted Each Day** (LeetCode 2158) -- Given daily paint ranges
    on a number line, find how much new area is painted each day. Sweep line with
    interval tracking.

---

## Exercises

Work through these in order. Each builds on the patterns above.

1. **Merge Intervals.** Implement from scratch. Then test it with:
   - `[[1,4],[2,3]]` (containment -- should produce `[[1,4]]`)
   - `[[1,1],[0,0],[2,2]]` (all disjoint -- should produce `[[0,0],[1,1],[2,2]]`)
   - A single interval (should return it unchanged)

2. **Insert Interval.** Insert `[0,0]` into `[[1,5]]` (no overlap, goes at the front).
   Insert `[0,10]` into `[[1,3],[5,7]]` (swallows everything). Insert `[3,6]` into
   `[[1,2],[8,10]]` (no overlap with either, goes in the middle).

3. **Non-Overlapping Intervals.** Implement the greedy approach. Then deliberately try
   sorting by start instead of end on `[[1,100],[2,3],[4,5]]` and verify it gives the
   wrong answer.

4. **Meeting Rooms II.** Implement both the sweep-line approach and the two-sorted-arrays
   approach. Verify they give the same answer on `[[0,30],[5,10],[15,20]]` and on
   `[[1,5],[2,3],[4,6],[7,8]]`.

5. **Interval Intersection.** Test with two empty lists, one empty list, and two lists
   that share no overlap at all.

---

*Interval problems are one of those categories where knowing the pattern gives you an
enormous advantage. The operations are simple -- sort, compare endpoints, track a running
maximum or count. The challenge is recognizing which variant you are facing and choosing
the right sort key. Once you have that, the code practically writes itself.*
