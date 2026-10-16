impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();
        let mut count_total = vec![0i64; 4 * n];
        let mut sq_total = vec![0i64; 4 * n];
        let mut lazy = vec![0i64; 4 * n];

        fn push(
            node: usize,
            tleft: usize,
            tright: usize,
            count_total: &mut [i64],
            sq_total: &mut [i64],
            lazy: &mut [i64],
        ) {
            if lazy[node] != 0 {
                let l = (tright - tleft + 1) as i64;
                let x = lazy[node];
                let prev_c = count_total[node];
                let prev_s = sq_total[node];

                let new_sq = (prev_s + 2 * x * prev_c + l * x * x) % MOD;
                let new_count = (prev_c + l * x) % MOD;

                sq_total[node] = new_sq;
                count_total[node] = new_count;

                if tleft != tright {
                    let left_child = 2 * node;
                    let right_child = 2 * node + 1;
                    lazy[left_child] = (lazy[left_child] + x) % MOD;
                    lazy[right_child] = (lazy[right_child] + x) % MOD;
                }
                lazy[node] = 0;
            }
        }

        fn update(
            node: usize,
            tleft: usize,
            tright: usize,
            ql: usize,
            qr: usize,
            count_total: &mut [i64],
            sq_total: &mut [i64],
            lazy: &mut [i64],
        ) {
            push(node, tleft, tright, count_total, sq_total, lazy);
            if ql <= tleft && tright <= qr {
                lazy[node] = (lazy[node] + 1) % MOD;
                push(node, tleft, tright, count_total, sq_total, lazy);
                return;
            }
            let mid = tleft + (tright - tleft) / 2;
            let left_child = 2 * node;
            let right_child = 2 * node + 1;

            if ql <= mid {
                update(left_child, tleft, mid, ql, qr, count_total, sq_total, lazy);
            }
            if qr > mid {
                update(right_child, mid + 1, tright, ql, qr, count_total, sq_total, lazy);
            }

            push(left_child, tleft, mid, count_total, sq_total, lazy);
            push(right_child, mid + 1, tright, count_total, sq_total, lazy);

            count_total[node] = (count_total[left_child] + count_total[right_child]) % MOD;
            sq_total[node] = (sq_total[left_child] + sq_total[right_child]) % MOD;
        }

        let mut ans = 0i64;
        let mut last_seen = std::collections::HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let prev_idx = *last_seen.get(&num).unwrap_or(&-1);
            let ql = (prev_idx + 1) as usize;
            let qr = i;
            
            update(1, 0, n - 1, ql, qr, &mut count_total, &mut sq_total, &mut lazy);
            last_seen.insert(num, i as i32);
            ans = (ans + sq_total[1]) % MOD;
        }

        ans as i32
    }
}