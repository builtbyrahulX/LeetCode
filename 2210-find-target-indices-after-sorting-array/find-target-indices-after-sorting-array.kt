class Solution {
    fun targetIndices(nums: IntArray, target: Int): List<Int> {
        nums.sort()
        val result = ArrayList<Int>()
        for (i in nums.indices) {
            if (nums[i] == target) {
                result.add(i)
            }
        }
        return result
    }
}