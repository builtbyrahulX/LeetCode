impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();

        // Precompute combinations using Pascal's triangle
        let mut comb = vec![vec![0_i64; n + 1]; n + 1];
        for i in 0..=n {
            comb[i][0] = 1;
            for j in 1..=i {
                comb[i][j] = (comb[i - 1][j - 1] + comb[i - 1][j]) % MOD;
            }
        }

        // Helper function to count valid ways recursively
        fn count_ways(nums: &[i32], comb: &Vec<Vec<i64>>, mod_val: i64) -> i64 {
            if nums.len() <= 2 {
                return 1;
            }

            let root = nums[0];
            let mut left = Vec::new();
            let mut right = Vec::new();

            for &x in &nums[1..] {
                if x < root {
                    left.push(x);
                } else {
                    right.push(x);
                }
            }

            let l_ways = count_ways(&left, comb, mod_val);
            let r_ways = count_ways(&right, comb, mod_val);

            let l_len = left.len();
            let r_len = right.len();

            // Combinations of interleaving left and right subtrees
            let ways = (comb[l_len + r_len][l_len] * l_ways) % mod_val;
            (ways * r_ways) % mod_val
        }

        // Subtract 1 because we want reorderings excluding the original array
        ((count_ways(&nums, &comb, MOD) - 1 + MOD) % MOD) as i32
    }
}