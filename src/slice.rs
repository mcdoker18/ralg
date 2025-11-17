// https://leetcode.com/problems/maximum-product-subarray/
pub fn max_product(nums: Vec<i32>) -> i32 {
    assert!(!nums.is_empty());

    let mut current: i32 = 1;
    let mut current_after_negative: i32 = 1;
    let mut was_negative = false;
    let mut max = i32::MIN;

    for x in nums {
        if x == 0 {
            max = std::cmp::max(max, x);
            current = 1;
            current_after_negative = 1;
            was_negative = false;

            continue;
        }

        if was_negative {
            current *= x;
            current_after_negative *= x;

            max = std::cmp::max(max, std::cmp::max(current, current_after_negative));
        } else {
            if x < 0 {
                was_negative = true;
                current_after_negative = 1;
            }

            current *= x;
            max = std::cmp::max(max, current);
        }
    }

    max
}

#[cfg(test)]
mod max_product_test {
    use crate::slice::max_product;

    #[test]
    fn all() {
        struct TestCase {
            nums: Vec<i32>,
            expected: i32,
        }

        let tests = [
            TestCase {
                nums: vec![5],
                expected: 5,
            },
            TestCase {
                nums: vec![5, 1, 2],
                expected: 10,
            },
            TestCase {
                nums: vec![2, 3, -2, 4],
                expected: 6,
            },
            TestCase {
                nums: vec![-2, 0, -1],
                expected: 0,
            },
            TestCase {
                nums: vec![0, 2],
                expected: 2,
            },
            TestCase {
                nums: vec![2, -5, -2, -4, 3],
                expected: 24,
            },
        ];

        for tc in tests {
            let desc = format!("nums={:?}", &tc.nums);

            assert_eq!(tc.expected, max_product(tc.nums), "{desc}");
        }
    }
}

