[1,2,3,4,5,6,7,8]

convert to hash for easy checking of existence
numbers = {1: true, 2: true}

build up, and memoize
memo = {}

check all starting with 1
result is length
remember the highest so far
(1,2: 5) (2,3: 4) (3,5: 3) (5,8: 2)
(1,3: 4) (3,4: 3) (4,7: 2)

whole chain length is determined by two first numbers
(nothing surprising can happen later on)

nothing will beat early max?
if early subsequence went all way, it's going to be best solution




[1,3,7,11,12,14,18]
(1, 3: 2) potentially no need to memoize any starting with "1", they won't be used later
(1, 7: 2)
(1, 11: 3) (11, 12: 2)
(1, 12: 2)
(1, 14: ...) skip because max so far 3 is longer then elements left here
(3, 7: 2)
(3, 11: 3) (11, 14: 2)
(3, 12: 2)
(3, 14: ...) skip
(7, 11: 3) (11, 18: 2) when last element is part of sequence, we know any other starting with same number will not be better
(11, 12: 2) if sum is greater then last element, we can skip whole rest of search

opti remember how many larger numbers number has
store numbers = {1: 7, 2: 6} meaning 6 numbers larger



Ideas
use some sort os sieve?
[1,3,7,11,12,14,18]

calculate diff to next element?
... but seqeunce can have skips

go from backwards?
[1,2,3,4,5,6,7,8]

[7,8]
