/// The struct representing the interval
#[derive(Clone,Copy,Eq,PartialEq)]
pub struct Interval {
    /// Start of the interval
    pub start: i32,
    /// End of the interval
    pub end: i32,
}
impl Interval {
    /// Create a new interval
    pub fn new(start: i32, end: i32) -> Self {
        Self {start, end}
    }
    /// Get the length of the interval
    pub fn len(&self) -> i32 {
        self.end - self.start
    }
    /// Check if element is in the interval
    pub fn contains(&self, item: i32) -> bool {
        self.start <= item && item < self.end
    }
    /// Return interval as a list
    pub fn as_list(&self) -> Vec<i32> {
        (self.start..self.end).collect()
    }
    /// Merge intervals
    pub fn merge(mut intervals: Vec<Interval>) -> Vec<Interval> {
        if intervals.is_empty() {
            return vec![];
        }
        intervals.sort_by_key(|i| i.start);
        let mut out: Vec<Interval> = Vec::new();
        for interval in intervals {
            if let Some(last) = out.last_mut() {
                if interval.start <= last.end {
                    last.end = last.end.max(interval.end);
                } else {
                    out.push(interval);
                }
            } else {
                out.push(interval);
            }
        }
        out
    }
    /// Print intervals
    pub fn print_intervals(intervals: &[Interval]) -> String {
        intervals.iter().map(|i| format!("{:?}",i)).collect::<Vec<_>>().join("")
    }
}
impl std::fmt::Debug for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interval ({}, {})", self.start, self.end)
    }
}
impl IntoIterator for Interval {
    type Item = i32;
    type IntoIter = std::ops::Range<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..self.end
    }
}
/// Given a collection of intervals, merge all overlapping intervals into a consolidated set.
///
/// adad
///
/// # Examples
///
/// Basic usage:
/// ```
/// use algorithmz::array::{Interval,merge_intervals};
/// let intervals = vec![
///     Interval::new(1, 3),
///     Interval::new(2, 6),
///     Interval::new(8, 10),
/// ];
/// let merged = Interval::merge(intervals);
/// println!("{}", Interval::print_intervals(&merged));
/// ```
///
/// Match example:
/// ```
/// use algorithmz::array::{Interval,merge_intervals};
/// let intervals = vec![
///     Interval::new(1, 3),
///     Interval::new(2, 6),
///     Interval::new(8, 10),
/// ];
/// let merged = Interval::merge(intervals);
/// assert_eq!(merged,vec![Interval::new(1,6),Interval::new(8,10)]);
/// ```
pub fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>> {
    if intervals.is_empty() {
        return Some(vec![]);
    }

    intervals.sort_by_key(|i| i[0]);

    let mut out = vec![intervals.remove(0)];

    for interval in intervals {
        let last = out.last_mut().unwrap();

        if last[1] >= interval[0] {
            last[1] = last[1].max(interval[1]);
        } else {
            out.push(interval);
        }
    }

    Some(out)
}