// https://leetcode.com/problems/find-peak-element/
pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    assert!(!nums.is_empty());

    if nums.len() == 1 {
        return 0;
    }

    let (mut i, mut j) = (0, nums.len() - 1);

    while i <= j {
        let mid_index = (j - i) / 2 + i;
        let mid_value = nums[mid_index];
        let after_mid_value = if mid_index != nums.len() - 1 {
            Some(nums[mid_index + 1])
        } else {
            None
        };
        let before_mid_value = if mid_index != 0 {
            Some(nums[mid_index - 1])
        } else {
            None
        };

        if after_mid_value.is_none_or(|v| mid_value > v)
            && before_mid_value.is_none_or(|v| mid_value > v)
        {
            return mid_index as i32;
        }

        if after_mid_value.is_some_and(|v| v > mid_value) {
            i = mid_index + 1;
            continue;
        }

        if before_mid_value.is_some_and(|v| v > mid_value) {
            j = mid_index;
            continue;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod find_peak_element_test {
    use super::*;

    #[test]
    fn all() {
        struct TestCase {
            nums: Vec<i32>,
            expected: i32,
        }

        let tests = [
            TestCase {
                nums: vec![1],
                expected: 0,
            },
            TestCase {
                nums: vec![1, 2],
                expected: 1,
            },
            TestCase {
                nums: vec![2, 1],
                expected: 0,
            },
            TestCase {
                nums: vec![1, 2, 3, 1],
                expected: 2,
            },
            TestCase {
                nums: vec![1, 2, 1, 3, 5, 6, 4],
                expected: 5,
            },
        ];

        for tc in tests {
            let desc = format!("vec={:?}", tc.nums);

            assert_eq!(tc.expected, find_peak_element(tc.nums), "{desc}");
        }
    }
}

// https://leetcode.com/problems/zigzag-conversion/
pub fn convert_zigzag(s: String, num_rows: i32) -> String {
    assert!(num_rows >= 1);

    let num_rows = num_rows as usize;

    if s.len() <= num_rows || num_rows == 1 {
        return s;
    }

    let s = s.as_bytes();

    let mut res = String::with_capacity(s.len());

    for row in 0..num_rows {
        let mut i = row;

        enum Direction {
            Up,
            Down,
        }

        let mut direction = Direction::Down;

        while i < s.len() {
            res.push(s[i] as char);

            let old_i = i;

            // oh, crap
            while old_i == i {
                match direction {
                    Direction::Up => {
                        i += 2 * row;
                        direction = Direction::Down;
                    }
                    Direction::Down => {
                        i += 2 * (num_rows - row - 1);
                        direction = Direction::Up;
                    }
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod convert_zigzag_test {
    use crate::slice::convert_zigzag;

    #[test]
    fn all() {
        let tests = [
            ("a", 3, "a"),
            ("ab", 3, "ab"),
            ("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR"),
        ];

        for tc in tests {
            let desc = format!("rows={}, str={}", tc.1, tc.0);

            assert_eq!(tc.2, convert_zigzag(tc.0.to_owned(), tc.1), "{desc}");
        }
    }
}

// https://leetcode.com/problems/minimum-size-subarray-sum/
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    struct Interval<'a> {
        i: usize,
        j: usize,
        curr_sum: i32,
        nums: &'a Vec<i32>,
    }

    impl<'a> Interval<'a> {
        fn new(nums: &'a Vec<i32>) -> Self {
            assert!(!nums.is_empty());

            Interval {
                i: 0,
                j: 0,
                curr_sum: nums[0],
                nums,
            }
        }

        fn sum(&self) -> i32 {
            self.curr_sum
        }

        fn is_empty(&self) -> bool {
            self.i >= self.nums.len() || self.j >= self.nums.len()
        }

        fn len(&self) -> i32 {
            (self.j - self.i + 1) as i32
        }

        fn increase(&mut self) {
            assert!(!self.is_empty());

            if self.j == self.nums.len() - 1 {
                self.curr_sum -= self.nums[self.i];
                self.i += 1;
            } else {
                self.j += 1;
                self.curr_sum += self.nums[self.j];
            }
        }

        fn decrease(&mut self) {
            assert!(!self.is_empty());

            if self.i == self.j && self.j != self.nums.len() - 1 {
                self.j += 1;
                self.curr_sum += self.nums[self.j];
            } else {
                self.curr_sum -= self.nums[self.i];
                self.i += 1;
            }
        }
    }

    let mut interval = Interval::new(&nums);

    let mut min_len: Option<i32> = None;

    while !interval.is_empty() {
        let sum = interval.sum();

        if sum >= target {
            min_len = Some(std::cmp::min(
                min_len.unwrap_or(interval.len()),
                interval.len(),
            ));

            interval.decrease();
        } else {
            interval.increase();
        }
    }

    min_len.unwrap_or(0)
}

#[cfg(test)]
mod min_sub_array_len_test {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            (7, vec![2, 3, 1, 2, 4, 3], 2),
            (7, vec![1], 0),
            (11, vec![1, 1, 1, 1, 1, 1, 1, 1], 0),
            (4, vec![1, 4, 4], 1),
            (11, vec![1, 2, 3, 4, 5], 3),
        ];

        for tc in tests {
            let desc = format!("target={}, nums={:?}", tc.0, &tc.1);

            assert_eq!(tc.2, min_sub_array_len(tc.0, tc.1), "{desc}");
        }
    }
}

#[derive(Copy, Clone)]
struct StockSpannerElem {
    price: i32,
    total_days: i32,
}

pub struct StockSpanner {
    stack: Vec<StockSpannerElem>,
}

// https://leetcode.com/problems/online-stock-span/
impl StockSpanner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        StockSpanner { stack: vec![] }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        let mut last = StockSpannerElem {
            price,
            total_days: 1,
        };

        let search_res_index = self
            .stack
            .binary_search_by_key(&std::cmp::Reverse(price), |elem| {
                std::cmp::Reverse(elem.price)
            })
            .unwrap_or_else(|v| v);

        if search_res_index == self.stack.len() {
            self.stack.push(last);

            return last.total_days;
        }

        for _ in (search_res_index..self.stack.len()).rev() {
            let curr = self.stack.pop().unwrap();

            last.total_days += curr.total_days;
        }

        self.stack.push(last);

        last.total_days
    }
}

#[cfg(test)]
mod stock_spanner_test {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            vec![(1, 1)],
            vec![(1, 1), (2, 2)],
            vec![(1, 1), (1, 2)],
            vec![(2, 1), (1, 1)],
            vec![(2, 1), (2, 2), (2, 3)],
            vec![
                (100, 1),
                (80, 1),
                (60, 1),
                (70, 2),
                (60, 1),
                (75, 4),
                (85, 6),
            ],
        ];

        for tc in tests {
            let desc = format!("{:?}", &tc);

            let mut span = StockSpanner::new();

            for (i, (price, expected)) in tc.iter().enumerate() {
                assert_eq!(*expected, span.next(*price), "i={i} {desc}");
            }
        }
    }
}

