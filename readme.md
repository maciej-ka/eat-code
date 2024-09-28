## 213 robber

#### Solving
[solution ideas](./213-robber/solving.md)<br>
(mostly diagrams)

#### Ruby
[1. unoptimized](./213-robber/1-rb/solution.rb)<br>
first solution, more clear than others<br>
63ms (41.94%) time<br>
221MB (41.94%) memory<br>
[submission](https://leetcode.com/submissions/detail/1397543026/)

[2. inlined](./213-robber/2-rb/solution.rb)<br>
43ms (100%) time<br>
211MB (90.32%) memory<br>
[submission](https://leetcode.com/submissions/detail/1397630686/)

[3. without Array.max](./213-robber/3-rb/solution.rb)<br>
check would it improve to avoid calling max on temporary arrays<br>
(it didn't)<br>
63ms (41.94%) time<br>
211MB (90.32%) memory<br>
[submision](https://leetcode.com/submissions/detail/1397606332/)

#### Elixir
[1. recursive and memoized](./213-robber/4-ex/lib/solution.ex)<br>
top-bottom, memoization to prevent exponential grow of recursive calls<br>
1ms (100%) time<br>
72MB (100%) memory<br>
[submission](https://leetcode.com/submissions/detail/1400044490/)

[2. memo in Agent](./213-robber/5-ex/lib/solution.ex)<br>
memo moved from parameter to Agent, better readibility at a cost of performance<br>
21ms (50%) time<br>
74MB (50%) memory<br>
[submission](https://leetcode.com/submissions/detail/1404058373/)
