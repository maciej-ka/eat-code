# Leet code solving  
[347. Top K Frequent Elements](#347-top-k-frequent-elements)  
[2834. Find the Minimum Possible Sum of a Beautiful Array](#2834-find-the-minimum-possible-sum-of-a-beautiful-array)  
[Trick or Tree'ing](#trick-or-treeing)  
[213. House Robber II](#213-house-robber-ii)
[31. Next Permutation](#31-next-permutation)



[347. Top K Frequent Elements](https://leetcode.com/problems/top-k-frequent-elements/description/)
===========================================================

## Count using hashmap
then convert to array and sort

![img](./347-top-k-frequent/1-count-in-map.png)

### Javascript
[solution](./347-top-k-frequent/1-js/solution.js)  
[submission](https://leetcode.com/problems/top-k-frequent-elements/submissions/1532809469/)  
10ms (81.57%)  
52.17MB (95.30%)



[2834. Find the Minimum Possible Sum of a Beautiful Array](https://leetcode.com/problems/find-the-minimum-possible-sum-of-a-beautiful-array/description/)
===========================================================

![img](./2834-sum-beautiful/1-problem-examples.png)

![img](./2834-sum-beautiful/2-solution-ideas.png)

### Solvable by math?
idea that there is a mathematical solution  
and its just enough to do some maths  
and solve equation for area on discrete plane

![img](./2834-sum-beautiful/3-just-calculate.png)

### Check results

![img](./2834-sum-beautiful/4-check-on-examples.png)

### Ruby
[solution](./2834-sum-beautiful/1-rb/solution.rb)  
[submission](https://leetcode.com/submissions/detail/1397630686/)  
2ms (100%)  
211.4MB (100%)



[Trick or Tree'ing](https://dmoj.ca/problem/dwite12c1p4)
===========================================================

### Parse a tree
Parse string using stack and in result have an object tree.

![img](./dwite12c1p4/1-make-a-tree.png)

### Poblems with "candy length"  
"Candy length" is taking into account that kids don't have to end on root node.

![img](./dwite12c1p4/2-candy-length.png)

### Test cases

![img](./dwite12c1p4/3-test-case.png)

![img](./dwite12c1p4/4-test-case.png)

![img](./dwite12c1p4/5-test-case.png)

![img](./dwite12c1p4/6-test-case.png)

### Ruby
perhaps can be improved by not really creating nodes  
[solution](./dwite12c1p4/1-rb/solution.rb)  
[submission](https://dmoj.ca/submission/6695557)  
0.024s  
11.16 MB



[213. House Robber II](https://leetcode.com/problems/house-robber-ii/description/)
===========================================================

![img](./213-robber/1-problem.png)

### Attempt
Dynamic programming, growing from left to right.

![img](./213-robber/2-dynamic-from-left-to-right.png)

### Mistake attempt
Dynamic programming, growing in both directions.  
Always calculating as elements would be cycled.  
Which leads to mistakes in result when reusing previous results.

![img](./213-robber/3-dynamic-in-both-directions.png)

### More complex example

![img](./213-robber/3-longer-example.png)

### Solution

![img](./213-robber/5-dynamic-bottom-up-two-lists.png)

### Ruby
first solution, more clear than others  
[solution](./213-robber/1-rb/solution.rb)  
[submission](https://leetcode.com/submissions/detail/1397543026/)  
63ms (41.94%)  
221MB (41.94%)  

### Ruby
Inlined. Probably better performance due to smaller call stack  
[solution](./213-robber/2-rb/solution.rb)  
[submission](https://leetcode.com/submissions/detail/1397630686/)  
43ms (100%)  
211MB (90.32%)  

### Ruby
Without Array max. Check would it improve to avoid calling max on temporary arrays  
(it didn't)  
[solution](./213-robber/3-rb/solution.rb)  
[submision](https://leetcode.com/submissions/detail/1397606332/)  
63ms (41.94%)  
211MB (90.32%)  

### Top-bottom
Sketch of calcutating from top to bottom.  
With recursion and memoization.

![img](./213-robber/4-dynamic-top-bottom.png)

### Elixir
Recursive and memoized. Top-bottom, memoization to prevent exponential grow of recursive calls  
[solution](./213-robber/4-ex/lib/solution.ex)  
[submission](https://leetcode.com/submissions/detail/1400044490/)  
1ms (100%)  
72MB (100%)  

### Elixir
Memo in stateful process. Memo moved from parameter to Agent, better readibility at a cost of performance  
[solution](./213-robber/5-ex/lib/solution.ex)  
[submission](https://leetcode.com/submissions/detail/1404058373/)  
21ms (50%)  
74MB (50%)  



[31. Next Permutation](https://leetcode.com/problems/next-permutation/description/)
===========================================================

### C
[solution](./31-next-perm/1.c)  
[submission](https://leetcode.com/submissions/detail/1404058373/)  
3ms (2.36%)  
6.20MB (100%)  
