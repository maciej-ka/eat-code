import 'package:solution/solution.dart';
import 'package:test/test.dart';

void main() {

  test('test 1', () {
    var actual = Solution().solve([1,2,3]);
    var expected = 3;
    expect(actual, expected);
  });

}
