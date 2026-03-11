from collections import defaultdict
import heapq

# two heaps
# add score (nlogn)
# top (klogn)


# one heap, lazy updates
# addScore (1)
# go over all elements (n)

class Leaderboard:

    def __init__(self):
        self.player_scores = defaultdict(int)
        

    def addScore(self, playerId: int, score: int) -> None:
        self.player_scores[playerId] += score


    def top(self, K: int) -> int:
        temp_heap = []
        for player in self.player_scores:
            heapq.heappush(temp_heap, self.player_scores[player])
            print(temp_heap)
            while len(temp_heap) > K:
                heapq.heappop(temp_heap)

        s = 0
        while len(temp_heap) > 0:
            s += heapq.heappop(temp_heap)
        return s

    def reset(self, playerId: int) -> None:
        self.player_scores[playerId] = 0
        

if __name__ == "__main__":
    l = Leaderboard()
    l.addScore(1, 2)
    l.addScore(1, 3)
    l.addScore(3, 3)
    l.addScore(5, 1)
    print(l.top(2))
