# https://leetcode.com/problems/largest-triangle-area/submissions/1784661429/

defmodule Solution do
  @spec largest_triangle_area(points :: [[integer]]) :: float
  def largest_triangle_area([_ | [_ | cs] = bs] = points), do: solution(points, bs, cs, 0)

  def solution([a, _, _], [b, _], [c], best), do: max(best, area(a, b, c))
  def solution(as, bs, [c | cs], best), do: solution(as, bs, cs, max(best, area(hd(as), hd(bs), c)))
  def solution(as, [_ | [_ | cs] = bs], [], best), do: solution(as, bs, cs, best)
  def solution([_ | [_ | [_ | cs] = bs] = as], _, [], best), do: solution(as, bs, cs, best)

  def area([ax, ay], [bx, by], [cx, cy]) do
    0.5 * abs(ay * (bx - cx) + by * (cx - ax) + cy * (ax - bx))
  end
end
