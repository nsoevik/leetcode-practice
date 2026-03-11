import random
from typing import List

class Solution:
    def checkSubarraySum(self, nums: List[int], k: int) -> bool:
        # sum up to 6
        # [5, 2, 0, 4, 1]

        nums = [num % k for num in nums]

        

if __name__ == "__main__":
    s = Solution([1, 2, 1])
    s.pickIndex()


