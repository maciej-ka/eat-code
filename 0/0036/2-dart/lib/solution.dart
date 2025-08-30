// https://leetcode.com/problems/valid-sudoku/submissions/1753223608/?envType=daily-question&envId=2025-08-30

class Solution {
  bool isValidSudoku(List<List<String>> board) {
    final cols = List<Set<String>>.generate(10, (_) => <String>{});
    var areas = List<Set<String>>.generate(3, (_) => <String>{});

    for (int i = 0; i < 9; i++) {
      final row = <String>{};
      if (i % 3 == 0) areas = List<Set<String>>.generate(3, (_) => <String>{});

      for (int k = 0; k < 9; k++) {
        final cell = board[i][k];
        if (cell == ".") continue;

        if (!row.add(cell)) return false;
        if (!cols[k].add(cell)) return false;
        if (!areas[k ~/ 3].add(cell)) return false;
      }
    }

    return true;
  }
}
