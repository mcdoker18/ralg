// https://leetcode.com/problems/rectangle-area/description/
#[allow(clippy::too_many_arguments)]
pub fn compute_area(
    ax1: i32,
    ay1: i32,
    ax2: i32,
    ay2: i32,
    bx1: i32,
    by1: i32,
    bx2: i32,
    by2: i32,
) -> i32 {
    let rec1 = Rectangle::new(Point { x: ax1, y: ay1 }, Point { x: ax2, y: ay2 });
    let rec2 = Rectangle::new(Point { x: bx1, y: by1 }, Point { x: bx2, y: by2 });

    let rec1_x_interval = Interval {
        start: ax1,
        end: ax2,
    };
    let rec2_x_interval = Interval {
        start: bx1,
        end: bx2,
    };

    let rec1_y_interval = Interval {
        start: ay1,
        end: ay2,
    };
    let rec2_y_interval = Interval {
        start: by1,
        end: by2,
    };

    let x_overlap = rec1_x_interval.calc_overlap_len(&rec2_x_interval);
    let y_overlap = rec1_y_interval.calc_overlap_len(&rec2_y_interval);

    let intersecting_area = x_overlap * y_overlap;

    rec1.area() + rec2.area() - intersecting_area
}

#[cfg(test)]
mod compute_area_tests {
    use std::collections::HashMap;

    use super::*;

    struct TestCase {
        coords: [i32; 8],
        res: i32,
    }

    #[test]
    fn all() {
        let tests = HashMap::from([
            (
                "both_zero_area",
                TestCase {
                    coords: [1, 1, 1, 1, 3, 3, 3, 3],
                    res: 0,
                },
            ),
            (
                "first_zero_area",
                TestCase {
                    coords: [1, 1, 1, 1, 3, 4, 6, 5],
                    res: 3,
                },
            ),
            (
                "second_zero_area",
                TestCase {
                    coords: [1, 1, 8, 5, 3, 3, 3, 3],
                    res: 28,
                },
            ),
            (
                "both_full_area",
                TestCase {
                    coords: [1, 1, 8, 5, 33, 34, 36, 36],
                    res: 34,
                },
            ),
            (
                "one_cell_common",
                TestCase {
                    coords: [0, 2, 3, 4, 2, 0, 5, 3],
                    res: 14,
                },
            ),
            (
                "square_cell_common",
                TestCase {
                    coords: [-2, 2, 1, 4, -1, 1, 2, 5],
                    res: 14,
                },
            ),
            (
                "square_in_square",
                TestCase {
                    coords: [-2, -2, 2, 2, -1, -1, 1, 1],
                    res: 16,
                },
            ),
        ]);

        for (name, tc) in tests {
            assert_eq!(
                tc.res,
                compute_area(
                    tc.coords[0],
                    tc.coords[1],
                    tc.coords[2],
                    tc.coords[3],
                    tc.coords[4],
                    tc.coords[5],
                    tc.coords[6],
                    tc.coords[7]
                ),
                "name={name}"
            );
        }
    }
}

#[derive(Debug)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn is_empty(&self) -> bool {
        self.end == self.start
    }

    fn calc_overlap_len(&self, other: &Interval) -> i32 {
        if self.is_empty() || other.is_empty() {
            return 0;
        }

        if self.end == other.start || other.end == self.start {
            return 0;
        }

        if (self.start <= other.start && other.start <= self.end)
            || (other.start <= self.start && self.start <= other.end)
        {
            std::cmp::min(self.end, other.end) - std::cmp::max(self.start, other.start)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod interval_tests {
    use std::collections::HashMap;

    use super::*;

    struct TestCase {
        x: Interval,
        y: Interval,
        res: i32,
    }

    #[test]
    fn all() {
        let tests = HashMap::from([
            (
                "y inside x",
                TestCase {
                    x: Interval { start: 1, end: 3 },
                    y: Interval { start: 1, end: 2 },
                    res: 1,
                },
            ),
            (
                "x inside y",
                TestCase {
                    x: Interval { start: 2, end: 3 },
                    y: Interval { start: 1, end: 4 },
                    res: 1,
                },
            ),
            (
                "x to the left of y",
                TestCase {
                    x: Interval { start: 1, end: 5 },
                    y: Interval { start: 7, end: 10 },
                    res: 0,
                },
            ),
            (
                "x to the left of y. included",
                TestCase {
                    x: Interval { start: 1, end: 7 },
                    y: Interval { start: 7, end: 10 },
                    res: 0,
                },
            ),
            (
                "x to the right of y. included",
                TestCase {
                    x: Interval { start: 7, end: 10 },
                    y: Interval { start: 1, end: 7 },
                    res: 0,
                },
            ),
            (
                "x enter to y from left",
                TestCase {
                    x: Interval { start: 1, end: 5 },
                    y: Interval { start: 3, end: 7 },
                    res: 2,
                },
            ),
            (
                "y enter to x from left",
                TestCase {
                    x: Interval { start: 3, end: 7 },
                    y: Interval { start: 1, end: 5 },
                    res: 2,
                },
            ),
        ]);

        for (name, tc) in tests {
            assert_eq!(tc.res, tc.x.calc_overlap_len(&tc.y), "name={name}");
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    bottom_left: Point,
    top_right: Point,
}

impl Rectangle {
    fn new(bottom_left: Point, top_right: Point) -> Self {
        Rectangle {
            bottom_left,
            top_right,
        }
    }

    fn area(&self) -> i32 {
        (self.top_right.x - self.bottom_left.x) * (self.top_right.y - self.bottom_left.y)
    }
}
