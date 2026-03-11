from collections import defaultdict


class Solution:
    def criticalConnections(self, n: int, connections: List[List[int]]) -> List[List[int]]:
        adj = {}
        visited = set()

        for source, target in connections:
            adj[source].add(target)
            adj[target].add(source) 

        def dfs(node, prevNode):
            nonlocal visited
            if node in visited and prevNode:
                adj[prevNode].remove(node)
                adj[node].remove(prevNode)

            visited.add(node)

            for next in adj[node]:
                if next != prevNode:
                    dfs(next, node)

            
        dfs(0, None)
            


if __name__ == "__main__":


