// https://leetcode.com/problems/valid-sudoku/submissions/1753261966/?envType=daily-question&envId=2025-08-30

class Solution {
  bool isValidSudoku(List<List<String>> board) {
    final cols = List.generate(9, (_) => List<int>.filled(10, 0));
    var areas = List.generate(3, (_) => List<int>.filled(10, 0));

    for (int i = 0; i < 9; i++) {
      final row = List<int>.filled(10, 0);
      if (i % 3 == 0) areas = List.generate(3, (_) => List<int>.filled(10, 0));

      for (int k = 0; k < 9; k++) {
        if (board[i][k] == ".") continue;

        final v = int.parse(board[i][k]);
        if (row[v]++ == 1) return false;
        if (cols[k][v]++ == 1) return false;
        if (areas[k ~/ 3][v]++ == 1) return false;
      }
    }

    return true;
  }
}
