import collections
from typing import List


class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:
        adj = collections.defaultdict(list)
        q = collections.deque()
        for a, b in edges:
            adj[a].append(b)
            adj[b].append(a)

        q.append((0, -1))
        visited = set()
        print(adj)
        while len(q) > 0:
            print(q)
            node, prev = q.popleft()
            visited.add(node)
            for next in adj[node]:
                if next == prev:
                    continue
                if next in visited:
                    print(f"node {node}, prev {prev}, next {next}")
                    return False
                q.append((next, node))

        return len(visited) == n

if __name__ == "__main__":
    s = Solution()
    print(s.validTree(5, [[0,1],[0,2],[0,3],[1,4]]))
    print(s.validTree(5, [[0,1],[1,2],[2,3],[1,3],[1,4]]))

