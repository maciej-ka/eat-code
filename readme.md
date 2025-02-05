# Leet code solving  
[347. Top K Frequent Elements](#347-top-k-frequent-elements)  
[2834. Find the Minimum Possible Sum of a Beautiful Array](#2834-find-the-minimum-possible-sum-of-a-beautiful-array)

## [347. Top K Frequent Elements](https://leetcode.com/problems/top-k-frequent-elements/description/)

#### Count using hashmap
then convert to array and sort

![img](./347-top-k-frequent/1-count-in-map.png)

#### 1. Javascript
[solution](./347-top-k-frequent/1-js/solution.js)  
[submission](https://leetcode.com/problems/top-k-frequent-elements/submissions/1532809469/)  
10ms (81.57%)  
52.17MB (95.30%)



## [2834. Find the Minimum Possible Sum of a Beautiful Array](https://leetcode.com/problems/find-the-minimum-possible-sum-of-a-beautiful-array/description/)

![img](./2834-sum-beautiful/1-problem-examples.png)

![img](./2834-sum-beautiful/2-solution-ideas.png)

#### Solvable by math?
idea that there is a mathematical solution  
and its just enough to do some maths  
and solve equation for area on discrete plane

![img](./2834-sum-beautiful/3-just-calculate.png)

#### Check results

![img](./2834-sum-beautiful/4-check-on-examples.png)

#### 1. Ruby
[solution](./2834-sum-beautiful/1-rb/solution.rb)  
[submission](https://leetcode.com/submissions/detail/1397630686/)  
2ms (100%)  
211.4MB (100%)



## Trick or Tree'ing
https://dmoj.ca/problem/dwite12c1p4

#### Parse a tree
Parse string using stack and in result have an object tree.

![img](./dwite12c1p4/1-make-a-tree.png)

# Poblems with "candy length"  
"Candy length" is taking into account that kids don't have to end on root node.

![img](./dwite12c1p4/2-candy-length.png)

# Test cases

![img](./dwite12c1p4/3-test-case.png)

![img](./dwite12c1p4/4-test-case.png)

![img](./dwite12c1p4/5-test-case.png)

![img](./dwite12c1p4/6-test-case.png)

#### 1. Ruby
perhaps can be improved by not really creating nodes  
[solution](./dwite12c1p4/1-rb/solution.rb)  
[submission](https://dmoj.ca/submission/6695557)  
0.024s  
11.16 MB











## 213 robber
https://leetcode.com/problems/house-robber-ii/description/

#### Solving
[solution ideas](./213-robber/solving.md)<br>

#### Ruby
[1. unoptimized](./213-robber/1-rb/solution.rb)<br>  
first solution, more clear than others<br>  
<sup>  
[submission](https://leetcode.com/submissions/detail/1397543026/)<br>  
63ms (41.94%)<br>  
221MB (41.94%)  
</sup>

[2. inlined](./213-robber/2-rb/solution.rb)<br>  
probably better performance due to smaller call stack<br>  
<sup>  
[submission](https://leetcode.com/submissions/detail/1397630686/)<br>  
43ms (100%)<br>  
211MB (90.32%)  
</sup>

[3. without Array.max](./213-robber/3-rb/solution.rb)<br>  
check would it improve to avoid calling max on temporary arrays<br>  
(it didn't)<br>  
<sup>  
[submision](https://leetcode.com/submissions/detail/1397606332/)<br>  
63ms (41.94%)<br>  
211MB (90.32%)  
</sup>

#### Elixir
[1. recursive and memoized](./213-robber/4-ex/lib/solution.ex)<br>  
top-bottom, memoization to prevent exponential grow of recursive calls<br>  
<sup>  
[submission](https://leetcode.com/submissions/detail/1400044490/)<br>  
1ms (100%)<br>  
72MB (100%)  
</sup>

[2. memo in stateful process](./213-robber/5-ex/lib/solution.ex)<br>  
memo moved from parameter to Agent, better readibility at a cost of performance<br>  
<sup>  
[submission](https://leetcode.com/submissions/detail/1404058373/)<br>  
21ms (50%)<br>  
74MB (50%)  
</sup>

