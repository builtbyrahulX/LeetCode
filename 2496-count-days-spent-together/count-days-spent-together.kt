import kotlin.math.max
import kotlin.math.min

class Solution {
    fun countDaysTogether(
        arriveAlice: String,
        leaveAlice: String,
        arriveBob: String,
        leaveBob: String
    ): Int {
        val daysInMonths = intArrayOf(31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31)

        fun dateToDayOfYear(date: String): Int {
            val month = date.substring(0, 2).toInt()
            val day = date.substring(3, 5).toInt()
            var totalDays = 0
            for (m in 0 until month - 1) {
                totalDays += daysInMonths[m]
            }
            return totalDays + day
        }

        val startAlice = dateToDayOfYear(arriveAlice)
        val endAlice = dateToDayOfYear(leaveAlice)
        val startBob = dateToDayOfYear(arriveBob)
        val endBob = dateToDayOfYear(leaveBob)

        val overlapStart = max(startAlice, startBob)
        val overlapEnd = min(endAlice, endBob)

        return max(0, overlapEnd - overlapStart + 1)
    }

    fun countDaysSpentTogether(
        arriveAlice: String,
        leaveAlice: String,
        arriveBob: String,
        leaveBob: String
    ): Int {
        return countDaysTogether(arriveAlice, leaveAlice, arriveBob, leaveBob)
    }
}