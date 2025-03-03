nums [7, 12, 34, 12, 67, 5]
pivot = 12

use swaps
have two pointers
count number of elements smaller or equal

### linked list
linked list could be proper
but in the end we have to convert to array
and conversion will be costly

### permutation
make a permutation table

nums          [7, 12, 34, 12, 67, 5], 12
result        [7, 5, 12, 12, 34, 67]
permutation   [0, 2, 4, 3, 5, 1]

#### how to apply?
nums          [7, 12, 34, 12, 67, 5], 12
permutation   [0, 2, 4, 3, 5, 1]

i = 0
[7, ...]
[0, ...]
i === p[i]: do nothing

i = 1
p[i] = 2
send value 1 to index 2

but how to not loose data?
swap i with p[i]
swap permutations same way

nums          [7, 34, 12, 12, 67, 5]
permutation   [0, 4, 2, 3, 5, 1]
*hmmm....*

#### reverse permutation
i = 1
find p[i] that is equal 1

reverse permutation
(also called preimage map)

nums          [7, 12, 34, 12, 67, 5], 12
result        [7, 5, 12, 12, 34, 67]
reverse perm  [0, 5, 1, 3, 2, 4]

i = 0
rp[i] = 0
skip

i = 1
rp[i] = 5
swap to have correct element on i === 1

... or just start separate empty array
and built it from reverse permutation

