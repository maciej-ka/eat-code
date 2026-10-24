// https://leetcode.com/problems/count-days-spent-together/submissions/2180458759/?envType=daily-question&envId=2026-10-24
import java.util.*;

class Solution {
    private int[] months = new int[]{31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30};

    public int countDaysTogether(String arriveAlice, String leaveAlice, String arriveBob, String leaveBob) {
        int arrive = Math.max(dayNumber(arriveAlice), dayNumber(arriveBob));
        int leave = Math.min(dayNumber(leaveAlice), dayNumber(leaveBob));
        return Math.max(0, leave - arrive + 1);
    }

    private int dayNumber(String date) {
        int result = 0;
        int month = Integer.valueOf(date.substring(0, 2));
        for (int i = 0; i < month - 1; i++) result += months[i];
        return result + Integer.valueOf(date.substring(3));
    }

}
