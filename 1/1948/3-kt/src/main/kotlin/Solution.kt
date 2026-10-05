import java.util.*

// https://leetcode.com/problems/delete-duplicate-folders-in-system/submissions/2163330684/

class Solution {
    val counts = HashMap<String, Int>()

    fun deleteDuplicateFolder(paths: List<List<String>>): List<List<String>> {
        val root = Node()

        for (path in paths) {
            var node = root
            for (folder in path)
                node = node.children.computeIfAbsent(folder) { Node() }
        }

        for ((_, node) in root.children)
            setContent(node)

        val result = ArrayList<List<String>>()
        for ((folder, node) in root.children)
            buildResult(node, listOf(folder), result)

        return result
    }

    fun setContent(node: Node) {
        if (node.children.isEmpty()) return

        val content = buildString {
            append("(")
            for ((folder, child) in node.children) {
                setContent(child)
                append(folder)
                append(child.content)
                append(",")
            }
            append(")")
        }

        val count = counts[content] ?: 0
        counts[content] = count + 1
        node.content = content
    }

    fun buildResult(node: Node, path: List<String>, result: MutableList<List<String>>) {
        if ((counts[node.content] ?: 1) > 1) return
        result += path
        for ((folder, child) in node.children)
            buildResult(child, path + folder, result)
    }
}

class Node {
    var content = "";
    val children = TreeMap<String, Node>();
}
