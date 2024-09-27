## 213 robber

#### Solving
[solution ideas](./213-robber/solving.md)<br>
(mostly diagrams)

#### Ruby
[1. unoptimized](./213-robber/1-rb/)<br>
first solution, more clear than others<br>
41.94% time<br>
41.94% memory<br>
[submission](https://leetcode.com/submissions/detail/1397543026/)

[2. inlined](./213-robber/2-rb/)<br>
100% time<br>
90.32% memory<br>
[submission](https://leetcode.com/submissions/detail/1397630686/)

[3. without Array.max](./213-robber/3-rb/)<br>
check would it improve to avoid calling max on temporary arrays<br>
(it didn't)<br>
41.94% time<br>
90.32% memory<br>
[submision](https://leetcode.com/submissions/detail/1397606332/)

#### Elixir
[1. recursive and memoized](./213-robber/4-ex/)<br>
top-bottom, memoization to prevent exponential grow of recursive calls<br>
100% time<br>
100% memory<br>
[submission](https://leetcode.com/submissions/detail/1400044490/)

[2. memo in Agent](./213-robber/5-ex/)<br>
memo moved from parameter to Agent, better readibility at a cost of performance<br>
50% time<br>
50% memory<br>
[submission](https://leetcode.com/submissions/detail/1404058373/)
