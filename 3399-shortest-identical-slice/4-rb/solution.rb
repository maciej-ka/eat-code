# @param {String} s
# @param {Integer} num_ops
# @return {Integer}
def min_length(s, num_ops)
  counts = {}
  sol01 = -1
  sol10 = 0
  sub_start = 0

  (0 .. s.length).each do |i|
    if(s[i] != s[sub_start])
      counts[i - sub_start] = (counts[i - sub_start] || 0) + 1
      sub_start = i
    end

    # if even char is 0
    if (i % 2).to_s == s[i]
      sol10 += 1
    else 
      # always on last iteration, sol01 init -1 compensated this
      sol01 += 1
    end
  end

  return 1 if (sol01 <= num_ops || sol10 <= num_ops)

  lengths = counts.keys.sort.reverse

  lo = 2
  hi = s.length
  while (lo < hi)
    m = (hi + lo) >> 1
    if solution_possible?(counts, lengths, num_ops, m)
      hi = m
    else
      lo = m + 1
    end
  end

  lo
end

def solution_possible?(counts, lengths, num_ops, score)
  sum = 0
  lengths.each do |length|
    sum += (length / (score + 1)).floor * counts[length]
    return false if (sum > num_ops) 
  end
  true
end
