import 'package:solution/solution.dart';
import 'package:test/test.dart';

void main() {

  test('test 1', () {
    var actual = Solution().maxSelectedElements([1, 2, 3]);
    var expected = 3;
    expect(actual, expected);
  });

  test('test 2', () {
    var actual = Solution().maxSelectedElements([1, 4, 7, 10]);
    var expected = 1;
    expect(actual, expected);
  });

  test('test 3', () {
    var actual = Solution().maxSelectedElements([1, 2, 3, 5, 6, 7]);
    var expected = 6;
    expect(actual, expected);
  });

  test('test 4', () {
    var actual = Solution().maxSelectedElements([1, 2, 4, 5, 7, 8, 9]);
    var expected = 5;
    expect(actual, expected);
  });

  test('test 5', () {
    var actual = Solution().maxSelectedElements([16,1,6,14,5,10,16,3,3,7,12,18,6,11,10,10,9,16]);
    var expected = 13;
    expect(actual, expected);
  });

  test('test 6', () {
    var actual = Solution().maxSelectedElements([13,1,10,11,15,12,7,17,20,16,19,4,12]);
    var expected = 8;
    expect(actual, expected);
  });

}
