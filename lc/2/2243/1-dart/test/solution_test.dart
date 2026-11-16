import 'package:solution/solution.dart';
import 'package:test/test.dart';

void main() {

  test('test 1', () {
    var actual = Solution().digitSum("11111222223", 3);
    var expected = "135";
    expect(actual, expected);
  });

  test('test 2', () {
    var actual = Solution().digitSum("00000000", 3);
    var expected = "000";
    expect(actual, expected);
  });

  test('test 3', () {
    var actual = Solution().digitSum("1234", 2);
    var expected = "37";
    expect(actual, expected);
  });

}
