# https://leetcode.com/problems/largest-triangle-area/submissions/1784682869/

defmodule Solution do
  @spec largest_triangle_area(points :: [[integer]]) :: float
  def largest_triangle_area(points) do
    for {a, ia} <- Enum.with_index(points),
        {b, ib} <- Enum.with_index(points),
        {c, ic} <- Enum.with_index(points),
        ia < ib && ib < ic,
        reduce: 0 do
          acc -> max(acc, area(a, b, c))
        end
  end

  def area([ax, ay], [bx, by], [cx, cy]) do
    0.5 * abs(ay * (bx - cx) + by * (cx - ax) + cy * (ax - bx))
  end
end
