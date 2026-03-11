import random
from typing import List

class Solution:

    def __init__(self, w: List[int]):
        s = sum(w)
        # [1, 2, 1]
        # [0.25, 0.75, 1.00]
        for i in range(len(w)):
            w[i] = w[i] / s
            if i > 0:
                w[i] += w[i-1]
        self.w = w

    def _binary(self, left, right, target) -> int:
        if left == right:
            return left

        mid = (left + right) // 2
        if self.w[mid] < target:
            return self._binary(mid + 1, right, target)

        return self._binary(left, mid, target)


    def pickIndex(self) -> int:
        r = random.random()
        print(r)
        print(self._binary(0, len(self.w)-1, r))
        
        

if __name__ == "__main__":
    s = Solution([1, 2, 1])
    s.pickIndex()


