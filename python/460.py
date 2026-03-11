from collections import defaultdict


class LFUCache:

    # dictionary of nodes
    # min heap of nodes
    # add 1, add 2, add 3
    # (1, 1) (1, 2) 

    def __init__(self, capacity: int):
        # key -> (freq, value)
        self.keyMap = {}
        # freq -> key
        self.freqMap = defaultdict(set)
        self.minFreq = 0

    def get(self, key: int) -> int:
        if key not in self.keyMap:
            return -1
        freq, value = self.keyMap[key]
        self.keyMap[key] = (freq + 1, value)
        self.freqMap[freq].remove(key)
        if len(self.freqMap[freq]) == 0:
            self.minFreq = freq + 1
        self.freqMap[freq + 1].add(key)

    def put(self, key: int, value: int) -> None:
        

