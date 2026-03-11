class MyCircularQueue:

    def __init__(self, k: int):
        self.arr = [-1 for _ in range(k)]
        self.k = k
        self.count = 0
        self.head = 0

    def enQueue(self, value: int) -> bool:
        if self.count == self.k:
            return False

        write_index = (self.head + self.count) % self.k
        self.arr[write_index] = value
        self.count += 1
        return True

    def deQueue(self) -> bool:
        if self.count == 0:
            return False

        self.head = (self.head + 1) % self.k
        self.count -= 1
        return True

    def Front(self) -> int:
        if self.count == 0:
            return -1
        return self.arr[self.head]

    def Rear(self) -> int:
        if self.count == 0:
            return -1
        return self.arr[(self.head + self.count - 1) % self.k]
        
    def isEmpty(self) -> bool:
        return self.count == 0

    def isFull(self) -> bool:
        return self.count == self.k
        
if __name__ == "__main__":
    obj = MyCircularQueue(k)
    param_1 = obj.enQueue(value)
    param_2 = obj.deQueue()
    param_3 = obj.Front()
    param_4 = obj.Rear()
    param_5 = obj.isEmpty()
    param_6 = obj.isFull()
