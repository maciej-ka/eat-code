class Solution {
  func minimumRecolors(_ blocks: String, _ k: Int) -> Int {
    let chars = Array(blocks)

    var count = 0
    for char in chars[0..<k] {
      if char == "W" { count += 1 }
    }

    var best = count
    for i in 0..<chars.count - k {
      if chars[i] == "W" { count -= 1 }
      if chars[i + k] == "W" { count += 1 }
      if count < best { best = count }
    }

    return best
  }
}