// https://leetcode.com/problems/subarray-product-less-than-k/description/
pub fn num_subarray_product_less_than_k(nums: Vec<i32>, max_product: i32) -> i32 {
    let mut res = 0;
    let (mut i, mut j): (usize, usize) = (0, 0);

    assert!(!nums.is_empty());
    let mut curr_product = nums[0];

    let mut last_product_index: Option<usize> = None;

    let calc = |i: usize, j: usize, last_product_index: Option<usize>| -> i32 {
        (1..=(j - i + 1))
            .skip({
                match last_product_index {
                    None => 0,
                    Some(v) => {
                        if v < i {
                            0
                        } else {
                            v + 1 - i
                        }
                    }
                }
            })
            .sum::<usize>() as i32
    };

    while i < nums.len() {
        if curr_product < max_product {
            res += calc(i, j, last_product_index);
            last_product_index = Some(j);

            if j == nums.len() - 1 {
                break;
            }

            j += 1;
            curr_product *= nums[j];
            continue;
        }

        i += 1;
        curr_product /= nums[i - 1];

        if j < i && i < nums.len() {
            j = i;
            curr_product = nums[i];
        }
    }

    res
}

#[cfg(test)]
mod num_subarray_product_less_than_ktest {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            (vec![1], 100, 1),
            (vec![1], 0, 0),
            (vec![1, 2, 3], 0, 0),
            (vec![10, 5, 2, 6], 100, 8),
            (vec![10, 5, 2, 6], 100, 8),
        ];

        for tc in tests {
            let desc = format!("elems={:?} k={}", &tc.0, tc.1);

            assert_eq!(tc.2, num_subarray_product_less_than_k(tc.0, tc.1), "{desc}");
        }
    }
}

// https://leetcode.com/problems/last-stone-weight/
pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    assert!(!stones.is_empty());

    let mut heap = std::collections::BinaryHeap::with_capacity(stones.len());

    for weight in stones {
        heap.push(weight);
    }

    loop {
        let Some(max_elem) = heap.pop() else {
            break 0;
        };

        let Some(prev_max_elem) = heap.pop() else {
            break max_elem;
        };

        if max_elem == prev_max_elem {
            continue;
        }

        heap.push(max_elem - prev_max_elem);
    }
}

#[cfg(test)]
mod last_stone_weight_test {
    use super::*;

    #[test]
    fn all() {
        let tests = [(vec![2, 7, 4, 1, 8, 1], 1), (vec![1], 1)];

        for tc in tests {
            let desc = format!("{:?}", tc.0);

            assert_eq!(tc.1, last_stone_weight(tc.0), "{desc}");
        }
    }
}

// https://leetcode.com/problems/contains-duplicate-ii/
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;

    let mut cache = std::collections::HashSet::with_capacity(k);

    for (i, num) in nums.iter().enumerate() {
        if cache.contains(num) {
            return true;
        }

        cache.insert(*num);

        if i >= k {
            cache.remove(&nums[i - k]);
        }
    }

    false
}

#[cfg(test)]
mod contains_nearby_duplicate_test {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            (vec![1, 2, 3, 1], 3, true),
            (vec![1, 0, 1, 1], 1, true),
            (vec![1, 2, 3, 1, 2, 3], 2, false),
        ];

        for tc in tests {
            let desc = format!("nums={:?} k={}", tc.0, tc.1);

            assert_eq!(tc.2, contains_nearby_duplicate(tc.0, tc.1), "{desc}");
        }
    }
}

