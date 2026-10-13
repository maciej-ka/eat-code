// https://leetcode.com/problems/valid-sudoku/submissions/1753197928/?envType=daily-question&envId=2025-08-30

class Solution {
  bool isValidSudoku(List<List<String>> board) {
    var cols = List<Set<String>>.generate(10, (_) => <String>{});
    var areas = List<Set<String>>.generate(3, (_) => <String>{});

    for (var i = 0; i < 9; i++) {
      var row = <String>{};
      if (i % 3 == 0) areas = List<Set<String>>.generate(3, (_) => <String>{});

      for (var k = 0; k < 9; k++) {
        var cell = board[i][k];
        if (cell == ".") continue;

        if (row.contains(cell)) return false;
        if (cols[k].contains(cell)) return false;
        if (areas[k ~/ 3].contains(cell)) return false;

        row.add(cell);
        cols[k].add(cell);
        areas[k ~/ 3].add(cell);
      }
    }

    return true;
  }
}
