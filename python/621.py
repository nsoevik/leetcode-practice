from collections import defaultdict
import heapq
from typing import List


class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        counts = defaultdict(int)
        for t in tasks:
            counts[t] += 1

        h = []
        for c in counts.values():
            h.append(-c)

        heapq.heapify(h)
        intervals = 0
        while len(h) > 0:
            popped = []
            while len(h) > 0 and len(popped) < n + 1:
                intervals += 1
                lastCount = heapq.heappop(h)
                if lastCount + 1 < 0:
                    popped.append(lastCount + 1)
            if len(popped) < n:
                intervals += n-len(popped)
            while len(popped) > 0:
                heapq.heappush(h, popped.pop())
        return intervals

            
if __name__ == "__main__":
    s = Solution()
    print(s.leastInterval(["A", "A", "A", "B"], 4))
