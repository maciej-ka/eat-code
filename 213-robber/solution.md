[problem list](../readme.md)

# Problem
https://leetcode.com/problems/house-robber-ii/

![img](./1-problem.png)

# Attempt
Dynamic programming, growing from left to right.

![img](./2-dynamic-from-left-to-right.png)

# Mistake attempt
Dynamic programming, growing in both directions.
Always calculating as elements would be cycled.
Which leads to mistakes in result when reusing previous results.

![img](./3-dynamic-in-both-directions.png)

# More complex example

![img](./3-longer-example.png)

# Sketch of attempt
Sketch of calcutating from top to bottom.
With recursion and memoization.

![img](./4-dynamic-top-bottom.png)

# Solution

![img](./5-dynamic-bottom-up-two-lists.png)
