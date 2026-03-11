class RandomizedSet:

    def __init__(self):
        self.arr = []
        self.valToIndex = {}

    def insert(self, val: int) -> bool:
        if val in self.valToIndex:
            return False
        self.arr.append(val)
        self.valToIndex.add((len(arr) - 1, val))

    def remove(self, val: int) -> bool:
        if val not in self.valToIndex:
            return False
        index = self.valToIndex[val]
        del self.valToIndex[val]
        
        lastVal = self.arr.pop()
        if len(self.arr) == 0:
            return

        self.arr[index] = lastVal
        self.valToIndex[lastVal] = index

    def getRandom(self) -> int:
        return self.arr[0]
        
