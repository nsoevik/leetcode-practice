from collections import defaultdict


class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        adj = defaultdict(list)
        for course, prereq in prerequisites:
            adj[course].append(prereq)


        checked_courses = set()
        ret = []
        def has_cycle(course, visited):
            if course in checked_courses:
                return False

            if course in visited:
                return True

            visited.add(course)
            for next in adj[course]:
                if has_cycle(next, visited):
                    return True

            checked_courses.add(course)
            ret.append(course)
            return False
            

        for i in range(numCourses):
            if has_cycle(i, set()):
                return []
        return ret


