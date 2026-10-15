use std::collections::HashSet;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut total_sum = 0;

        for i in 0..n {
            let mut set = HashSet::new();
            for j in i..n {
                set.insert(nums[j]);
                let count = set.len() as i32;
                total_sum += count * count;
            }
        }

        total_sum
    }
}