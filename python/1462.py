from collections import defaultdict
from typing import List


class Solution:
    def checkIfPrerequisite(self, numCourses: int, prerequisites: List[List[int]], queries: List[List[int]]) -> List[bool]:
        adj = defaultdict(list)
        course_to_prereqs = defaultdict(set)

        for pre, course in prerequisites:
            adj[pre].append(course)

        def dfs(node, visited):
            visited.add(node)
            for next in adj[node]:
                for visited_node in visited:
                    course_to_prereqs[next].add(visited_node)
                dfs(next, visited)

        for i in range(numCourses):
            dfs(i, set())

        ret = []
        for prereq, course in queries:
            if course in course_to_prereqs and prereq in course_to_prereqs[course]:
                ret.append(True)
                continue
            ret.append(False)

        return ret
                
if __name__ == "__main__":
    s = Solution()
    numCourses = 3
    prerequisites = [[0,1],[1,2],[2,3],[3,4]]
    queries = [[0,4],[4,0],[1,3],[3,0]]

    print(s.checkIfPrerequisite(numCourses, prerequisites, queries))

