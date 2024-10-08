[https://leetcode.com](https://leetcode.com/problemset/)

## 213 robber

#### Solving
[solution ideas](./213-robber/solving.md)<br>
(mostly diagrams)

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

