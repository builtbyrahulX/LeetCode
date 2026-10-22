class Solution {
    func twoSum(_ nums: [Int], _ target: Int) -> [Int] {
        var seen = [Int: Int]()
        
        for (i, num) in nums.enumerated() {
            let complement = target - num
            if let complementIndex = seen[complement] {
                return [complementIndex, i]
            }
            seen[num] = i
        }
        
        return []
    }
}