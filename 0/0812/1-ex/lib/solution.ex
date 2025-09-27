# https://leetcode.com/problems/largest-triangle-area/submissions/1784643188/?envType=daily-question&envId=2025-09-27

defmodule Solution do
  @spec largest_triangle_area(points :: [[integer]]) :: float
  def largest_triangle_area(points) do
    [_ | bs] = points
    [_ | cs] = bs
    largest_triangle_area(points, bs, cs, 0)
  end

  def largest_triangle_area([a, _, _], [b, _], [c], best), do: max(best, area(a, b, c))
  def largest_triangle_area(as, bs, [c | cs], best) do
    largest_triangle_area(as, bs, cs, max(best, area(hd(as), hd(bs), c)))
  end

  def largest_triangle_area(as, [_, b2 | bs], [], best) do
    largest_triangle_area(as, [b2 | bs], bs, best)
  end
  def largest_triangle_area([_ | as], _, [], best) do
    [_ | bs] = as
    [_ | cs] = bs
    largest_triangle_area(as, bs, cs, best)
  end

  def area([ax, ay], [bx, by], [cx, cy]) do
    0.5 * abs(ay * (bx - cx) + by * (cx - ax) + cy * (ax - bx))
  end
end

# defmodule Solution do
#   @spec largest_triangle_area(points :: [[integer]]) :: float
#   def largest_triangle_area(points) do
#     [_ | bs] = points
#     [_ | cs] = bs
#     largest_triangle_area(points, bs, cs, 0)
#   end
#
#   def largest_triangle_area([a, _, _], [b, _], [c], best), do: max(best, area(a, b, c))
#   def largest_triangle_area(as, bs, [c | cs], best) do
#     largest_triangle_area(as, bs, cs, max(best, area(hd(as), hd(bs), c)))
#   end
#
#   def largest_triangle_area(as, [_, b2 | bs], [], best) do
#     largest_triangle_area(as, [b2 | bs], bs, best)
#   end
#   def largest_triangle_area([_ | as], _, [], best) do
#     [_ | bs] = as
#     [_ | cs] = bs
#     largest_triangle_area(as, bs, cs, best)
#   end
#
#   def area([ax, ay], [bx, by], [cx, cy]) do
#     0.5 * abs(ay * (bx - cx) + by * (cx - ax) + cy * (ax - bx))
#   end
# end
