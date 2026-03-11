import collections


class TimeMap:

    def __init__(self):
        self.series = collections.defaultdict(list)


    def set(self, key: str, value: str, timestamp: int) -> None:
        self.series[key].append((timestamp,value))
        

    def _binary_search(self, left, right, series, timestamp):
        if left == right:
            ts, val = series[left]
            if ts <= timestamp:
                return val
            return ""

        mid = (left + right) // 2
        ts, val = series[mid]
        if ts <= timestamp:
            next_ts, next_val = series[mid+1]
            if next_ts > timestamp:
                return val
            return self._binary_search(mid + 1, right, series, timestamp)
        return self._binary_search(left, mid, series, timestamp)


    def get(self, key: str, timestamp: int) -> str:
        if key not in self.series:
            return ""

        return self._binary_search(0, len(self.series[key]) - 1, series, timestamp)
        
if __name__ == "__main__":
    s = TimeMap()

