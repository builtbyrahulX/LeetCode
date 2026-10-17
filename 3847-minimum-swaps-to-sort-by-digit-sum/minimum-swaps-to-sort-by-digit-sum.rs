impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        fn digit_sum(mut x: i32) -> i32 {
            let mut sum = 0;
            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            sum
        }

        // Pair each number with its digit sum and original index
        let mut indexed: Vec<(i32, i32, usize)> = nums
            .iter()
            .enumerate()
            .map(|(i, &val)| (digit_sum(val), val, i))
            .collect();

        // Sort by digit sum first, then by value
        indexed.sort_unstable_by(|a, b| {
            if a.0 != b.0 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });

        // Map each value to its target sorted index
        let mut target_indices = vec![0; n];
        for (sorted_idx, &(_, _, original_idx)) in indexed.iter().enumerate() {
            target_indices[original_idx] = sorted_idx;
        }

        // Count cycles to find minimum swaps (swaps = n - number of cycles)
        let mut visited = vec![false; n];
        let mut cycles = 0;

        for i in 0..n {
            if !visited[i] {
                cycles += 1;
                let mut curr = i;
                while !visited[curr] {
                    visited[curr] = true;
                    curr = target_indices[curr];
                }
            }
        }

        (n - cycles) as i32
    }
}