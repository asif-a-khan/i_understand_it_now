use rand::Rng;
use std::collections::BTreeMap;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part5_paradigms::intervals as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(MergeIntervals),
        Box::new(MeetingRooms),
        Box::new(InsertInterval),
        Box::new(SummaryRanges),
        Box::new(CoveredLength),
        Box::new(MeetingRoomsII),
        Box::new(NonOverlapping),
        Box::new(MinArrows),
        Box::new(IntervalIntersection),
        Box::new(MyCalendar),
        Box::new(EmployeeFreeTime),
        Box::new(Skyline),
        Box::new(DataStreamDisjoint),
        Box::new(MaxEvents),
        Box::new(MinIntervalQuery),
    ]
}

/// Helper: generate a random set of intervals.
fn gen_intervals(rng: &mut impl Rng, count: usize, max_val: i32) -> Vec<(i32, i32)> {
    (0..count)
        .map(|_| {
            let a = rng.random_range(0..=max_val);
            let b = rng.random_range(a..=max_val + 5);
            (a, b)
        })
        .collect()
}

// ── Easy 1: Merge Intervals ─────────────────────────────────────────

struct MergeIntervals;
struct MergeIntervalsTest {
    intervals: Vec<(i32, i32)>,
}

impl Problem for MergeIntervals {
    fn id(&self) -> &str {
        "intervals_merge"
    }
    fn name(&self) -> &str {
        "Merge Intervals"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array of intervals where intervals[i] = (start, end), merge all \
         overlapping intervals and return the non-overlapping intervals that cover \
         all the intervals in the input.\n\n\
         Constraints:\n\
         - 1 <= intervals.len() <= 10^4\n\
         - 0 <= start <= end <= 10^4"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=15);
                let intervals = gen_intervals(&mut rng, n, 30);
                TestCase {
                    data: Box::new(MergeIntervalsTest { intervals }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MergeIntervalsTest>().unwrap();
        let expected = ref_merge_intervals(&t.intervals);
        let actual = solutions::merge_intervals(&t.intervals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("intervals={:?}", t.intervals),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_merge_intervals(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut sorted: Vec<(i32, i32)> = intervals.to_vec();
    sorted.sort();
    let mut merged: Vec<(i32, i32)> = Vec::new();
    for iv in sorted {
        if let Some(last) = merged.last_mut() {
            if iv.0 <= last.1 {
                last.1 = last.1.max(iv.1);
                continue;
            }
        }
        merged.push(iv);
    }
    merged
}

// ── Easy 2: Meeting Rooms ───────────────────────────────────────────

struct MeetingRooms;
struct MeetingRoomsTest {
    intervals: Vec<(i32, i32)>,
}

impl Problem for MeetingRooms {
    fn id(&self) -> &str {
        "intervals_meeting_rooms"
    }
    fn name(&self) -> &str {
        "Meeting Rooms"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given an array of meeting time intervals (start, end), determine if a person \
         can attend all meetings (no two meetings overlap).\n\n\
         Constraints:\n\
         - 0 <= intervals.len() <= 10^4\n\
         - Meetings are half-open: [start, end). Two meetings [1,5) and [5,8) do NOT overlap."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=10);
                let intervals = gen_intervals(&mut rng, n, 30);
                TestCase {
                    data: Box::new(MeetingRoomsTest { intervals }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MeetingRoomsTest>().unwrap();
        let expected = ref_meeting_rooms(&t.intervals);
        let actual = solutions::meeting_rooms(&t.intervals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("intervals={:?}", t.intervals),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_meeting_rooms(intervals: &[(i32, i32)]) -> bool {
    let mut sorted: Vec<(i32, i32)> = intervals.to_vec();
    sorted.sort();
    for i in 1..sorted.len() {
        if sorted[i].0 < sorted[i - 1].1 {
            return false;
        }
    }
    true
}

// ── Easy 3: Insert Interval ─────────────────────────────────────────

struct InsertInterval;
struct InsertIntervalTest {
    intervals: Vec<(i32, i32)>,
    new_interval: (i32, i32),
}

impl Problem for InsertInterval {
    fn id(&self) -> &str {
        "intervals_insert"
    }
    fn name(&self) -> &str {
        "Insert Interval"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a list of non-overlapping intervals sorted by start time, insert a new \
         interval into the list. Merge if necessary. Return the resulting list of \
         non-overlapping intervals.\n\n\
         Constraints:\n\
         - 0 <= intervals.len() <= 10^4\n\
         - intervals are sorted and non-overlapping."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=8);
                // Build non-overlapping sorted intervals
                let mut intervals = Vec::new();
                let mut cur = rng.random_range(0..=5);
                for _ in 0..n {
                    let start = cur;
                    let end = start + rng.random_range(1..=5);
                    intervals.push((start, end));
                    cur = end + rng.random_range(1..=5);
                }
                let ns = rng.random_range(0..=cur + 5);
                let ne = ns + rng.random_range(1..=8);
                let new_interval = (ns, ne);
                TestCase {
                    data: Box::new(InsertIntervalTest {
                        intervals,
                        new_interval,
                    }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<InsertIntervalTest>().unwrap();
        let expected = ref_insert_interval(&t.intervals, t.new_interval);
        let actual = solutions::insert_interval(&t.intervals, t.new_interval);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("intervals={:?}, new={:?}", t.intervals, t.new_interval),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_insert_interval(intervals: &[(i32, i32)], new: (i32, i32)) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::new();
    let mut new = new;
    let mut i = 0;
    // Add intervals that come before new
    while i < intervals.len() && intervals[i].1 < new.0 {
        result.push(intervals[i]);
        i += 1;
    }
    // Merge overlapping
    while i < intervals.len() && intervals[i].0 <= new.1 {
        new.0 = new.0.min(intervals[i].0);
        new.1 = new.1.max(intervals[i].1);
        i += 1;
    }
    result.push(new);
    // Add remaining
    while i < intervals.len() {
        result.push(intervals[i]);
        i += 1;
    }
    result
}

// ── Easy 4: Summary Ranges ──────────────────────────────────────────

struct SummaryRanges;
struct SummaryRangesTest {
    nums: Vec<i32>,
}

impl Problem for SummaryRanges {
    fn id(&self) -> &str {
        "intervals_summary_ranges"
    }
    fn name(&self) -> &str {
        "Summary Ranges"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a sorted unique integer array, return the smallest sorted list of ranges \
         that cover all numbers. Format: \"a->b\" if a != b, or \"a\" if a == b.\n\n\
         Constraints:\n\
         - 0 <= nums.len() <= 20\n\
         - nums is sorted in ascending order with no duplicates."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(0..=15);
                let mut set = std::collections::BTreeSet::new();
                while set.len() < n {
                    set.insert(rng.random_range(-20..=20));
                }
                let nums: Vec<i32> = set.into_iter().collect();
                TestCase {
                    data: Box::new(SummaryRangesTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SummaryRangesTest>().unwrap();
        let expected = ref_summary_ranges(&t.nums);
        let actual = solutions::summary_ranges(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_summary_ranges(nums: &[i32]) -> Vec<String> {
    let mut result = Vec::new();
    if nums.is_empty() {
        return result;
    }
    let mut start = nums[0];
    for i in 1..=nums.len() {
        if i == nums.len() || nums[i] != nums[i - 1] + 1 {
            let end = nums[i - 1];
            if start == end {
                result.push(format!("{start}"));
            } else {
                result.push(format!("{start}->{end}"));
            }
            if i < nums.len() {
                start = nums[i];
            }
        }
    }
    result
}

// ── Easy 5: Covered Length ──────────────────────────────────────────

struct CoveredLength;
struct CoveredLengthTest {
    intervals: Vec<(i32, i32)>,
}

impl Problem for CoveredLength {
    fn id(&self) -> &str {
        "intervals_covered_length"
    }
    fn name(&self) -> &str {
        "Total Covered Length"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
    fn description(&self) -> &str {
        "Given a list of intervals, return the total length covered by the union \
         of all intervals. Overlapping regions are counted once.\n\n\
         Constraints:\n\
         - 1 <= intervals.len() <= 10^4\n\
         - 0 <= start <= end <= 10^5"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=15);
                let intervals = gen_intervals(&mut rng, n, 30);
                TestCase {
                    data: Box::new(CoveredLengthTest { intervals }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CoveredLengthTest>().unwrap();
        let expected = ref_covered_length(&t.intervals);
        let actual = solutions::covered_length(&t.intervals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("intervals={:?}", t.intervals),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_covered_length(intervals: &[(i32, i32)]) -> i32 {
    let merged = ref_merge_intervals(intervals);
    merged.iter().map(|(s, e)| e - s).sum()
}

// ── Medium 1: Meeting Rooms II ──────────────────────────────────────

struct MeetingRoomsII;
struct MeetingRoomsIITest {
    intervals: Vec<(i32, i32)>,
}

impl Problem for MeetingRoomsII {
    fn id(&self) -> &str {
        "intervals_meeting_rooms_ii"
    }
    fn name(&self) -> &str {
        "Meeting Rooms II"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given an array of meeting time intervals, find the minimum number of \
         conference rooms required.\n\n\
         Constraints:\n\
         - 1 <= intervals.len() <= 10^4\n\
         - Meetings are half-open: [start, end)."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=15);
                let intervals = gen_intervals(&mut rng, n, 30);
                TestCase {
                    data: Box::new(MeetingRoomsIITest { intervals }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MeetingRoomsIITest>().unwrap();
        let expected = ref_meeting_rooms_ii(&t.intervals);
        let actual = solutions::meeting_rooms_ii(&t.intervals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("intervals={:?}", t.intervals),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_meeting_rooms_ii(intervals: &[(i32, i32)]) -> i32 {
    let mut events: Vec<(i32, i32)> = Vec::new();
    for &(s, e) in intervals {
        events.push((s, 1));
        events.push((e, -1));
    }
    events.sort();
    let mut max_rooms = 0;
    let mut cur = 0;
    for (_, delta) in events {
        cur += delta;
        max_rooms = max_rooms.max(cur);
    }
    max_rooms
}

// ── Medium 2: Non-Overlapping Intervals ──────────────────────────────

struct NonOverlapping;
struct NonOverlappingTest {
    intervals: Vec<(i32, i32)>,
}

impl Problem for NonOverlapping {
    fn id(&self) -> &str {
        "intervals_non_overlapping"
    }
    fn name(&self) -> &str {
        "Non-Overlapping Intervals"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given a collection of intervals, find the minimum number of intervals you need \
         to remove to make the rest non-overlapping.\n\n\
         Constraints:\n\
         - 1 <= intervals.len() <= 10^5\n\
         - Intervals are [start, end)."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=15);
                let intervals = gen_intervals(&mut rng, n, 30);
                TestCase {
                    data: Box::new(NonOverlappingTest { intervals }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<NonOverlappingTest>().unwrap();
        let expected = ref_non_overlapping(&t.intervals);
        let actual = solutions::non_overlapping(&t.intervals);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("intervals={:?}", t.intervals),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_non_overlapping(intervals: &[(i32, i32)]) -> i32 {
    if intervals.is_empty() {
        return 0;
    }
    let mut sorted: Vec<(i32, i32)> = intervals.to_vec();
    sorted.sort_by_key(|iv| iv.1);
    let mut count = 0;
    let mut end = sorted[0].1;
    for iv in &sorted[1..] {
        if iv.0 < end {
            count += 1; // remove this interval
        } else {
            end = iv.1;
        }
    }
    count
}

// ── Medium 3: Minimum Number of Arrows ──────────────────────────────

struct MinArrows;
struct MinArrowsTest {
    balloons: Vec<(i32, i32)>,
}

impl Problem for MinArrows {
    fn id(&self) -> &str {
        "intervals_min_arrows"
    }
    fn name(&self) -> &str {
        "Minimum Number of Arrows to Burst Balloons"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given balloons as intervals (xstart, xend), an arrow shot at position x \
         bursts all balloons where xstart <= x <= xend. Return the minimum number \
         of arrows to burst all balloons.\n\n\
         Constraints:\n\
         - 1 <= balloons.len() <= 10^5"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=15);
                let balloons = gen_intervals(&mut rng, n, 30);
                TestCase {
                    data: Box::new(MinArrowsTest { balloons }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MinArrowsTest>().unwrap();
        let expected = ref_min_arrows(&t.balloons);
        let actual = solutions::min_arrows(&t.balloons);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("balloons={:?}", t.balloons),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_min_arrows(balloons: &[(i32, i32)]) -> i32 {
    if balloons.is_empty() {
        return 0;
    }
    let mut sorted: Vec<(i32, i32)> = balloons.to_vec();
    sorted.sort_by_key(|b| b.1);
    let mut arrows = 1;
    let mut end = sorted[0].1;
    for b in &sorted[1..] {
        if b.0 > end {
            arrows += 1;
            end = b.1;
        }
    }
    arrows
}

// ── Medium 4: Interval List Intersections ────────────────────────────

struct IntervalIntersection;
struct IntervalIntersectionTest {
    first: Vec<(i32, i32)>,
    second: Vec<(i32, i32)>,
}

impl Problem for IntervalIntersection {
    fn id(&self) -> &str {
        "intervals_interval_intersection"
    }
    fn name(&self) -> &str {
        "Interval List Intersections"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Given two lists of closed intervals (each sorted and non-overlapping), \
         return the intersection of these two interval lists.\n\n\
         Constraints:\n\
         - 0 <= first.len(), second.len() <= 1000\n\
         - Each list is sorted by start and pairwise disjoint."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let make_sorted =
                    |rng: &mut rand::rngs::ThreadRng, count: usize| -> Vec<(i32, i32)> {
                        let mut intervals = Vec::new();
                        let mut cur = rng.random_range(0..=5);
                        for _ in 0..count {
                            let start = cur;
                            let end = start + rng.random_range(1..=5);
                            intervals.push((start, end));
                            cur = end + rng.random_range(1..=5);
                        }
                        intervals
                    };
                let n1 = rng.random_range(0..=6);
                let n2 = rng.random_range(0..=6);
                let first = make_sorted(&mut rng, n1);
                let second = make_sorted(&mut rng, n2);
                TestCase {
                    data: Box::new(IntervalIntersectionTest { first, second }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test
            .data
            .downcast_ref::<IntervalIntersectionTest>()
            .unwrap();
        let expected = ref_interval_intersection(&t.first, &t.second);
        let actual = solutions::interval_intersection(&t.first, &t.second);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("first={:?}, second={:?}", t.first, t.second),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_interval_intersection(first: &[(i32, i32)], second: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < first.len() && j < second.len() {
        let lo = first[i].0.max(second[j].0);
        let hi = first[i].1.min(second[j].1);
        if lo <= hi {
            result.push((lo, hi));
        }
        if first[i].1 < second[j].1 {
            i += 1;
        } else {
            j += 1;
        }
    }
    result
}

// ── Medium 5: My Calendar ───────────────────────────────────────────

struct MyCalendar;
struct MyCalendarTest {
    bookings: Vec<(i32, i32)>,
}

impl Problem for MyCalendar {
    fn id(&self) -> &str {
        "intervals_my_calendar"
    }
    fn name(&self) -> &str {
        "My Calendar I"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
    fn description(&self) -> &str {
        "Implement a calendar that can book events. For each booking attempt (start, end), \
         return true if the event can be added without causing a double-booking, false \
         otherwise. If true, the event is added.\n\n\
         Events are half-open: [start, end).\n\n\
         Input: a list of (start, end) booking attempts in order.\n\
         Output: a list of bool results for each attempt."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=12);
                let bookings: Vec<(i32, i32)> = (0..n)
                    .map(|_| {
                        let s = rng.random_range(0..=20);
                        let e = rng.random_range(s + 1..=s + 8);
                        (s, e)
                    })
                    .collect();
                TestCase {
                    data: Box::new(MyCalendarTest { bookings }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MyCalendarTest>().unwrap();
        let expected = ref_my_calendar(&t.bookings);
        let actual = solutions::my_calendar(&t.bookings);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("bookings={:?}", t.bookings),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_my_calendar(bookings: &[(i32, i32)]) -> Vec<bool> {
    let mut booked: Vec<(i32, i32)> = Vec::new();
    let mut results = Vec::new();
    for &(s, e) in bookings {
        let conflict = booked.iter().any(|&(bs, be)| s < be && e > bs);
        if conflict {
            results.push(false);
        } else {
            booked.push((s, e));
            results.push(true);
        }
    }
    results
}

// ── Hard 1: Employee Free Time ──────────────────────────────────────

struct EmployeeFreeTime;
struct EmployeeFreeTimeTest {
    schedules: Vec<Vec<(i32, i32)>>,
}

impl Problem for EmployeeFreeTime {
    fn id(&self) -> &str {
        "intervals_employee_free_time"
    }
    fn name(&self) -> &str {
        "Employee Free Time"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given the working time intervals for each employee (sorted for each employee), \
         find the common free time intervals for all employees, sorted in ascending order.\n\n\
         Free time is a time interval where no employee is working.\n\n\
         Constraints:\n\
         - 1 <= schedules.len() <= 50\n\
         - 1 <= schedules[i].len() <= 50"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let num_employees = rng.random_range(1..=4);
                let schedules: Vec<Vec<(i32, i32)>> = (0..num_employees)
                    .map(|_| {
                        let num_intervals = rng.random_range(1..=4);
                        let mut intervals = Vec::new();
                        let mut cur = rng.random_range(0..=5);
                        for _ in 0..num_intervals {
                            let start = cur;
                            let end = start + rng.random_range(1..=5);
                            intervals.push((start, end));
                            cur = end + rng.random_range(1..=5);
                        }
                        intervals
                    })
                    .collect();
                TestCase {
                    data: Box::new(EmployeeFreeTimeTest { schedules }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<EmployeeFreeTimeTest>().unwrap();
        let expected = ref_employee_free_time(&t.schedules);
        let actual = solutions::employee_free_time(&t.schedules);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("schedules={:?}", t.schedules),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_employee_free_time(schedules: &[Vec<(i32, i32)>]) -> Vec<(i32, i32)> {
    let all: Vec<(i32, i32)> = schedules.iter().flat_map(|s| s.iter().copied()).collect();
    let merged = ref_merge_intervals(&all);
    let mut free = Vec::new();
    for i in 1..merged.len() {
        if merged[i].0 > merged[i - 1].1 {
            free.push((merged[i - 1].1, merged[i].0));
        }
    }
    free
}

// ── Hard 2: Skyline Problem ─────────────────────────────────────────

struct Skyline;
struct SkylineTest {
    buildings: Vec<(i32, i32, i32)>,
}

impl Problem for Skyline {
    fn id(&self) -> &str {
        "intervals_skyline"
    }
    fn name(&self) -> &str {
        "The Skyline Problem"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given buildings as (left, right, height) tuples, return the skyline as \
         key points (x, height).\n\n\
         A key point is the left endpoint of a horizontal line segment in the skyline. \
         The last key point always has height 0.\n\n\
         Constraints:\n\
         - 1 <= buildings.len() <= 10^4\n\
         - Buildings sorted by left coordinate."
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=8);
                let mut buildings: Vec<(i32, i32, i32)> = (0..n)
                    .map(|_| {
                        let l = rng.random_range(0..=20);
                        let r = rng.random_range(l + 1..=l + 10);
                        let h = rng.random_range(1..=20);
                        (l, r, h)
                    })
                    .collect();
                buildings.sort_by_key(|b| b.0);
                TestCase {
                    data: Box::new(SkylineTest { buildings }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SkylineTest>().unwrap();
        let expected = ref_skyline_sweep(&t.buildings);
        let actual = solutions::skyline(&t.buildings);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("buildings={:?}", t.buildings),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_skyline_sweep(buildings: &[(i32, i32, i32)]) -> Vec<(i32, i32)> {
    // Event-based sweep line using a BTreeMap as a multiset
    let mut events: Vec<(i32, i32)> = Vec::new();
    for &(l, r, h) in buildings {
        events.push((l, -h)); // building start (negative for sorting)
        events.push((r, h)); // building end
    }
    events.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut heights: BTreeMap<i32, usize> = BTreeMap::new();
    heights.insert(0, 1);
    let mut result = Vec::new();
    let mut prev_max = 0;

    for (x, h) in events {
        if h < 0 {
            *heights.entry(-h).or_insert(0) += 1;
        } else {
            let count = heights.get_mut(&h).unwrap();
            *count -= 1;
            if *count == 0 {
                heights.remove(&h);
            }
        }
        let cur_max = *heights.keys().next_back().unwrap();
        if cur_max != prev_max {
            result.push((x, cur_max));
            prev_max = cur_max;
        }
    }
    result
}

// ── Hard 3: Data Stream as Disjoint Intervals ────────────────────────

struct DataStreamDisjoint;
struct DataStreamDisjointTest {
    nums: Vec<i32>,
}

impl Problem for DataStreamDisjoint {
    fn id(&self) -> &str {
        "intervals_data_stream_disjoint"
    }
    fn name(&self) -> &str {
        "Data Stream as Disjoint Intervals"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a stream of integers (in the order they arrive), after all numbers \
         have been added, return the disjoint intervals that cover all added numbers, \
         sorted by start.\n\n\
         Constraints:\n\
         - 1 <= nums.len() <= 10^4\n\
         - 0 <= nums[i] <= 10^4"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=20);
                let nums: Vec<i32> = (0..n).map(|_| rng.random_range(0..=30)).collect();
                TestCase {
                    data: Box::new(DataStreamDisjointTest { nums }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<DataStreamDisjointTest>().unwrap();
        let expected = ref_data_stream_disjoint(&t.nums);
        let actual = solutions::data_stream_disjoint(&t.nums);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("nums={:?}", t.nums),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_data_stream_disjoint(nums: &[i32]) -> Vec<(i32, i32)> {
    let set: std::collections::BTreeSet<i32> = nums.iter().copied().collect();
    let sorted: Vec<i32> = set.into_iter().collect();
    if sorted.is_empty() {
        return vec![];
    }
    let mut result = Vec::new();
    let mut start = sorted[0];
    for i in 1..=sorted.len() {
        if i == sorted.len() || sorted[i] != sorted[i - 1] + 1 {
            result.push((start, sorted[i - 1]));
            if i < sorted.len() {
                start = sorted[i];
            }
        }
    }
    result
}

// ── Hard 4: Maximum Number of Events ─────────────────────────────────

struct MaxEvents;
struct MaxEventsTest {
    events: Vec<(i32, i32)>,
}

impl Problem for MaxEvents {
    fn id(&self) -> &str {
        "intervals_max_events"
    }
    fn name(&self) -> &str {
        "Maximum Number of Events That Can Be Attended"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given events as (startDay, endDay), you can attend an event on any single day \
         in [startDay, endDay]. Each day you can attend at most one event. Return the \
         maximum number of events you can attend.\n\n\
         Constraints:\n\
         - 1 <= events.len() <= 10^5\n\
         - 1 <= startDay <= endDay <= 10^5"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let n = rng.random_range(1..=12);
                let events: Vec<(i32, i32)> = (0..n)
                    .map(|_| {
                        let s = rng.random_range(1..=15);
                        let e = rng.random_range(s..=s + 5);
                        (s, e)
                    })
                    .collect();
                TestCase {
                    data: Box::new(MaxEventsTest { events }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxEventsTest>().unwrap();
        let expected = ref_max_events(&t.events);
        let actual = solutions::max_events(&t.events);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("events={:?}", t.events),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_events(events: &[(i32, i32)]) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut sorted: Vec<(i32, i32)> = events.to_vec();
    sorted.sort();
    let max_day = sorted.iter().map(|e| e.1).max().unwrap_or(0);
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut idx = 0;
    let mut count = 0;
    for day in 1..=max_day {
        // Add all events starting on this day
        while idx < sorted.len() && sorted[idx].0 == day {
            heap.push(Reverse(sorted[idx].1));
            idx += 1;
        }
        // Remove expired events
        while let Some(&Reverse(end)) = heap.peek() {
            if end < day {
                heap.pop();
            } else {
                break;
            }
        }
        // Attend the event with the earliest end
        if heap.pop().is_some() {
            count += 1;
        }
    }
    count
}

// ── Hard 5: Minimum Interval to Include Each Query ───────────────────

struct MinIntervalQuery;
struct MinIntervalQueryTest {
    intervals: Vec<(i32, i32)>,
    queries: Vec<i32>,
}

impl Problem for MinIntervalQuery {
    fn id(&self) -> &str {
        "intervals_min_interval_query"
    }
    fn name(&self) -> &str {
        "Minimum Interval to Include Each Query"
    }
    fn topic(&self) -> &str {
        "intervals"
    }
    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
    fn description(&self) -> &str {
        "Given a 2D array of intervals and a query array, for each query point, \
         find the size of the smallest interval that contains that point. The size \
         of interval [l, r] is r - l + 1. Return -1 if no interval contains the query.\n\n\
         Constraints:\n\
         - 1 <= intervals.len(), queries.len() <= 10^5"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10)
            .map(|_| {
                let ni = rng.random_range(1..=10);
                let intervals: Vec<(i32, i32)> = (0..ni)
                    .map(|_| {
                        let a = rng.random_range(1..=20);
                        let b = rng.random_range(a..=a + 10);
                        (a, b)
                    })
                    .collect();
                let nq = rng.random_range(1..=10);
                let queries: Vec<i32> = (0..nq).map(|_| rng.random_range(0..=30)).collect();
                TestCase {
                    data: Box::new(MinIntervalQueryTest { intervals, queries }),
                }
            })
            .collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MinIntervalQueryTest>().unwrap();
        let expected = ref_min_interval_query(&t.intervals, &t.queries);
        let actual = solutions::min_interval_query(&t.intervals, &t.queries);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("intervals={:?}, queries={:?}", t.intervals, t.queries),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_min_interval_query(intervals: &[(i32, i32)], queries: &[i32]) -> Vec<i32> {
    queries
        .iter()
        .map(|&q| {
            let mut best = -1;
            for &(l, r) in intervals {
                if l <= q && q <= r {
                    let size = r - l + 1;
                    if best == -1 || size < best {
                        best = size;
                    }
                }
            }
            best
        })
        .collect()
}
