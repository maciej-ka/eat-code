// https://leetcode.com/problems/delete-duplicate-folders-in-system/submissions/2163168360/
import java.util.*;

class Solution {

    private Map<String, Integer> counts = new HashMap<>();

    public List<List<String>> deleteDuplicateFolder(List<List<String>> paths) {
        var root = new Node();

        for (var path: paths) {
            var node = root;
            for (var name: path)
                node = node.children.computeIfAbsent(name, k -> new Node());
        }

        visitContentHashing(root);

        var result = new ArrayList<List<String>>();
        for (var entry: root.children.entrySet()) {
            var path = new ArrayList<String>();
            path.add(entry.getKey());
            visitResponseBuilding(entry.getValue(), result, path);
        }

        return result;
    }

    private void visitContentHashing(Node node) {
        if (node.children.isEmpty()) {
            node.content = "";
            return;
        }

        var builder = new StringBuilder();
        builder.append("(");
        for (var entry: node.children.entrySet()) {
            Node child = entry.getValue();
            visitContentHashing(child);
            builder.append(entry.getKey());
            builder.append(",");
            builder.append(child.content);
        }
        builder.append(")");

        var content = builder.toString();
        int count = counts.getOrDefault(content, 0);
        counts.put(content, count + 1);
        node.content = content;
    }

    private void visitResponseBuilding(Node node, List<List<String>> result, List<String> path) {
        if (node.content == "" || counts.get(node.content) < 2) {
            result.add(path);
            for (var entry: node.children.entrySet()) {
                var newPath = new ArrayList<String>(path);
                newPath.add(entry.getKey());
                visitResponseBuilding(entry.getValue(), result, newPath);
            }
        }
    }
}

class Node {
    public String content;
    public Map<String, Node> children = new TreeMap<>();
}
