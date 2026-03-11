import collections
from typing import List


class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        dq = collections.deque()
        left = 0
        right = 0
        dq.append(nums[right])
        while right < k - 1:
            right += 1
            new_num = nums[right]
            while len(dq) > 0 and new_num > dq[0]:
                dq.popleft()

            dq.append(new_num)

        ret = []
        ret.append(dq[0])
        while right < len(nums) - 1:
            if nums[left] == dq[0]:
                dq.popleft()
            left += 1

            right += 1
            while len(dq) > 0 and nums[right] > dq[0]:
                dq.popleft()
            while len(dq) > 0 and nums[right] > dq[-1]:
                dq.pop()
            dq.append(nums[right])
            print(dq)
            ret.append(dq[0])
        return ret



if __name__ == "__main__":
    nums = [1,3,1,2,0,5]
    k = 3
    s = Solution()
    print(s.maxSlidingWindow(nums, k))

