import collections


class HitCounter:

    def __init__(self):
        self.q = collections.deque()
        self.count = 0

    def hit(self, timestamp: int) -> None:
        self.count += 1
        if len(self.q) == 0:
            self.q.append((timestamp, 1))
            return

        lastTs, count = self.q[-1]
        if lastTs == timestamp:
            self.q[-1] = (timestamp, count + 1)
        else:
            self.q.append((timestamp, 1))

        while len(self.q) > 300:
            _, poppedCount = self.q.popleft()
            self.count -= poppedCount


    def getHits(self, timestamp: int) -> int:
        while len(self.q) > 0 and (timestamp - self.q[0][0]) >= 300:
            _, poppedCount = self.q.popleft()
            self.count -= poppedCount
        return self.count


if __name__ == '__main__':
    s = HitCounter()
    s.hit(1)
    s.hit(2)
    s.hit(3)
    print(s.getHits(4))
    s.hit(300)
    print(s.getHits(300))
    print(s.getHits(301))
