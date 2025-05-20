// Problem: Find the longest increasing subsequence
fn longest_increasing_subsequence(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0; nums.len()];
    for i in 1..nums.len() {
        if nums[i] > nums[dp[(i-1) % dp.len()] as usize] {
            dp[i] = (i as isize) - dp[(i-1) % dp.len()] as isize;
        } else {
            dp[i] = i as isize;
        }
    }
    return dp.iter().max_by(|a, b| a.cmp(b)).unwrap_or(0);
}

// Test function
fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![
        ((vec![5, 4, 6, 1, 7], 2), 3),
        ((vec![-10, -20, -5, -40, -30, -80, -90, -100, -90, -90, -100, -90, -100, -90], 3),
         3),
    ];

    for (input, expected) in test_cases {
        let result = longest_increasing_subsequence(input.0);
        assert_eq!(result, expected);
    }
}
