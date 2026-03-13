/// Find all unique n-tuples in nums that sum to target.
///
/// Takes `n`, `nums` and `target` and returns a `Vec<Vec<i32>>`
///
/// # Examples
///
/// Basic usage:
/// ```
/// let result = algorithmz::array::n_sum(3, vec![-1, 0, 1, 2, -1, -4], 0);
/// assert_eq!(result,vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
/// ```
///
/// Match example:
/// ```
/// let nums = vec![-1, 0, 1, 2, -1, -4];
/// let result = algorithmz::array::n_sum(3, nums, 0);
/// match result.as_slice() {
///     [] => {
///         println!("No solutions found");
///     }
///     [single] => {
///         println!("One solution: {:?}", single);
///     }
///      many => {
///        println!("Multiple solutions:");
///         for r in many {
///             println!("{:?}", r);
///         }
///     }
/// }
/// ```
pub fn n_sum(n: usize, mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    nums.sort();
    n_sum_inner(n, &nums, target)
}
/// The n_sum_inner helper function
fn n_sum_inner(n: usize, nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut results: Vec<Vec<i32>> = Vec::new();

    if nums.len() < n || n < 2 {
        return results;
    }

    if n == 2 {
        return two_sum(nums, target);
    }

    let mut prev: Option<i32> = None;

    for i in 0..nums.len() {
        let num = nums[i];

        if prev.is_some() && prev.unwrap() == num {
            continue;
        }

        prev = Some(num);

        let sub_results = n_sum_inner(n - 1, &nums[i + 1..], target - num);

        for mut r in sub_results {
            r.push(num);
            r.sort();
            results.push(r);
        }
    }

    union(results)
}
/// The two_sum helper function
fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut left = 0;
    let mut right = nums.len().saturating_sub(1);

    let mut results = Vec::new();

    while left < right {
        let sum = nums[left] + nums[right];

        match sum.cmp(&target) {
            std::cmp::Ordering::Less => left += 1,
            std::cmp::Ordering::Greater => right -= 1,
            std::cmp::Ordering::Equal => {
                results.push(vec![nums[left], nums[right]]);
                left += 1;
                right -= 1;

                while left < nums.len() && nums[left] == nums[left - 1] {
                    left += 1;
                }

                while right > 0 && nums[right] == nums[right + 1] {
                    right -= 1;
                }
            }
        }
    }

    results
}
/// The union helper function
fn union(mut results: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if results.is_empty() {
        return results;
    }

    results.sort();

    let mut unique = vec![results[0].clone()];

    for r in results.into_iter().skip(1) {
        if r != *unique.last().unwrap() {
            unique.push(r);
        }
    }

    unique
}
