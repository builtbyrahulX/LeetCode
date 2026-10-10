import kotlin.math.max

class Solution {
    fun largestOverlap(img1: Array<IntArray>, img2: Array<IntArray>): Int {
        val n = img1.size
        val points1 = ArrayList<IntArray>()
        val points2 = ArrayList<IntArray>()

        for (r in 0 until n) {
            for (c in 0 until n) {
                if (img1[r][c] == 1) {
                    points1.add(intArrayOf(r, c))
                }
                if (img2[r][c] == 1) {
                    points2.add(intArrayOf(r, c))
                }
            }
        }

        val shiftCount = HashMap<Int, Int>()
        var maxOverlap = 0

        for (p1 in points1) {
            for (p2 in points2) {
                val dr = p2[0] - p1[0]
                val dc = p2[1] - p1[1]
                val key = (dr + 100) * 1000 + (dc + 100)
                val count = shiftCount.getOrDefault(key, 0) + 1
                shiftCount[key] = count
                maxOverlap = max(maxOverlap, count)
            }
        }

        return maxOverlap
    }
}