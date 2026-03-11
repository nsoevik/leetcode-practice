import collections
from typing import List


class Solution:
    def countComponents(self, n: int, edges: List[List[int]]) -> int:
        adj = collections.defaultdict(list)
        for a, b in edges:
            adj[a].append(b)
            adj[b].append(a)

        visited = set()
        def dfs(node):
            nonlocal visited
            visited.add(node)
            for next in adj[node]:
                if next not in visited:
                    dfs(next)

        count = 0
        for i in range(n):
            if i not in visited:
                count += 1
                dfs(i)


        return count

if __name__ == "__main__":
    s = Solution()
    print(s.countComponents(5, [[0,1],[1,2],[3,4]]))
    print(s.countComponents(5, [[0,1],[1,2],[2,3],[3,4]]))

