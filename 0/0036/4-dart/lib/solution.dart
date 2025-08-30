// https://leetcode.com/problems/valid-sudoku/submissions/1753260661/?envType=daily-question&envId=2025-08-30

class Solution {
  bool isValidSudoku(List<List<String>> board) {
    final cols = List.filled(90, 0);
    final areas = List.filled(90, 0);

    for (int i = 0; i < 9; i++) {
      final row = List.filled(10, 0);
      for (int k = 0; k < 9; k++) {
        if (board[i][k] == ".") continue;

        final v = int.parse(board[i][k]);
        if (row[v]++ == 1) return false;
        if (cols[10 * k + v]++ == 1) return false;
        if (areas[30 * (i ~/ 3) + 10 * (k ~/ 3) + v]++ == 1) return false;
      }
    }

    return true;
  }
}
