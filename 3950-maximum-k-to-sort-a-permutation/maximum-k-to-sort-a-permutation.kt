class Solution {
    fun sortPermutation(nums: IntArray): Int {
        val n = nums.size
        var isSorted = true
        for (i in 0 until n - 1) {
            if (nums[i] > nums[i + 1]) {
                isSorted = false
                break
            }
        }
        if (isSorted) return 0

        var ans: Int? = null

        for (i in 0 until n) {
            if (nums[i] != i) {
                val cycleAnd = nums[i] and i
                ans = if (ans == null) cycleAnd else (ans and cycleAnd)
            }
        }

        return ans ?: 0
    }

    fun maxK(nums: IntArray): Int {
        return sortPermutation(nums)
    }
}