// https://leetcode.com/problems/car-fleet/description/
pub fn car_fleet(target: i32, positions: Vec<i32>, speeds: Vec<i32>) -> i32 {
    assert!(positions.len() == speeds.len());

    #[derive(Debug)]
    struct Position {
        pos: i32,
        steps: f64,
    }

    impl std::cmp::Ord for Position {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.pos.cmp(&other.pos)
        }
    }

    impl std::cmp::PartialOrd for Position {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl std::cmp::PartialEq for Position {
        fn eq(&self, other: &Self) -> bool {
            self.pos == other.pos
        }
    }

    impl std::cmp::Eq for Position {}

    #[derive(Debug)]
    struct Step {
        steps: f64,
    }

    impl std::cmp::Ord for Step {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.steps.partial_cmp(&other.steps).unwrap()
        }
    }

    impl std::cmp::PartialOrd for Step {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl std::cmp::PartialEq for Step {
        fn eq(&self, other: &Self) -> bool {
            self.steps == other.steps
        }
    }

    impl std::cmp::Eq for Step {}

    let calc_steps = |pos, speed| ((target - pos) as f64) / speed as f64;

    let mut steps: std::collections::BinaryHeap<Step> = positions
        .iter()
        .cloned()
        .zip(speeds.iter().copied())
        .map(|(pos, speed)| Step {
            steps: calc_steps(pos, speed),
        })
        .collect();

    let mut positions: std::collections::BinaryHeap<Position> = positions
        .iter()
        .cloned()
        .zip(speeds.iter().copied())
        .map(|(pos, speed)| Position {
            pos,
            steps: calc_steps(pos, speed),
        })
        .collect();

    let mut max_steps: f64 = -1.0;
    let mut res = 0;

    while let Some(pos) = positions.pop() {
        // println!("{:?}", pos);
        if pos.steps <= max_steps {
            continue;
        }

        max_steps = pos.steps;

        while let Some(step) = steps.peek() {
            // println!("{:?}", step);

            if step.steps < pos.steps {
                break;
            }

            steps.pop().unwrap();
        }

        res += 1;
    }

    res
}

#[cfg(test)]
mod car_fleet_test {
    use std::vec;

    use super::*;

    #[test]
    fn all() {
        let tests = [
            (12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3], 3),
            (10, vec![3], vec![3], 1),
            (100, vec![0, 2, 4], vec![4, 2, 1], 1),
            (10, vec![6, 8], vec![3, 2], 2),
            (10, vec![8, 3, 7, 4, 6, 5], vec![4, 4, 4, 4, 4, 4], 6),
        ];

        for tc in tests {
            let desc = format!("target={}, position={:?} speed={:?}", tc.0, tc.1, tc.2);

            assert_eq!(tc.3, car_fleet(tc.0, tc.1, tc.2), "{desc}");
        }
    }
}

// https://leetcode.com/problems/summary-ranges/
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    struct Interval {
        begin: i32,
        end: i32,
    }

    impl std::fmt::Display for Interval {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.begin == self.end {
                write!(f, "{}", self.begin)
            } else {
                write!(f, "{}->{}", self.begin, self.end)
            }
        }
    }

    impl Interval {
        fn new(begin: i32, end: i32) -> Self {
            Interval { begin, end }
        }
    }

    if nums.is_empty() {
        return Vec::new();
    }

    let Some(first_value) = nums.first().copied() else {
        return Vec::new();
    };
    let mut curr_range = Interval::new(first_value, first_value);

    let mut res: Vec<String> = Vec::new();

    for num in nums[1..].iter().copied() {
        if num == curr_range.end + 1 {
            curr_range = Interval::new(curr_range.begin, num);
        } else {
            res.push(curr_range.to_string());

            curr_range = Interval::new(num, num);
        }
    }

    res.push(curr_range.to_string());

    res
}

#[cfg(test)]
mod summary_range_test {
    use super::*;

    #[test]
    fn all() {
        let tests = [
            (vec![], vec![]),
            (vec![1], vec!["1"]),
            (vec![1, 3], vec!["1", "3"]),
            (vec![1, 2], vec!["1->2"]),
            (vec![1, 2, 3], vec!["1->3"]),
            (vec![1, 3, 4], vec!["1", "3->4"]),
            (vec![0, 2, 3, 4, 6, 8, 9], vec!["0", "2->4", "6", "8->9"]),
            (vec![0, 1, 2, 4, 5, 7], vec!["0->2", "4->5", "7"]),
        ];

        for tc in tests {
            let desc = format!("{:?}", tc.0);
            assert_eq!(tc.1, summary_ranges(tc.0), "{desc}");
        }
    }
}
