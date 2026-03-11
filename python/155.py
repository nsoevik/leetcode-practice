class MinStack:

    def __init__(self):
        self.s = []
        

    def push(self, val: int) -> None:
        if len(self.s) == 0:
            self.s.append((val, val))
            return

        min_at_push = min(self.s[-1][1], val)
        self.s.append((val, min_at_push))

    def pop(self) -> None:
        if len(self.s) == 0:
            return
        self.s.pop()
        

    def top(self) -> int:
        if len(self.s) == 0:
            return -1
        return self.s[-1][0]
        

    def getMin(self) -> int:
        if len(self.s) == 0:
            return -1
        return self.s[-1][1]
        
