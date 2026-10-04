// https://leetcode.com/problems/delete-duplicate-folders-in-system/submissions/2162539810/
import java.util.*;

class Solution {
    static int MOD = 972663749;
    static int A = 911382323;

    private Map<Integer, Integer> counts = new HashMap<Integer, Integer>();

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
        int acc = 0;
        for (var entry: node.children.entrySet()) {
            Node child = entry.getValue();
            visitContentHashing(child);
            acc = acc + hash(child.contentHash + hash(entry.getKey()));
            acc %= MOD;
        }

        node.contentHash = acc;
        int count = counts.getOrDefault(acc, 0);
        counts.put(acc, count + 1);
    }

    private void visitResponseBuilding(Node node, List<List<String>> result, List<String> path) {
        if (node.contentHash == 0 || counts.get(node.contentHash) < 2) {
            result.add(path);
            for (var entry: node.children.entrySet()) {
                var newPath = new ArrayList<String>(path);
                newPath.add(entry.getKey());
                visitResponseBuilding(entry.getValue(), result, newPath);
            }
        }
    }

    private int hash(int val) {
        return hash("" + val);
    }

    private int hash(String s) {
        int res = 0;
        int a = 1;
        for (var b: s.getBytes()) {
            res = (res + mul(b, a)) % MOD;
            a = mul(a, A);
        }
        return res;
    }

    public int pow(int a, int b) {
        int res = 1;
        while (b > 0) {
            if ((b & 1) == 1) res = mul(res, a);
            b >>= 1;
            a = mul(a, a);
        }
        return res;
    }

    private int mul(int a, int b) {
        int res = 0;
        while (b > 0)  {
            if ((b & 1) == 1) res = (res + a) % MOD;
            b >>= 1;
            a = (a << 1) % MOD;
        }
        return res;
    }
}

class Node {
    public int contentHash;
    public Map<String, Node> children = new HashMap<>();
}
