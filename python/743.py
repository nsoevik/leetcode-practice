from collections import defaultdict
import heapq
from typing import List

class Solution:
    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
        adj = defaultdict(list)
        for source, target, time in times:
            adj[source].append((target, time))

        minHeap = []
        heapq.heappush(minHeap, (0, k))
        visited = {}

        while minHeap:
            time, node = heapq.heappop(minHeap)
            if node in visited:
                continue

            visited[node] = time
            for nextTarget, timeToTarget in adj[node]:
                heapq.heappush(minHeap, (timeToTarget + time, nextTarget))

        if len(visited) < n:
            return -1
        
        return max(visited.values())
        
if __name__ == "__main__":
    times = [[2,1,1],[2,3,2],[3,4,1]]
    n = 4
    k = 2
    s = Solution()

    print(s.networkDelayTime(times, n, k))
