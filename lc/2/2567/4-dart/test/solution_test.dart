import 'package:solution/solution.dart';
import 'package:test/test.dart';

void main() {

  test('test 1', () {
    var actual = Solution().minimizeSum([1,4,7,8,5]);
    var expected = 3;
    expect(actual, expected);
  });

  test('test 2', () {
    var actual = Solution().minimizeSum([1,4,3]);
    var expected = 0;
    expect(actual, expected);
  });

}
