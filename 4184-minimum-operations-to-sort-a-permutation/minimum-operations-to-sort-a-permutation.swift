class Solution {
    func minOperations(_ nums: [Int]) -> Int {
        let n = nums.count
        if n <= 1 { return 0 }

        var isIncreasing = true
        var isDecreasing = true
        var zeroIdx = 0

        for i in 0..<n {
            if nums[i] == 0 {
                zeroIdx = i
            }
            let nextVal = nums[(i + 1) % n]
            if (nums[i] + 1) % n != nextVal {
                isIncreasing = false
            }
            if (nums[i] - 1 + n) % n != nextVal {
                isDecreasing = false
            }
        }

        if !isIncreasing && !isDecreasing {
            return -1
        }

        var ans = Int.max

        if isIncreasing {
            let leftRotations = zeroIdx
            let rightRotations = (zeroIdx == 0) ? 0 : (n - zeroIdx + 2)
            ans = min(ans, leftRotations, rightRotations)
        }

        if isDecreasing {
            let rotateThenReverse = ((zeroIdx + 1) % n) + 1
            let reverseThenRotate = n - zeroIdx
            ans = min(ans, rotateThenReverse, reverseThenRotate)
        }

        return ans
    }
}